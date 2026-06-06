use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{
    ai::create_ai_client,
    app::{
        config::load_config,
        events::AppEventsBroadcaster,
        state::{AppState, SharedAppState},
    },
    infra::db::{database_close, database_connect, database_migrate},
    workers::{
        api::run_api_server,
        article_processing::{
            ProcessingRequestReceiver, create_processing_requests_channel, run_article_processing,
        },
        rss_reader::run_rss_reader,
    },
};

/// A dynamic error type capable of capturing any standard, thread-safe Rust error.
/// Used to standardize error handling across different background workers.
pub type WorkerDynError = Box<dyn std::error::Error + 'static + Send + Sync>;

/// A standard result type returned by background workers.
pub type WorkerResult = Result<(), WorkerDynError>;

/// Represents errors that can occur during the application's entire lifecycle.
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    /// Failed to prepare the initial application state.
    #[error(transparent)]
    Prepare(#[from] PrepareStateError),
    /// A tokio background task panicked or was cancelled abruptly.
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
    /// A background worker encountered a fatal operational error.
    #[error(transparent)]
    Worker(#[from] WorkerDynError),
}

/// The main entry point for the application's runtime.
pub async fn run() -> Result<(), RunError> {
    let (state, article_processing_rx) = prepare_state().await?;

    // Spawn a dedicated task to listen for OS interrupt signals.
    tokio::spawn(shutdown_signal_listener(state.cancel_token.clone()));

    // A JoinSet is used to manage the lifecycles of concurrent workers.
    let mut workers = JoinSet::<WorkerResult>::new();

    workers.spawn(worker(run_api_server(state.clone())));
    workers.spawn(worker(run_rss_reader(state.clone())));
    workers.spawn(worker(run_article_processing(
        state.clone(),
        article_processing_rx,
    )));

    // Block until the first worker completes or encounters an error.
    // If the cancel token is triggered, well-behaved workers will exit gracefully.
    while let Some(worker_result) = workers.join_next().await {
        worker_result??;
    }

    cleanup(state).await;
    Ok(())
}

/// Gracefully closes external connections and cleans up application resources.
async fn cleanup(state: SharedAppState) {
    database_close(&state.db_pool).await;
}

/// A helper function to wrap worker futures, unifying their distinct error types
/// into a standard [`WorkerDynError`].
async fn worker(
    future: impl Future<Output = Result<(), impl Into<WorkerDynError>>>,
) -> WorkerResult {
    future.await.map_err(Into::into)
}

/// Represents errors that can occur during the [`prepare_state`] initialization phase.
#[derive(Debug, thiserror::Error)]
pub enum PrepareStateError {
    /// Failed to connect to or interact with the database.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    /// Failed to apply database migrations.
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    /// Failed to initialize the generative AI client.
    #[error(transparent)]
    Genai(#[from] genai::Error),
}

/// Initializes the application's configuration, infrastructure, and shared state.
async fn prepare_state() -> Result<(SharedAppState, ProcessingRequestReceiver), PrepareStateError> {
    let config = load_config();

    let cancel_token = CancellationToken::new();

    let db_pool = database_connect(&config.database_url).await?;
    database_migrate(&db_pool).await?;

    let ai = create_ai_client(&config.ai).await?;

    let (article_processing_tx, article_processing_rx) = create_processing_requests_channel();
    let app_events_broadcaster = AppEventsBroadcaster::new();

    Ok((
        SharedAppState::from(AppState {
            config,
            db_pool,
            ai,
            article_processing_tx,
            app_events_broadcaster,
            cancel_token,
        }),
        article_processing_rx,
    ))
}

/// Blocks until an OS interrupt signal (Ctrl+C or SIGTERM) is received,
/// then triggers the provided `CancellationToken` to initiate a graceful shutdown.
async fn shutdown_signal_listener(cancel_token: CancellationToken) {
    let ctrl_c = async {
        tracing::info!("listening to ctrl + c notification");
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;

        tracing::info!("listening to SIGTERM");
        tokio::signal::unix::signal(SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    // Windows and other non-Unix platforms do not support the POSIX SIGTERM signal.
    // Standard graceful shutdown on these platforms is typically handled via Ctrl+C
    // (CTRL_C_EVENT or CTRL_BREAK_EVENT), which is already covered by the listener above.
    // We use `std::future::pending()` here to create a future that never completes.
    // This allows the `tokio::select!` block below to compile and execute seamlessly
    // on all operating systems without requiring complex `#[cfg]` macros inside the block.
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => tracing::trace!("ctrl + c received"),
        () = terminate => tracing::trace!("SIGTERM received")
    }

    // Broadcast the cancellation signal to all listening workers.
    cancel_token.cancel();
}
