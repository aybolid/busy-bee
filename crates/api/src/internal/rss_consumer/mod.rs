use lapin::{
    Channel,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::{FieldTable, ShortString},
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::internal::infra::amqp::declare_durable_queue;

pub struct RssArticlesConsumerContext {
    channel: Channel,
    queue: ShortString,
    cancel_token: CancellationToken,
}

impl RssArticlesConsumerContext {
    pub fn new(channel: Channel, queue: ShortString, cancel_token: CancellationToken) -> Self {
        Self {
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
                    _ = process_rss_delivery(delivery).await;
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
enum ProcessRssDeliveryError {
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_rss_delivery(
    delivery: lapin::Result<Delivery>,
) -> Result<(), ProcessRssDeliveryError> {
    let delivery = delivery?;
    delivery.ack(BasicAckOptions::default()).await?;

    let article = serde_json::from_slice::<rss_reader::ParsedArticle>(&delivery.data)?;
    tracing::trace!(article_title = article.title, "got article");

    Ok(())
}
