use std::num::{NonZeroU8, NonZeroU32};

use chrono::{DateTime, Utc};
use sqlx::Row;
use types::{NonEmpty, TrimmedString, Url};
use uuid::Uuid;

use crate::infra::db::DatabaseRow;

/// A strongly-typed, globally unique identifier for an [`RssFeed`].
///
/// Backed by a `UUIDv7` to ensure that identifiers are time-ordered,
/// which improves database insertion performance and indexing.
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
    /// Creates a new [`RssFeedId`] using a time-ordered `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

/// Represents the specific reason why an [`RssFeed`] entered an error state.
///
/// This wrapper type guarantees that the error reason is a valid,
/// non-empty, and trimmed string, preventing empty or whitespace-only error messages.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct RssFeedErrorReason(pub NonEmpty<TrimmedString>);

impl RssFeedErrorReason {
    /// Attempts to create a new [`RssFeedErrorReason`].
    ///
    /// Returns [`None`] if the provided string is empty or consists entirely of whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(reason: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(reason.to_string())).map(Self)
    }
}

impl std::ops::Deref for RssFeedErrorReason {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The current operational state of an [`RssFeed`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "status", content = "error_reason", rename_all = "lowercase")]
pub enum RssFeedStatus {
    /// The feed has been registered but has not yet been successfully fetched or processed.
    New,
    /// The feed is active, being polled regularly, and functioning without issues.
    Healthy,
    /// The feed encountered a critical error during a fetch or parsing operation.
    /// Contains the specific [`RssFeedErrorReason`] detailing the failure.
    Error(RssFeedErrorReason),
}

/// The core domain entity representing a registered RSS feed.
///
/// This struct holds the feed's metadata, its current processing status,
/// and configuration rules for how the system should interact with it.
#[derive(Debug, serde::Serialize)]
pub struct RssFeed {
    /// The unique, time-ordered identifier for the feed.
    pub id: RssFeedId,
    /// The timestamp of when the feed was first registered in the system.
    pub created_at: DateTime<Utc>,
    /// The timestamp of when the feed's metadata or status was last modified.
    pub updated_at: DateTime<Utc>,

    /// The current operational state of the feed.
    ///
    /// Flattened during serialization so `status` and `error_reason` appear at the top level.
    #[serde(flatten)]
    pub status: RssFeedStatus,

    /// The target URL from which to fetch the RSS feed data.
    pub url: Url,
    /// The maximum number of concurrent HTTP requests allowed for this specific feed.
    pub max_concurrent_requests: NonZeroU8,
    /// The minimum amount of time, in seconds, to wait between fetch attempts.
    pub fetch_interval_seconds: NonZeroU32,
}

/// Custom database decoding logic for [`RssFeed`].
///
/// This implementation manually constructs the [`RssFeedStatus`] by inspecting both the
/// `status` string column and the nullable `error_reason` column.
///
/// # Invariants Enforced
/// * A `new` or `healthy` status **must not** have an associated `error_reason`.
/// * An `error` status **must** have an associated `error_reason`.
impl<'r> sqlx::FromRow<'r, DatabaseRow> for RssFeed {
    fn from_row(row: &'r DatabaseRow) -> Result<Self, sqlx::Error> {
        let raw_status: String = row.try_get("status")?;
        let raw_reason: Option<RssFeedErrorReason> = row.try_get("error_reason")?;

        let status = decode_rss_feed_status(raw_status, raw_reason)
            .map_err(|error| sqlx::Error::Decode(error.into()))?;

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

#[derive(Debug, thiserror::Error)]
pub enum RssFeedStatusDecodeError {
    #[error("unknown status: {0}")]
    UnknownStatus(String),
    #[error("invariant broken: status={status} reason={reason:?}")]
    InvariantBroken {
        status: String,
        reason: Option<RssFeedErrorReason>,
    },
}

fn decode_rss_feed_status(
    raw_status: String,
    raw_reason: Option<RssFeedErrorReason>,
) -> Result<RssFeedStatus, RssFeedStatusDecodeError> {
    match (raw_status.as_str(), raw_reason) {
        ("new", None) => Ok(RssFeedStatus::New),
        ("healthy", None) => Ok(RssFeedStatus::Healthy),
        ("error", Some(reason)) => Ok(RssFeedStatus::Error(reason)),

        // Because match arms are evaluated top-to-bottom, reaching this arm
        // guarantees that a known status has an invalid reason combination.
        ("new" | "healthy" | "error", reason) => Err(RssFeedStatusDecodeError::InvariantBroken {
            status: raw_status,
            reason,
        }),

        _ => Err(RssFeedStatusDecodeError::UnknownStatus(raw_status)),
    }
}
