use tokio::sync::mpsc;
use types::{NonEmptyMaxLength, TrimmedString, nonempty_trimmed_string};

use crate::{
    ai::{ChatMessage, ChatRequest, ExecChatError, Message},
    app::{
        events::{NotificationData, NotificationString, RefetchTriggerType},
        state::SharedAppState,
    },
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
            _ = articles::mark_article_as_error(
                &state.db_pool,
                article_id,
                &ArticleErrorReason::new(error).unwrap_or_else(|| {
                    ArticleErrorReason(nonempty_trimmed_string!(
                        "Unexpected error occurred while processing article"
                    ))
                }),
            )
            .await;

            state
                .app_events_broadcaster
                .send_refetch_trigger(RefetchTriggerType::Articles);

            state.app_events_broadcaster.send_notification(
                NotificationData::error(NotificationString(nonempty_trimmed_string!(
                    "Failed to process article"
                )))
                .with_description(NotificationString::new(
                    "Article was not processed successfully",
                )),
            );
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessArticleError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("article {0:?} not found")]
    ArticleNotFound(ArticleId),
    #[error(transparent)]
    Chat(#[from] ExecChatError),
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

    let mut chat_request = ChatRequest::default().with_system(Message(nonempty_trimmed_string!(
        "Write a post based on an article"
    )));
    if let Some(context) = request.context.as_ref() {
        chat_request.push_message(ChatMessage::user(
            Message::new(format!("Additional context: {context}")).unwrap(),
        ));
    }
    chat_request.push_message(ChatMessage::user(Message(
        article.readability.text_content.0,
    )));

    let chat_response = state.ai.exec_chat(chat_request).await?;

    let mut tx = state.db_pool.begin().await?;

    article_processing_outputs::create_article_processing_output(
        &mut *tx,
        article.id,
        request.context.as_ref(),
        &state.ai.model,
        &chat_response.content.0,
        &chat_response.usage,
    )
    .await?;

    articles::mark_article_as_processed(&mut *tx, article.id)
        .await?
        .ok_or(ProcessArticleError::ArticleNotFound(article.id))?;

    tx.commit().await?;

    state
        .app_events_broadcaster
        .send_refetch_trigger(RefetchTriggerType::Articles);
    state
        .app_events_broadcaster
        .send_refetch_trigger(RefetchTriggerType::ArticleProcessingOutputs);

    state.app_events_broadcaster.send_notification(
        NotificationData::info(NotificationString(nonempty_trimmed_string!(
            "Article processed"
        )))
        .with_description(NotificationString::new(format!(
            r#"Article "{}" was processed successfully"#,
            article.readability.title
        ))),
    );

    Ok(())
}
