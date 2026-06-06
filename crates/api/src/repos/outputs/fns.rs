use std::num::NonZeroU8;

use crate::{
    ai::{ModelName, Usage},
    infra::db::{DatabaseExecutor, DatabaseQueryResult},
    repos::{
        articles::ArticleId,
        outputs::{Output, OutputId, OutputText},
    },
    workers::article_processing::ProcessingUserContext,
};

/// Retrieves a specific AI output by its unique identifier.
///
/// # Returns
///
/// * [`Output`] if the record exists.
/// * [`None`] if no record matches the given ID.
pub async fn get_output_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: OutputId,
) -> sqlx::Result<Option<Output>> {
    let query = sqlx::query_as("SELECT * FROM outputs WHERE id = ?;").bind(id);
    query.fetch_optional(executor).await
}

/// Counts the total number of AI outputs stored in the database.
///
/// # Returns
///
/// The total count of outputs as a `usize`.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn count_outputs<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<usize> {
    let query = sqlx::query_scalar("SELECT COUNT(*) FROM outputs;");
    query
        .fetch_one(executor)
        .await
        .map(|count: i64| count as usize)
}

/// Retrieves a paginated list of AI outputs, ordered by creation date (newest first).
///
/// # Returns
///
/// A [`Vec`] containing up to `limit` number of [`Output`] records.
#[allow(clippy::cast_possible_wrap)]
pub async fn get_outputs<'c>(
    executor: impl DatabaseExecutor<'c>,
    page_index: usize,
    limit: NonZeroU8,
) -> sqlx::Result<Vec<Output>> {
    let limit = limit.get();
    let offset = page_index * usize::from(limit);

    let query = sqlx::query_as("SELECT * FROM outputs ORDER BY created_at DESC LIMIT ? OFFSET ?;")
        .bind(i64::from(limit))
        .bind(offset as i64);

    query.fetch_all(executor).await
}

/// Persists a newly generated AI output to the database.
///
/// This function generates a new [`OutputId`] and inserts the provided details
/// into the database.
///
/// # Returns
///
/// The [`DatabaseQueryResult`] indicating the success of the insert operation.
pub async fn create_output<'c>(
    executor: impl DatabaseExecutor<'c>,
    article_id: ArticleId,
    user_context: Option<&ProcessingUserContext>,
    model: &ModelName,
    output_text: &OutputText,
    usage: &Usage,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query(
        "
        INSERT INTO outputs
            (
                id, article_id, user_context,
                text, model, usage
            )
        VALUES
            (
                ?, ?, ?,
                ?, ?, ?
            )
        ",
    )
    .bind(OutputId::new())
    .bind(article_id)
    .bind(user_context)
    .bind(output_text)
    .bind(model)
    .bind(sqlx::types::Json(usage));

    query.execute(executor).await
}
