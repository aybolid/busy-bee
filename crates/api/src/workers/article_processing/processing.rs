use types::nonempty_trimmed_string;

use crate::{
    ai::{ChatMessage, ChatRequest, ExecChatError, Message},
    app::{
        events::{NotificationData, NotificationString, RefetchTriggerType},
        state::SharedAppState,
    },
    repos::{
        articles::{self, Article, ArticleErrorReason},
        outputs::{self, OutputText},
    },
    workers::article_processing::ProcessingRequest,
};

/// The main entry point for the background article processing job.
///
/// This function coordinates the processing lifecycle. If the underlying processing
/// logic fails, it acts as a fallback boundary: catching the error, updating the
/// article's status in the database to reflect the failure, and broadcasting an error
/// notification to the system.
#[tracing::instrument(
    skip_all,
    fields(
        article_id = %request.article_id.as_hyphenated(),
        has_context = request.context.is_some(),
    )
)]
pub(super) async fn process_article(state: &SharedAppState, request: ProcessingRequest) {
    tracing::info!("processing article");

    if let Err(error) = try_process_article(state, &request).await {
        _ = articles::mark_article_as_error(
            &state.db_pool,
            request.article_id,
            &ArticleErrorReason::from(&error),
        )
        .await;

        tracing::info!("failed to process article");
        broadcast_fail(state);
    } else {
        tracing::info!("article processed successfully");
        broadcast_success(state);
    }
}

/// Internal error type representing the various failure modes during article processing.
#[derive(Debug, thiserror::Error)]
enum ProcessArticleError {
    /// A database operation failed.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    /// The requested article could not be found in the database.
    #[error("article not found in db")]
    NotFound,
    /// The external AI service returned an error during text generation.
    #[error(transparent)]
    Chat(#[from] ExecChatError),
}

impl From<&ProcessArticleError> for ArticleErrorReason {
    fn from(value: &ProcessArticleError) -> Self {
        Self::new(value.to_string()).expect("process article error should not be an empty string")
    }
}

/// Executes the core logic for processing an article using the AI model.
///
/// # Workflow
/// 1. Fetches the raw article from the database.
/// 2. Compiles a prompt incorporating the system instructions, optional user context, and article text.
/// 3. Sends the prompt to the AI service to generate a response.
/// 4. Opens a database transaction to atomically save the AI's output and update the article's status.
/// 5. Broadcasts success events to notify connected clients of the new data.
async fn try_process_article(
    state: &SharedAppState,
    request: &ProcessingRequest,
) -> Result<(), ProcessArticleError> {
    let article = articles::get_article_by_id(&state.db_pool, request.article_id)
        .await?
        .ok_or(ProcessArticleError::NotFound)?;

    let chat_request = prepare_chat_request(request, &article);
    let chat_response = state.ai.exec_chat(chat_request).await?;

    let mut tx = state.db_pool.begin().await?;

    outputs::create_output(
        &mut *tx,
        article.id,
        request.context.as_ref(),
        &state.ai.model,
        &OutputText(chat_response.content.0),
        &chat_response.usage,
    )
    .await?;

    articles::mark_article_as_processed(&mut *tx, article.id)
        .await?
        .ok_or(ProcessArticleError::NotFound)?;

    tx.commit().await?;

    Ok(())
}

/// Constructs the payload to be sent to the AI service.
fn prepare_chat_request(request: &ProcessingRequest, article: &Article) -> ChatRequest {
    let mut chat_request = ChatRequest::default().with_system(Message(nonempty_trimmed_string!(
        "Write a post based on an article"
    )));

    if let Some(context_message) = request
        .context
        .as_ref()
        .and_then(|context| Message::new(format!("Additional context: {context}")))
    {
        chat_request.push_message(ChatMessage::user(context_message));
    }

    chat_request.push_message(ChatMessage::user(Message(
        article.readability.text_content.0.clone(),
    )));

    chat_request
}

/// Dispatches a success notification and triggers a data refresh.
///
/// Tells connected clients to refetch both the `Articles` and `Outputs` lists
/// to display the newly generated content.
fn broadcast_success(state: &SharedAppState) {
    let notification = NotificationData::info(NotificationString(nonempty_trimmed_string!(
        "Article processed"
    )))
    .with_description(NotificationString::new(
        "Article was processed successfully",
    ));

    state
        .app_events_broadcaster
        .send_refetch_triggers([RefetchTriggerType::Articles, RefetchTriggerType::Outputs])
        .send_notification(notification);
}

/// Dispatches a failure notification and triggers a data refresh.
///
/// Tells connected clients to refetch the `Articles` list so they can see
/// the updated error status and reason for the failed article.
fn broadcast_fail(state: &SharedAppState) {
    let notification = NotificationData::error(NotificationString(nonempty_trimmed_string!(
        "Failed to process article"
    )))
    .with_description(NotificationString::new(
        "Something went wrong during article processing",
    ));

    state
        .app_events_broadcaster
        .send_refetch_trigger(RefetchTriggerType::Articles)
        .send_notification(notification);
}
