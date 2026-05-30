use std::num::{NonZeroU8, NonZeroU32};

use chrono::{DateTime, Utc};
use sqlx::Row;
use types::{NonEmpty, TrimmedString, Url};
use uuid::Uuid;

use crate::infra::db::{DatabaseExecutor, DatabaseRow};

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
pub struct RssFeedId(Uuid);

impl RssFeedId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "status", content = "error_reason", rename_all = "lowercase")]
pub enum RssFeedStatus {
    Healthy,
    Error(RssFeedErrorReason),
}

pub type RssFeedErrorReason = NonEmpty<TrimmedString>;

#[derive(Debug, serde::Serialize)]
pub struct RssFeed {
    pub id: RssFeedId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[serde(flatten)]
    pub status: RssFeedStatus,

    pub url: Url,
    pub max_concurrent_requests: NonZeroU8,
    pub fetch_interval_seconds: NonZeroU32,
}

impl<'r> sqlx::FromRow<'r, DatabaseRow> for RssFeed {
    fn from_row(row: &'r DatabaseRow) -> Result<Self, sqlx::Error> {
        let raw_status: String = row.try_get("status")?;

        let raw_reason: Option<RssFeedErrorReason> = row.try_get("error_reason")?;

        let status = match raw_status.as_str() {
            "healthy" => {
                if raw_reason.is_some() {
                    return Err(sqlx::Error::Decode(
                        "invariant broken: 'healthy' feed cannot have an error_reason".into(),
                    ));
                }
                RssFeedStatus::Healthy
            }
            "error" => {
                let reason = raw_reason.ok_or_else(|| {
                    sqlx::Error::Decode(
                        "invariant broken: 'error' feed must have an error_reason".into(),
                    )
                })?;
                RssFeedStatus::Error(reason)
            }
            _ => {
                return Err(sqlx::Error::Decode(
                    format!("unknown status: {raw_status}").into(),
                ));
            }
        };

        Ok(Self {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            status,
            url: row.try_get("url")?,
            max_concurrent_requests: row.try_get("max_concurrent_requests")?,
            fetch_interval_seconds: row.try_get("fetch_interval_seconds")?,
        })
    }
}

#[tracing::instrument(level = "trace", skip_all, ret, err(Debug))]
pub async fn get_rss_feeds<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<Vec<RssFeed>> {
    let query = sqlx::query_as("SELECT * FROM rss_feeds ORDER BY created_at;");

    query.fetch_all(executor).await
}

#[tracing::instrument(level = "trace", skip_all, ret, err(Debug))]
pub async fn create_rss_feed<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &Url,
    max_concurrent_requests: NonZeroU8,
    fetch_interval_seconds: NonZeroU32,
) -> sqlx::Result<RssFeed> {
    let query = sqlx::query_as(
        "
        INSERT INTO rss_feeds
            (
                id, url, max_concurrent_requests,
                fetch_interval_seconds, status
            )
        VALUES
            (
                ?, ?, ?,
                ?, ?
            )
        RETURNING *;
        ",
    )
    .bind(RssFeedId::new())
    .bind(url)
    .bind(max_concurrent_requests)
    .bind(fetch_interval_seconds)
    .bind("healthy");

    query.fetch_one(executor).await
}
