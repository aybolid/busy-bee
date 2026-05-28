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

    model: NonEmpty<TrimmedString>,
    prompt_tokens: Option<i32>,
    prompt_cache_creation_tokens: Option<i32>,
    prompt_cached_tokens: Option<i32>,
    prompt_audio_tokens: Option<i32>,
    completion_tokens: Option<i32>,
    completion_accepted_prediction_tokens: Option<i32>,
    completion_rejected_prediction_tokens: Option<i32>,
    completion_reasoning_tokens: Option<i32>,
    completion_audio_tokens: Option<i32>,
    total_tokens: Option<i32>,
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
    model: &NonEmpty<TrimmedString>,
    output_text: &OutputText,
    user_context: Option<&AdditionalContext>,
    usage: &genai::chat::Usage,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query(
        "
        INSERT INTO article_processing_outputs
            (
                id, article_id, user_context, output_text,
                model, prompt_tokens, prompt_cache_creation_tokens, prompt_cached_tokens,
                prompt_audio_tokens, completion_tokens, completion_accepted_prediction_tokens, completion_rejected_prediction_tokens,
                completion_reasoning_tokens, completion_audio_tokens, total_tokens
            )
        VALUES
            (
                ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?
            )
        ",
    )
    .bind(ArticleProcessingOutputId::new())
    .bind(article_id)
    .bind(user_context)
    .bind(output_text)
    .bind(model)
    .bind(usage.prompt_tokens)
    .bind(usage.prompt_tokens_details.as_ref().map(|d| d.cache_creation_tokens))
    .bind(usage.prompt_tokens_details.as_ref().map(|d| d.cached_tokens))
    .bind(usage.prompt_tokens_details.as_ref().map(|d| d.audio_tokens))
    .bind(usage.completion_tokens)
    .bind(usage.completion_tokens_details.as_ref().map(|d| d.accepted_prediction_tokens))
    .bind(usage.completion_tokens_details.as_ref().map(|d| d.rejected_prediction_tokens))
    .bind(usage.completion_tokens_details.as_ref().map(|d| d.reasoning_tokens))
    .bind(usage.completion_tokens_details.as_ref().map(|d| d.audio_tokens))
    .bind(usage.total_tokens);

    query.execute(executor).await
}
