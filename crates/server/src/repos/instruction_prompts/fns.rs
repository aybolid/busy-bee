use sqlx::QueryBuilder;

use crate::{
    infra::db::DatabaseExecutor,
    repos::{
        VersionNumber,
        instruction_prompts::{
            InstructionPrompt, InstructionPromptId, InstructionPromptIds, InstructionPromptText,
            InstructionPromptTitle,
        },
    },
};

/// Creates an instruction prompt in the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database insert fails.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn create_instruction_prompt<'c>(
    executor: impl DatabaseExecutor<'c>,
    title: &InstructionPromptTitle,
    text: &InstructionPromptText,
) -> sqlx::Result<InstructionPrompt> {
    let query = sqlx::query_as(
        "
        INSERT INTO instruction_prompts (
            id,
            title,
            text
        )
        VALUES (?, ?, ?)
        RETURNING *;
        ",
    )
    .bind(InstructionPromptId::new())
    .bind(title)
    .bind(text);

    query
        .fetch_one(executor)
        .await
        .inspect(|_| tracing::trace!("instruction prompt created"))
}

/// Retrieves instruction prompt by ID from the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// row cannot be decoded into [`InstructionPrompt`] instance.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_instruction_prompt<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: InstructionPromptId,
) -> sqlx::Result<Option<InstructionPrompt>> {
    let query = sqlx::query_as(
        "
        SELECT * FROM instruction_prompts
        WHERE id = ?;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|prompt| {
        tracing::trace!(
            "{}",
            if prompt.is_some() {
                "instruction prompt fetched from db"
            } else {
                "instruction prompt not found"
            }
        );
    })
}

/// Retrieves instruction prompts by IDs from the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// rows cannot be decoded into [`InstructionPrompt`] instances.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_instruction_prompts_by_ids<'c>(
    executor: impl DatabaseExecutor<'c>,
    ids: &InstructionPromptIds,
) -> sqlx::Result<Vec<InstructionPrompt>> {
    let mut query_builder = QueryBuilder::new("SELECT * FROM instruction_prompts WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in ids.inner() {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build_query_as();

    query.fetch_all(executor).await.inspect(|prompts| {
        tracing::trace!(count = prompts.len(), "instruction prompts fetched from db");
    })
}

/// Retrieves all instruction prompts from the database.
///
/// The returned prompts are ordered chronologically by their creation time ([`InstructionPrompt::created_at`]).
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// rows cannot be decoded into [`InstructionPrompt`] instances.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_instruction_prompts<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<Vec<InstructionPrompt>> {
    let query = sqlx::query_as(
        "
        SELECT * FROM instruction_prompts
        ORDER BY created_at DESC;
        ",
    );

    query
        .fetch_all(executor)
        .await
        .inspect(|_| tracing::trace!("instruction prompts fetched from db"))
}

/// Deletes an instruction prompt from the database by its unique identifier.
///
/// # Returns
///
/// Returns [`InstructionPromptId`] if the prompt was found and successfully deleted,
/// or [`None`] if no prompt with that ID existed.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database deletion operation fails.
#[tracing::instrument(level = "trace", skip_all, fields(instruction_prompt_id = %id.as_hyphenated()), err(Debug))]
pub async fn delete_instruction_prompt_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: InstructionPromptId,
) -> sqlx::Result<Option<InstructionPromptId>> {
    let query = sqlx::query_scalar(
        "
        DELETE FROM instruction_prompts
        WHERE id = ?
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "instruction prompt deleted"
            } else {
                "instruction prompt to delete not found"
            }
        );
    })
}

/// Represents the data required to update an existing instruction prompt.
///
/// This struct uses the builder pattern to allow selective updating
/// of the prompts's fields.
#[derive(Debug)]
pub struct InstructionPromptUpdateData<'a> {
    id: InstructionPromptId,
    version: VersionNumber,
    title: Option<&'a InstructionPromptTitle>,
    text: Option<&'a InstructionPromptText>,
}

impl<'a> InstructionPromptUpdateData<'a> {
    /// Creates a new [`InstructionPromptUpdateData`] instance for the specified ID and the expected version.
    ///
    /// By default, no fields are configured for an update.
    pub fn new(id: InstructionPromptId, version: VersionNumber) -> Self {
        Self {
            id,
            version,
            text: None,
            title: None,
        }
    }

    /// Sets the new title for the record.
    pub fn title(mut self, title: Option<&'a InstructionPromptTitle>) -> Self {
        self.title = title;
        self
    }

    /// Sets the new text for the record.
    pub fn text(mut self, text: Option<&'a InstructionPromptText>) -> Self {
        self.text = text;
        self
    }
}

/// Updates an existing instruction prompt record in the database.
///
/// This function applies the changes specified in the [`InstructionPromptUpdateData`] payload.
/// Any fields set to `None` in the payload will retain their current values
/// in the database using the `COALESCE` sql function.
///
/// # Returns
///
/// * `Some(InstructionPrompt)` containing the updated record if the update succeeded.
/// * `None` if no record matching the ID and the expected version was found.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update query fails.
#[tracing::instrument(level = "trace", skip(executor), err(Debug))]
pub async fn update_instruction_prompt_by_id<'c, 'a>(
    executor: impl DatabaseExecutor<'c>,
    data: &InstructionPromptUpdateData<'a>,
) -> sqlx::Result<Option<InstructionPrompt>> {
    // VOLATILE: Changing WHERE clause will break API hanlder logic.
    let query = sqlx::query_as(
        "
        UPDATE instruction_prompts
        SET
            title = COALESCE(?, instruction_prompts.title),
            text = COALESCE(?, instruction_prompts.text),
            version = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE
            id = ? AND version = ?
        RETURNING *;
        ",
    )
    .bind(data.title)
    .bind(data.text)
    .bind(data.version.get() + 1)
    .bind(data.id)
    .bind(data.version);

    query.fetch_optional(executor).await.inspect(|output| {
        tracing::trace!(
            "{}",
            if output.is_some() {
                "instruction prompt updated"
            } else {
                "instruction prompt to update not found"
            }
        );
    })
}
