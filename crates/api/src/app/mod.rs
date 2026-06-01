use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{
    ai,
    app::{
        config::load_config,
        events::AppEventsBroadcaster,
        state::{AppState, SharedAppState},
    },
    infra::db::{database_close, database_connect, database_migrate},
    workers::{
        api::run_api_server,
        article_processor::{create_article_processing_channel, run_article_processor},
        rss_reader::run_rss_reader,
    },
};

pub mod config;
pub mod events;
pub mod state;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
    #[error(transparent)]
    ArtcileProcessor(#[from] crate::workers::article_processor::ArticleProcessorError),
    #[error(transparent)]
    RssReader(#[from] crate::workers::rss_reader::RssReaderError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Genai(#[from] genai::Error),
}

#[tracing::instrument(level = "trace", err(Debug))]
pub async fn run() -> Result<(), RunError> {
    let config = load_config();

    let cancel_token = CancellationToken::new();
    tokio::spawn(shutdown_signal_listener(cancel_token.clone()));

    let db_pool = database_connect(&config.database_url).await?;
    database_migrate(&db_pool).await?;

    let ai = ai::create_ai_client(&config.ai).await?;

    let (article_processing_tx, article_processing_rx) = create_article_processing_channel();
    let app_events_broadcaster = AppEventsBroadcaster::new();

    let state = SharedAppState::new(AppState {
        config,
        db_pool,
        ai,
        article_processing_tx,
        app_events_broadcaster,
        cancel_token,
    });

    let mut tasks = JoinSet::new();

    tasks.spawn(worker(run_api_server(state.clone())));

    tasks.spawn(worker(run_rss_reader(state.clone())));
    tasks.spawn(worker(run_article_processor(
        state.clone(),
        article_processing_rx,
    )));

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    database_close(&state.db_pool).await;

    tracing::info!("bye!");
    Ok(())
}

async fn worker(
    future: impl Future<Output = Result<(), impl Into<RunError>>>,
) -> Result<(), RunError> {
    future.await.map_err(Into::into)
}

#[tracing::instrument(level = "trace", skip_all)]
async fn shutdown_signal_listener(cancel_token: CancellationToken) {
    let ctrl_c = async {
        tracing::info!("listening to ctrl + c notification");
        tokio::signal::ctrl_c()
            .await
            .inspect_err(|error| tracing::error!(?error))
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;

        tracing::info!("listening to SIGTERM");
        tokio::signal::unix::signal(SignalKind::terminate())
            .inspect_err(|error| tracing::error!(?error))
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => tracing::trace!("ctrl + c"),
        () = terminate => tracing::trace!("SIGTERM")
    }

    cancel_token.cancel();
}
