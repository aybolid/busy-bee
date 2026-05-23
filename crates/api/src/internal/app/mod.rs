use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::internal::{
    ai,
    api::{run_api_server, state::ApiState},
    app::config::load_config,
    infra::{
        amqp::{amqp_close, amqp_connect, declare_durable_queue},
        db::{database_close, database_connect, database_migrate},
    },
    workers::{
        article_processor::{ArticleProcessorContext, run_article_processor},
        publisher::{Queues, create_publisher_mpsc_channel, run_publisher},
        rss_consumer::{RssArticlesConsumerContext, run_rss_articles_consumer},
    },
};

pub mod config;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum RunError {
    #[error(transparent)]
    LoadConfigError(#[from] config::LoadConfigError),
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
    #[error(transparent)]
    TaskError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    RssArtcilesConsumerError(
        #[from] crate::internal::workers::rss_consumer::RssArticlesConsumerError,
    ),
    #[error(transparent)]
    ArtcileProcessorError(
        #[from] crate::internal::workers::article_processor::ArticleProcessorError,
    ),
    #[error(transparent)]
    PublisherError(#[from] crate::internal::workers::publisher::PublisherError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    AiError(#[from] ai::ClientInitError),
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
    let ai_client = ai::SharedClient::new(ai_client);

    let (tx, rx) = create_publisher_mpsc_channel();
    let publisher = run_publisher(
        rx,
        amqp_connection.create_channel().await?,
        Queues::new(config.article_processor_queue().clone()),
    );

    let rss_consumer = run_rss_articles_consumer(RssArticlesConsumerContext::new(
        db_pool.clone(),
        amqp_connection.create_channel().await?,
        config.rss_articles_queue().clone(),
        cancel_token.clone(),
    ));

    let article_processor = run_article_processor(ArticleProcessorContext::new(
        ai_client,
        db_pool.clone(),
        amqp_connection.create_channel().await?,
        config.article_processor_queue().clone(),
        cancel_token.clone(),
    ));

    let api_server = run_api_server(ApiState::new(config, db_pool.clone(), tx), cancel_token);

    let mut tasks = JoinSet::new();

    tasks.spawn(async move { publisher.await.map_err(RunError::from) });
    tracing::info!("publisher task spawned");
    tasks.spawn(async move { api_server.await.map_err(RunError::from) });
    tracing::info!("api server task spawned");
    tasks.spawn(async move { rss_consumer.await.map_err(RunError::from) });
    tracing::info!("rss consumer task spawned");
    tasks.spawn(async move { article_processor.await.map_err(RunError::from) });
    tracing::info!("article processor task spawned");

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    _ = amqp_close(amqp_connection)
        .await
        .inspect_err(|error| tracing::error!(%error));
    database_close(db_pool).await;

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
