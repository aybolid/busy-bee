use std::num::NonZeroU8;

use chrono::{DateTime, Utc};
use types::{NonEmpty, TrimmedString};
use uuid::Uuid;

use crate::{
    infra::db::{DatabaseExecutor, DatabaseQueryResult},
    repos::articles::ArticleId,
    workers::article_processor::AdditionalContext,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ArticleProcessingOutputId(Uuid);

impl ArticleProcessingOutputId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

pub type OutputText = NonEmpty<TrimmedString>;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ArticleProcessingOutput {
    id: ArticleProcessingOutputId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    article_id: Option<ArticleId>,

    user_context: Option<AdditionalContext>,
    output_text: OutputText,
}

#[tracing::instrument(level = "trace", skip(executor), err(Debug))]
pub async fn get_article_processing_output_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleProcessingOutputId,
) -> sqlx::Result<Option<ArticleProcessingOutput>> {
    let query = sqlx::query_as("SELECT * FROM article_processing_outputs WHERE id = ?;").bind(id);
    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip_all, err, ret)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn count_article_processing_outputs<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<usize> {
    let query = sqlx::query_scalar("SELECT COUNT(*) FROM article_processing_outputs;");
    query
        .fetch_one(executor)
        .await
        .map(|count: i64| count as usize)
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
#[allow(clippy::cast_possible_wrap)]
pub async fn get_article_processing_outputs<'c>(
    executor: impl DatabaseExecutor<'c>,
    page_index: usize,
    limit: NonZeroU8,
) -> sqlx::Result<Vec<ArticleProcessingOutput>> {
    let limit = limit.get();
    let offset = page_index * usize::from(limit);
    tracing::trace!(limit, offset);

    let query = sqlx::query_as(
        "SELECT * FROM article_processing_outputs ORDER BY created_at DESC LIMIT ? OFFSET ?;",
    )
    .bind(i64::from(limit))
    .bind(offset as i64);

    query.fetch_all(executor).await
}

#[tracing::instrument(level = "trace", skip_all, ret, err(Debug))]
pub async fn create_article_processing_output<'c>(
    executor: impl DatabaseExecutor<'c>,
    article_id: ArticleId,
    output_text: &OutputText,
    user_context: Option<&AdditionalContext>,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query(
        "
        INSERT INTO article_processing_outputs
            (id, article_id, user_context, output_text)
        VALUES
            (?, ?, ?, ?)
        ",
    )
    .bind(ArticleProcessingOutputId::new())
    .bind(article_id)
    .bind(user_context)
    .bind(output_text);

    query.execute(executor).await
}
