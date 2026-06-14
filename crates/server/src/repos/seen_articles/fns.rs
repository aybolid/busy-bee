use types::Url;

use crate::infra::db::{DatabaseExecutor, DatabaseQueryResult};

/// Attempts to insert a URL into the `seen_articles` ledger.
///
/// This query utilizes an `INSERT OR IGNORE` strategy. If the `Url` already
/// exists within the database's unique index, the operation will silently succeed
/// without modifying the database or returning a constraint violation error.
///
/// # Arguments
///
/// * `executor` - A database connection or transaction pool.
/// * `url` - A reference to the parsed `Url` to mark as seen.
///
/// # Returns
///
/// Returns a [`sqlx::Result`] containing the [`DatabaseQueryResult`]. If the URL
/// was newly inserted, `rows_affected` will be `1`. If the URL was ignored due
/// to a duplicate entry, `rows_affected` will be `0`.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn create_seen_article<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &Url,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query("INSERT OR IGNORE INTO seen_articles (url) VALUES (?);").bind(url);

    query.execute(executor).await.inspect(|result| {
        tracing::trace!(
            rows_affected = result.rows_affected(),
            "created seen article"
        );
    })
}

/// Checks whether a specific URL has already been recorded in the ledger.
///
/// This query is highly optimized for performance, utilizing `SQLite`'s
/// `SELECT EXISTS(...)` pattern. Instead of returning the full row, the database
/// halts its search the moment a match is found in the index and yields a simple
/// boolean value.
///
/// # Arguments
///
/// * `executor` - A database connection or transaction pool.
/// * `url` - A reference to the parsed `Url` to check.
///
/// # Returns
///
/// Returns `true` if the URL is present in the `seen_articles` table,
/// or `false` otherwise.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn check_if_seen_article<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &Url,
) -> sqlx::Result<bool> {
    let query =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM seen_articles WHERE url = ?);").bind(url);

    query
        .fetch_one(executor)
        .await
        .inspect(|exists| tracing::trace!(exists, "checked seen article"))
}
