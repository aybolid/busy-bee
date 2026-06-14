use types::Url;

use crate::infra::db::{DatabaseExecutor, DatabaseQueryResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the outcome of [`create_seen_article`] function.
pub enum CreateSeenArticleQueryResult {
    /// New entry was created.
    Created,
    /// Entry was ignored due to conflict.
    Ignored,
}

impl CreateSeenArticleQueryResult {
    pub fn is_ignored(self) -> bool {
        self == Self::Ignored
    }
}

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
/// Returns a [`sqlx::Result`] containing the [`CreateSeenArticleQueryResult`].
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn create_seen_article<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &Url,
) -> sqlx::Result<CreateSeenArticleQueryResult> {
    let query = sqlx::query("INSERT OR IGNORE INTO seen_articles (url) VALUES (?);").bind(url);

    query
        .execute(executor)
        .await
        .map(|result: DatabaseQueryResult| {
            if result.rows_affected() == 0 {
                tracing::trace!("seen article entry already exists");
                CreateSeenArticleQueryResult::Ignored
            } else {
                tracing::trace!("seen article entry created");
                CreateSeenArticleQueryResult::Created
            }
        })
}
