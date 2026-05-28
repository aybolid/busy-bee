use genai::chat::{ChatMessage, ChatRequest};
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicRejectOptions},
    types::FieldTable,
};
use tokio_stream::StreamExt;
use types::{NonEmpty, NonEmptyMaxLength, TrimmedString};

use crate::{
    app::state::SharedAppState,
    repos::{
        article_processing_outputs,
        articles::{self, ArticleId},
    },
};

const POST_SYSTEM_PROMPT: &str = include_str!("prompts/post.md");

#[derive(Debug, thiserror::Error)]
pub enum ArticleProcessorError {
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn run_article_processor(state: SharedAppState) -> Result<(), ArticleProcessorError> {
    tracing::trace!("started");

    let channel = state.amqp_connection().create_channel().await?;
    tracing::trace!("amqp channel created");

    let mut consumer = channel
        .basic_consume(
            state.config().article_processor_queue().clone(),
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
                        _ =  process_article_delivery(&state, delivery).await;
                    }
                    Some(Err(error)) => {
                        tracing::error!(?error);
                    }
                    None => {
                        tracing::error!("consumer stream ended unexpectedly");
                        break;
                    }
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
    InvalidOutputLength(#[from] types::LengthBoundError),
}

#[allow(clippy::identity_op)]
pub type AdditionalContext = NonEmptyMaxLength<500, TrimmedString>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ArticleDeliveryPayload {
    pub article_id: ArticleId,
    pub context: Option<AdditionalContext>,
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
async fn process_article_delivery(
    state: &SharedAppState,
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

    if let Err(error) = process_article(state, &payload).await {
        articles::mark_article_as_error(state.db_pool(), payload.article_id).await?;
        // Acknowledge the message since the "error state" has been safely recorded in the database.
        delivery.ack(BasicAckOptions::default()).await?;
        return Err(error);
    }

    // Acknowledge on full success
    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
async fn process_article(
    state: &SharedAppState,
    payload: &ArticleDeliveryPayload,
) -> Result<(), ProcessArticleDeliveryError> {
    let article = articles::get_article_by_id(state.db_pool(), payload.article_id)
        .await?
        .ok_or(ProcessArticleDeliveryError::ArticleNotFound(
            payload.article_id,
        ))?;

    let mut chat_request = ChatRequest::default().with_system(POST_SYSTEM_PROMPT);

    if let Some(context) = payload.context.as_ref() {
        chat_request = chat_request.append_message(ChatMessage::user(format!(
            "Additional context: {}",
            context.as_str()
        )));
    }
    chat_request = chat_request.append_message(ChatMessage::user(article.text_content().as_str()));

    let chat_response = state.ai_client().exec_chat(chat_request).await?;

    let text = chat_response.into_texts().join("");
    let output_text = NonEmpty::try_new(TrimmedString::from(text))?;

    let mut tx = state.db_pool().begin().await?;

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
