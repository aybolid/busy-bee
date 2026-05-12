use lapin::{
    Channel,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::{FieldTable, ShortString},
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::internal::{
    infra::{amqp::declare_durable_queue, db::DatabasePool},
    repos::articles::{self, Article, FromParsedArticeError},
};

pub struct RssArticlesConsumerContext {
    db_pool: DatabasePool,
    channel: Channel,
    queue: ShortString,
    cancel_token: CancellationToken,
}

impl RssArticlesConsumerContext {
    pub fn new(
        db_pool: DatabasePool,
        channel: Channel,
        queue: ShortString,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            db_pool,
            channel,
            queue,
            cancel_token,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RssArticlesConsumerError {
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_rss_articles_consumer(
    context: RssArticlesConsumerContext,
) -> Result<(), RssArticlesConsumerError> {
    let queue = declare_durable_queue(&context.channel, context.queue.clone()).await?;

    let mut consumer = context
        .channel
        .basic_consume(
            queue.name().clone(),
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
                    _ = process_rss_delivery(&context.db_pool, delivery).await;
                } else {
                    tracing::error!("consumer stream ended unexpectedly");
                    break;
                }
            }
            () = context.cancel_token.cancelled() => {
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
    db_pool: &DatabasePool,
    delivery: lapin::Result<Delivery>,
) -> Result<(), ProcessRssDeliveryError> {
    let delivery = delivery?;
    delivery.ack(BasicAckOptions::default()).await?;

    let parsed_article = serde_json::from_slice::<rss_reader::ParsedArticle>(&delivery.data)?;
    tracing::trace!(article_title = parsed_article.title, "got article");

    let article = Article::try_from(parsed_article)?;

    articles::create_article(db_pool, &article).await?;

    Ok(())
}
