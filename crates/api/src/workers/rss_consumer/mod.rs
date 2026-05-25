use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use tokio_stream::StreamExt;

use crate::{
    app::state::SharedAppState,
    repos::articles::{self, Article, FromParsedArticeError},
};

#[derive(Debug, thiserror::Error)]
pub enum RssArticlesConsumerError {
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_rss_articles_consumer(
    state: SharedAppState,
) -> Result<(), RssArticlesConsumerError> {
    let channel = state.amqp_connection().create_channel().await?;

    let mut consumer = channel
        .basic_consume(
            state.config().rss_articles_queue().clone(),
            "rss_articles_consumer".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    tracing::trace!("consumer created");

    tracing::info!("processing delivery");
    loop {
        tokio::select! {
            delivery = consumer.next() => {
                if let Some(delivery) = delivery {
                    _ = process_rss_delivery(&state, delivery).await;
                } else {
                    tracing::error!("consumer stream ended unexpectedly");
                    break;
                }
            }
            () = state.cancel_token().cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
enum ProcessRssDeliveryError {
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ConvertError(#[from] FromParsedArticeError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_rss_delivery(
    state: &SharedAppState,
    delivery: lapin::Result<Delivery>,
) -> Result<(), ProcessRssDeliveryError> {
    let delivery = delivery?;
    delivery.ack(BasicAckOptions::default()).await?;

    let parsed_article = serde_json::from_slice::<rss_reader::ParsedArticle>(&delivery.data)?;
    tracing::trace!(article_title = parsed_article.title, "got article");

    let article = Article::try_from(parsed_article)?;

    articles::create_article(state.db_pool(), &article).await?;

    Ok(())
}
