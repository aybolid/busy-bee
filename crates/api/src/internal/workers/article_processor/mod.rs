use lapin::{
    Channel,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::{FieldTable, ShortString},
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::internal::{
    infra::db::DatabasePool,
    repos::{
        articles::{self, ArticleId},
        types::{
            length::{MaxLength, NonEmpty},
            trimmed_string::TrimmedString,
        },
    },
};

pub struct ArticleProcessorContext {
    db_pool: DatabasePool,
    channel: Channel,
    queue: ShortString,
    cancel_token: CancellationToken,
}

impl ArticleProcessorContext {
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
pub enum ArticleProcessorError {
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_article_processor(
    context: ArticleProcessorContext,
) -> Result<(), ArticleProcessorError> {
    let mut consumer = context
        .channel
        .basic_consume(
            context.queue.clone(),
            "article_processor_consumer".into(),
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
                    _ = process_article_delivery(&context.db_pool, delivery).await;
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
enum ProcessArticleDeliveryError {
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("article {0:?} not found")]
    ArticleNotFound(ArticleId),
}

pub type AdditionalContext = MaxLength<500, NonEmpty<TrimmedString>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ArticleDeliveryPayload {
    pub article_id: ArticleId,
    pub context: Option<AdditionalContext>,
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_article_delivery(
    db_pool: &DatabasePool,
    delivery: lapin::Result<Delivery>,
) -> Result<(), ProcessArticleDeliveryError> {
    let delivery = delivery?;
    delivery.ack(BasicAckOptions::default()).await?;

    let payload = serde_json::from_slice::<ArticleDeliveryPayload>(&delivery.data)?;
    tracing::trace!(article_id = ?payload.article_id, context = ?payload.context, "got payload");

    let article = articles::get_article_by_id(db_pool, payload.article_id)
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(
            payload.article_id,
        ))?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    articles::mark_article_as_processed(db_pool, article.id())
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(article.id()))?;

    Ok(())
}
