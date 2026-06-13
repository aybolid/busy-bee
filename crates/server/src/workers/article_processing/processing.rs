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
        system_prompts,
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
        system_prompt_id = %request.system_prompt_id.as_hyphenated(),
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
    ArticleNotFound,
    /// The requested system prompt could not be found in the database.
    #[error("system prompt not found in db")]
    SystemPromptNotFound,
    /// The external AI service returned an error during text generation.
    #[error(transparent)]
    Chat(#[from] ExecChatError),
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
        .ok_or(ProcessArticleError::ArticleNotFound)?;

    let chat_request = prepare_chat_request(state, request, &article).await?;
    let chat_response = state.ai.exec_chat(chat_request).await?;

    // Avoid partial updates by using DB transaction
    let mut tx = state.db_pool.begin().await?;
    tracing::trace!("database transaction started");

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
        .ok_or(ProcessArticleError::ArticleNotFound)?;

    tx.commit().await?;
    tracing::trace!("database transaction commited");

    Ok(())
}

/// Constructs the payload to be sent to the AI service.
async fn prepare_chat_request(
    state: &SharedAppState,
    request: &ProcessingRequest,
    article: &Article,
) -> Result<ChatRequest, ProcessArticleError> {
    let system_prompt = system_prompts::get_system_prompt(&state.db_pool, request.system_prompt_id)
        .await?
        .ok_or(ProcessArticleError::SystemPromptNotFound)?;

    let mut chat_request = ChatRequest::default().with_system(Message::from(system_prompt));

    if let Some(context_message) = request
        .context
        .as_ref()
        .and_then(|context| Message::new(format!("Additional context: {context}")))
    {
        tracing::trace!("with additional context");
        tracing::debug!(context_message = context_message.as_str());
        chat_request.push_message(ChatMessage::user(context_message));
    }

    chat_request.push_message(ChatMessage::user(Message(
        article.readability.text_content.0.clone(),
    )));

    Ok(chat_request)
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

    tracing::trace!("success broadcasted");
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

    tracing::trace!("fail broadcasted");
}
