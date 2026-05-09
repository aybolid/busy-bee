use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::internal::{
    api::{ApiContext, run_api_server},
    app::config::load_config,
    infra::amqp::{amqp_connect, amqp_success_close},
    rss_consumer::{RssArticlesConsumerContext, run_rss_articles_consumer},
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
    RssArtcilesConsumerError(#[from] crate::internal::rss_consumer::RssArticlesConsumerError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[tracing::instrument(level = "trace", err)]
pub async fn run() -> Result<(), RunError> {
    let config = load_config()?;

    let cancel_token = CancellationToken::new();
    tokio::spawn(shutdown_signal_listener(cancel_token.clone()));

    let amqp_connection = amqp_connect(&config).await?;
    let channel = amqp_connection.create_channel().await?;

    let rss_consumer = run_rss_articles_consumer(RssArticlesConsumerContext::new(
        channel,
        config.rss_articles_queue().clone(),
        cancel_token.clone(),
    ));

    let api_server = run_api_server(ApiContext {
        config,
        cancel_token: cancel_token.clone(),
    });

    let mut run_tasks = JoinSet::new();

    run_tasks.spawn(async move { api_server.await.map_err(RunError::from) });
    tracing::info!("api server task spawned");
    run_tasks.spawn(async move { rss_consumer.await.map_err(RunError::from) });
    tracing::info!("rss consumer task spawned");

    while let Some(result) = run_tasks.join_next().await {
        result??
    }

    _ = amqp_success_close(amqp_connection)
        .await
        .inspect_err(|error| tracing::error!(%error));

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
        _ = ctrl_c => tracing::trace!("ctrl + c"),
        _ = terminate => tracing::trace!("SIGTERM")
    }

    cancel_token.cancel();
}
