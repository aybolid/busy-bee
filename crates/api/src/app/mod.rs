use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{
    ai,
    app::{
        config::load_config,
        state::{AppState, SharedAppState},
    },
    infra::{
        amqp::{amqp_close, amqp_connect, declare_durable_queue},
        db::{database_close, database_connect, database_migrate},
    },
    workers::{
        api::run_api_server,
        article_processor::run_article_processor,
        publisher::{create_publisher_mpsc_channel, run_publisher},
        rss_reader::run_rss_reader,
    },
};

pub mod config;
pub mod state;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error(transparent)]
    LoadConfig(#[from] config::LoadConfigError),
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
    #[error(transparent)]
    ArtcileProcessor(#[from] crate::workers::article_processor::ArticleProcessorError),
    #[error(transparent)]
    RssReader(#[from] crate::workers::rss_reader::RssReaderError),
    #[error(transparent)]
    Publisher(#[from] crate::workers::publisher::PublisherError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Ai(#[from] ai::ClientInitError),
}

#[tracing::instrument(level = "trace", err(Debug))]
pub async fn run() -> Result<(), RunError> {
    let config = load_config()?;

    let cancel_token = CancellationToken::new();
    tokio::spawn(shutdown_signal_listener(cancel_token.clone()));

    let amqp_connection = amqp_connect(&config).await?;
    {
        let channel = amqp_connection.create_channel().await?;
        declare_durable_queue(&channel, config.rss_articles_queue.as_str().into()).await?;
        declare_durable_queue(&channel, config.article_processor_queue.as_str().into()).await?;
        channel.close(200, "setup completed".into()).await?;
    }

    let db_pool = database_connect(config.database_url.as_str()).await?;
    database_migrate(&db_pool).await?;

    let ai_client = ai::Client::try_new(&config).await?;

    let (publisher_tx, publisher_rx) = create_publisher_mpsc_channel();

    let state = SharedAppState::new(AppState {
        config,
        db_pool,
        amqp_connection,
        ai_client,
        publisher_tx,
        cancel_token,
    });

    let mut tasks = JoinSet::new();

    tasks.spawn(worker(run_publisher(state.clone(), publisher_rx)));
    tasks.spawn(worker(run_api_server(state.clone())));
    tasks.spawn(worker(run_article_processor(state.clone())));
    tasks.spawn(worker(run_rss_reader(state.clone())));

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    _ = amqp_close(&state.amqp_connection)
        .await
        .inspect_err(|error| tracing::error!(?error));

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
