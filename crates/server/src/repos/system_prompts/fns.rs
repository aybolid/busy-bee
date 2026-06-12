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

/// Retrieves system prompt by ID from the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// row cannot be decoded into [`SystemPrompt`] instance.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_system_prompt<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: SystemPromptId,
) -> sqlx::Result<Option<SystemPrompt>> {
    let query = sqlx::query_as(
        "
        SELECT * FROM system_prompts
        WHERE id = ?;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|prompt| {
        tracing::trace!(
            "{}",
            if prompt.is_some() {
                "system prompt fetched from db"
            } else {
                "system prompt not found"
            }
        );
    })
}

/// Retrieves all system prompts from the database.
///
/// The returned prompts are ordered chronologically by their creation time ([`SystemPrompt::created_at`]).
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// rows cannot be decoded into [`SystemPrompt`] instances.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_system_prompts<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<Vec<SystemPrompt>> {
    let query = sqlx::query_as(
        "
        SELECT * FROM system_prompts
        ORDER BY created_at DESC;
        ",
    );

    query
        .fetch_all(executor)
        .await
        .inspect(|_| tracing::trace!("system prompts fetched from db"))
}

/// Deletes a system prompt from the database by its unique identifier.
///
/// # Returns
///
/// Returns [`SystemPromptId`] if the feed was found and successfully deleted,
/// or [`None`] if no feed with that ID existed.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database deletion operation fails.
#[tracing::instrument(level = "trace", skip_all, fields(system_prompt_id = %id.as_hyphenated()), err(Debug))]
pub async fn delete_system_prompt_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: SystemPromptId,
) -> sqlx::Result<Option<SystemPromptId>> {
    let query = sqlx::query_scalar(
        "
        DELETE FROM system_prompts
        WHERE id = ?
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "system prompt deleted"
            } else {
                "system prompt to delete not found"
            }
        );
    })
}
