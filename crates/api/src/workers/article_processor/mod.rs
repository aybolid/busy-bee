use genai::chat::{ChatMessage, ChatRequest};
use tokio::sync::mpsc;
use types::{NonEmpty, NonEmptyMaxLength, TrimmedString};

use crate::{
    app::state::SharedAppState,
    repos::{
        article_processing_outputs,
        articles::{self, ArticleErrorReason, ArticleId},
    },
};

const ARTICLE_PROCESSING_CHANNEL_CAPACITY: usize = 100;

pub type ArticleProcessingSender = mpsc::Sender<ArticleProcessingRequest>;
pub type ArticleProcessingReceiver = mpsc::Receiver<ArticleProcessingRequest>;

#[tracing::instrument(level = "trace", skip_all)]
pub fn create_article_processing_channel() -> (ArticleProcessingSender, ArticleProcessingReceiver) {
    tracing::trace!(ARTICLE_PROCESSING_CHANNEL_CAPACITY);

    let channel = mpsc::channel(ARTICLE_PROCESSING_CHANNEL_CAPACITY);
    tracing::trace!("channel created");

    channel
}

pub type ProcessArticleUserContext = NonEmptyMaxLength<500, TrimmedString>;

#[derive(Debug)]
pub struct ArticleProcessingRequest {
    pub article_id: ArticleId,
    pub context: Option<ProcessArticleUserContext>,
}

#[derive(Debug, thiserror::Error)]
pub enum ArticleProcessorError {}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn run_article_processor(
    state: SharedAppState,
    mut rx: mpsc::Receiver<ArticleProcessingRequest>,
) -> Result<(), ArticleProcessorError> {
    tracing::trace!("started");

    tracing::trace!("listening for processing requests");
    loop {
        tokio::select! {
            request = rx.recv() => {
                if let Some(request) = request {
                    tracing::trace!(?request);
                    handle_article_processing(&state, request).await;
                } else {
                    tracing::trace!("mpsc channel closed");
                    break;
                }
            }
            () = state.cancel_token.cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
#[allow(clippy::collapsible_if)]
async fn handle_article_processing(state: &SharedAppState, request: ArticleProcessingRequest) {
    let article_id = request.article_id;

    if let Err(error) = process_article(state, request).await {
        if !matches!(error, ProcessArticleError::ArticleNotFound(_)) {
            if let Err(error) = articles::mark_article_as_error(
                &state.db_pool,
                article_id,
                ArticleErrorReason::new(TrimmedString::from(error.to_string())).as_ref(),
            )
            .await
            {
                tracing::error!(?error);
            }
        }
    }
}

const POST_SYSTEM_PROMPT: &str = include_str!("prompts/post.md");

#[derive(Debug, thiserror::Error)]
pub enum ProcessArticleError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("article {0:?} not found")]
    ArticleNotFound(ArticleId),
    #[error(transparent)]
    Genai(#[from] genai::Error),
    #[error(transparent)]
    InvalidOutputLength(#[from] types::LengthBoundError),
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
async fn process_article(
    state: &SharedAppState,
    request: ArticleProcessingRequest,
) -> Result<(), ProcessArticleError> {
    let article = articles::get_article_by_id(&state.db_pool, request.article_id)
        .await?
        .ok_or(ProcessArticleError::ArticleNotFound(request.article_id))?;

    let mut chat_request = ChatRequest::default().with_system(POST_SYSTEM_PROMPT);

    if let Some(context) = request.context.as_ref() {
        chat_request = chat_request.append_message(ChatMessage::user(format!(
            "Additional context: {}",
            context.as_str()
        )));
    }
    chat_request = chat_request.append_message(ChatMessage::user(article.text_content.as_str()));

    let chat_response = state.ai_client.exec_chat(chat_request).await?;

    let usage = chat_response.usage.clone();
    let text = chat_response.into_texts().join("");
    let output_text = NonEmpty::try_new(TrimmedString::from(text))?;

    let mut tx = state.db_pool.begin().await?;

    article_processing_outputs::create_article_processing_output(
        &mut *tx,
        article.id,
        &state.ai_client.model,
        &output_text,
        request.context.as_ref(),
        &usage,
    )
    .await?;

    articles::mark_article_as_processed(&mut *tx, article.id)
        .await?
        .ok_or(ProcessArticleError::ArticleNotFound(article.id))?;

    tx.commit().await?;

    Ok(())
}
