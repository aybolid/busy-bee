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
        rss_consumer::run_rss_articles_consumer,
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
    RssArtcilesConsumer(#[from] crate::workers::rss_consumer::RssArticlesConsumerError),
    #[error(transparent)]
    ArtcileProcessor(#[from] crate::workers::article_processor::ArticleProcessorError),
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

#[tracing::instrument(level = "trace", err)]
pub async fn run() -> Result<(), RunError> {
    let config = load_config()?;

    let cancel_token = CancellationToken::new();
    tokio::spawn(shutdown_signal_listener(cancel_token.clone()));

    let amqp_connection = amqp_connect(&config).await?;
    {
        let channel = amqp_connection.create_channel().await?;
        declare_durable_queue(&channel, config.rss_articles_queue().clone()).await?;
        declare_durable_queue(&channel, config.article_processor_queue().clone()).await?;
        channel.close(200, "setup completed".into()).await?;
    }

    let db_pool = database_connect(config.database_url()).await?;
    database_migrate(&db_pool).await?;

    let ai_client = ai::Client::try_new(&config).await?;

    let (publisher_tx, publisher_rx) = create_publisher_mpsc_channel();

    let state = SharedAppState::new(AppState::new(
        config,
        db_pool,
        amqp_connection,
        ai_client,
        publisher_tx,
        cancel_token,
    ));

    let publisher = run_publisher(state.clone(), publisher_rx);
    let rss_consumer = run_rss_articles_consumer(state.clone());
    let article_processor = run_article_processor(state.clone());
    let api_server = run_api_server(state.clone());

    let mut tasks = JoinSet::new();

    tasks.spawn(async move { publisher.await.map_err(RunError::from) });
    tasks.spawn(async move { api_server.await.map_err(RunError::from) });
    tasks.spawn(async move { rss_consumer.await.map_err(RunError::from) });
    tasks.spawn(async move { article_processor.await.map_err(RunError::from) });

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    _ = amqp_close(state.amqp_connection())
        .await
        .inspect_err(|error| tracing::error!(%error));

    database_close(state.db_pool()).await;

    tracing::info!("bye!");
    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn shutdown_signal_listener(cancel_token: CancellationToken) {
    let ctrl_c = async {
        tracing::info!("listening to ctrl + c notification");
        tokio::signal::ctrl_c()
            .await
            .inspect_err(|error| tracing::error!(%error))
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;

        tracing::info!("listening to SIGTERM");
        tokio::signal::unix::signal(SignalKind::terminate())
            .inspect_err(|error| tracing::error!(%error))
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
