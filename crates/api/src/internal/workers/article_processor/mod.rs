use genai::chat::{ChatMessage, ChatRequest};
use lapin::{
    Channel,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicRejectOptions},
    types::{FieldTable, ShortString},
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::internal::{
    ai,
    infra::db::DatabasePool,
    repos::{
        article_processing_outputs,
        articles::{self, ArticleId},
        types::{
            self,
            length::{MaxLength, NonEmpty},
            trimmed_string::TrimmedString,
        },
    },
};

const POST_SYSTEM_PROMPT: &str = include_str!("prompts/post.md");

pub struct ArticleProcessorContext {
    pub ai_client: ai::SharedClient,
    pub db_pool: DatabasePool,
    pub channel: Channel,
    pub queue: ShortString,
    pub cancel_token: CancellationToken,
}

impl ArticleProcessorContext {
    pub fn new(
        ai_client: ai::SharedClient,
        db_pool: DatabasePool,
        channel: Channel,
        queue: ShortString,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            ai_client,
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
    Amqp(#[from] lapin::Error),
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
            delivery_result = consumer.next() => {
                match delivery_result {
                    Some(Ok(delivery)) => {
                        // Error logged by `tracing::instrument`
                        _ =  process_article_delivery(&context.ai_client, &context.db_pool, delivery).await;
                    }
                    Some(Err(error)) => {
                        tracing::error!(%error);
                    }
                    None => {
                        tracing::error!("consumer stream ended unexpectedly");
                        break;
                    }
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
pub enum ProcessArticleDeliveryError {
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("article {0:?} not found")]
    ArticleNotFound(ArticleId),
    #[error(transparent)]
    Genai(#[from] genai::Error),
    #[error(transparent)]
    EmptyOutput(#[from] types::length::EmptyValueError),
}

pub type AdditionalContext = MaxLength<500, NonEmpty<TrimmedString>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ArticleDeliveryPayload {
    pub article_id: ArticleId,
    pub context: Option<AdditionalContext>,
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_article_delivery(
    ai_client: &ai::SharedClient,
    db_pool: &DatabasePool,
    delivery: Delivery,
) -> Result<(), ProcessArticleDeliveryError> {
    let payload = match serde_json::from_slice::<ArticleDeliveryPayload>(&delivery.data) {
        Ok(p) => p,
        Err(e) => {
            // Reject unparseable messages without requeuing to avoid poison pill loops
            _ = delivery.reject(BasicRejectOptions { requeue: false }).await;
            return Err(e.into());
        }
    };
    tracing::trace!(article_id = ?payload.article_id, context = ?payload.context);

    if let Err(error) = process_article(ai_client, db_pool, &payload).await {
        articles::mark_article_as_error(db_pool, payload.article_id).await?;
        // Acknowledge the message since the "error state" has been safely recorded in the database.
        delivery.ack(BasicAckOptions::default()).await?;
        return Err(error);
    }

    // Acknowledge on full success
    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_article(
    ai_client: &ai::SharedClient,
    db_pool: &DatabasePool,
    payload: &ArticleDeliveryPayload,
) -> Result<(), ProcessArticleDeliveryError> {
    let article = articles::get_article_by_id(db_pool, payload.article_id)
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(
            payload.article_id,
        ))?;

    let mut chat_request = ChatRequest::new(vec![ChatMessage::system(POST_SYSTEM_PROMPT)]);

    if let Some(context) = payload.context.as_ref() {
        chat_request = chat_request.append_message(ChatMessage::user(format!(
            "Additional context: {}",
            context.as_str()
        )));
    }
    chat_request = chat_request.append_message(ChatMessage::user(article.text_content().as_str()));

    let chat_response = ai_client.exec_chat(chat_request).await?;

    let text = chat_response.into_texts().join("");
    let output_text = NonEmpty::try_new(TrimmedString::new(text))?;

    let mut tx = db_pool.begin().await?;

    article_processing_outputs::create_article_processing_output(
        &mut *tx,
        article.id(),
        &output_text,
        payload.context.as_ref(),
    )
    .await?;

    articles::mark_article_as_processed(&mut *tx, article.id())
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(article.id()))?;

    tx.commit().await?;

    Ok(())
}
