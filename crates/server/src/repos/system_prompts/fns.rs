use crate::{
    infra::db::DatabaseExecutor,
    repos::system_prompts::{SystemPrompt, SystemPromptId, SystemPromptText, SystemPromptTitle},
};

/// Creates a newly submitted system prompt in the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database insert fails.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn create_system_prompt<'c>(
    executor: impl DatabaseExecutor<'c>,
    title: &SystemPromptTitle,
    text: &SystemPromptText,
) -> sqlx::Result<SystemPrompt> {
    let query = sqlx::query_as(
        "
        INSERT INTO system_prompts (
            id,
            title,
            text
        )
        VALUES (?, ?, ?)
        RETURNING *;
        ",
    )
    .bind(SystemPromptId::new())
    .bind(title)
    .bind(text);

    query
        .fetch_one(executor)
        .await
        .inspect(|_| tracing::trace!("system prompt created"))
}
