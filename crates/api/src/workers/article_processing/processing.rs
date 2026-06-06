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

pub(super) async fn process_article(state: &SharedAppState, request: ProcessingRequest) {
    if let Err(error) = try_process_article(state, &request).await {
        _ = articles::mark_article_as_error(
            &state.db_pool,
            request.article_id,
            &ArticleErrorReason::from(error),
        )
        .await;

        broadcast_fail(state);
    }
}

#[derive(Debug, thiserror::Error)]
enum ProcessArticleError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("article not found in db")]
    NotFound,
    #[error(transparent)]
    Chat(#[from] ExecChatError),
}

impl From<ProcessArticleError> for ArticleErrorReason {
    fn from(value: ProcessArticleError) -> Self {
        Self::new(value.to_string()).expect("process article error should not be an empty string")
    }
}

async fn try_process_article(
    state: &SharedAppState,
    request: &ProcessingRequest,
) -> Result<(), ProcessArticleError> {
    let article = articles::get_article_by_id(&state.db_pool, request.article_id)
        .await?
        .ok_or(ProcessArticleError::NotFound)?;

    let chat_request = prepare_chat_request(request, &article);
    let chat_response = state.ai.exec_chat(chat_request).await?;

    {
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
    }

    broadcast_success(state);

    Ok(())
}

fn prepare_chat_request(request: &ProcessingRequest, article: &Article) -> ChatRequest {
    let mut chat_request = ChatRequest::default().with_system(Message(nonempty_trimmed_string!(
        "Write a post based on an article"
    )));

    if let Some(context) = request.context.as_ref() {
        chat_request.push_message(ChatMessage::user(
            Message::new(format!("Additional context: {context}")).unwrap(),
        ));
    }

    chat_request.push_message(ChatMessage::user(Message(
        article.readability.text_content.0.clone(),
    )));

    chat_request
}

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
