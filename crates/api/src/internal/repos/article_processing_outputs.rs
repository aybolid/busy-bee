use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::internal::{
    infra::db::{DatabaseExecutor, DatabaseQueryResult},
    repos::{
        articles::ArticleId,
        types::{length::NonEmpty, trimmed_string::TrimmedString},
    },
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

#[tracing::instrument(level = "trace", skip_all, ret, err)]
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
