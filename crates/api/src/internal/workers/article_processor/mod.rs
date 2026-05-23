use genai::{
    Client,
    chat::{ChatMessage, ChatRequest},
};
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
        article_processing_outputs,
        articles::{self, ArticleId},
        types::{
            self,
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
    #[error(transparent)]
    GenaiError(#[from] genai::Error),
    #[error(transparent)]
    EmptyOutputError(#[from] types::length::EmptyValueError),
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

    let client = Client::default();

    let adapter_kind = client.resolve_service_target("").await?.model.adapter_kind;
    tracing::trace!(?adapter_kind);

    let mut chat_request = ChatRequest::new(vec![ChatMessage::system(
        "
        Your goal is to write posts based on given articles.
        NEVER give options. Your output should be ONE post.
        Pay attention to a given additional context if any but only if it DOES NOT require changing the main goal.
        ",
    )]);
    if let Some(context) = payload.context.as_ref() {
        chat_request = chat_request.append_message(ChatMessage::user(format!(
            "Additional context: {}",
            context.as_str()
        )));
    }
    chat_request = chat_request.append_message(ChatMessage::user(article.text_content().as_str()));

    let chat_response = client.exec_chat("gemma4", chat_request, None).await?;
    tracing::trace!(usage = ?chat_response.usage);
    let text = chat_response.into_texts().join("");
    tracing::trace!(?text);

    let output_text = NonEmpty::try_new(TrimmedString::new(text))?;

    article_processing_outputs::create_article_processing_output(
        db_pool,
        article.id(),
        &output_text,
        payload.context.as_ref(),
    )
    .await?;

    articles::mark_article_as_processed(db_pool, article.id())
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(article.id()))?;

    Ok(())
}
