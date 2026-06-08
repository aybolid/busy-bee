use crate::{
    ai::{ModelName, Usage},
    infra::db::{DatabaseExecutor, DatabaseQueryResult},
    repos::{
        Pagination,
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
#[tracing::instrument(level = "trace", skip_all, fields(output_id = %id.as_hyphenated()), err(Debug))]
pub async fn get_output_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: OutputId,
) -> sqlx::Result<Option<Output>> {
    let query = sqlx::query_as("SELECT * FROM outputs WHERE id = ?;").bind(id);

    query.fetch_optional(executor).await.inspect(|output| {
        tracing::trace!(
            "{}",
            if output.is_some() {
                "output fetched from db"
            } else {
                "output not found"
            }
        );
    })
}

/// Counts the total number of AI outputs stored in the database.
///
/// # Returns
///
/// The total count of outputs as a `usize`.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn count_outputs<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<usize> {
    let query = sqlx::query_scalar("SELECT COUNT(*) FROM outputs;");
    query
        .fetch_one(executor)
        .await
        .map(|count: i64| count as usize)
        .inspect(|count| tracing::trace!(count, "got outputs count"))
}

/// Retrieves a paginated list of AI outputs, ordered by creation date (newest first).
///
/// # Returns
///
/// A [`Vec`] containing up to `limit` number of [`Output`] records.
#[allow(clippy::cast_possible_wrap)]
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn get_outputs<'c>(
    executor: impl DatabaseExecutor<'c>,
    pagination: Pagination,
) -> sqlx::Result<Vec<Output>> {
    let (limit, offset) = pagination.as_limit_and_offset();

    let query = sqlx::query_as("SELECT * FROM outputs ORDER BY created_at DESC LIMIT ? OFFSET ?;")
        .bind(limit)
        .bind(offset);

    query
        .fetch_all(executor)
        .await
        .inspect(|_| tracing::trace!("outputs fetched from db"))
}

/// Persists a newly generated AI output to the database.
///
/// This function generates a new [`OutputId`] and inserts the provided details
/// into the database.
///
/// # Returns
///
/// The [`DatabaseQueryResult`] indicating the success of the insert operation.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
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

    query
        .execute(executor)
        .await
        .inspect(|_| tracing::trace!("output created"))
}

/// Deletes an output from the database by its unique identifier.
///
/// # Returns
///
/// Returns [`OutputId`] if the output was successfully deleted, or [`None`]
/// if no such output existed.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database deletion operation fails.
#[tracing::instrument(level = "trace", skip_all, fields(output_id = %id.as_hyphenated()), err(Debug))]
pub async fn delete_output_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: OutputId,
) -> sqlx::Result<Option<OutputId>> {
    let query = sqlx::query_scalar(
        "
        DELETE FROM outputs
        WHERE
            id = ?
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "output deleted"
            } else {
                "output to delete not found"
            }
        );
    })
}
