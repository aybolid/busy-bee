use std::num::NonZeroU32;

use chrono::{DateTime, Utc};
use sqlx::Row;
use types::{NonEmpty, TrimmedString, Url};
use uuid::Uuid;

use crate::{infra::db::DatabaseRow, repos::rss_feeds::RssFeedId};

/// A strongly-typed, globally unique identifier for an [`Article`].
///
/// Backed by a time-ordered `UUIDv7` to improve database insertion performance.
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
pub struct ArticleId(Uuid);

impl std::ops::Deref for ArticleId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleId {
    /// Creates a new `ArticleId` using a time-ordered `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

/// The headline or title of an [`Article`].
///
/// Guaranteed to be a non-empty string with no leading or trailing whitespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleTitle(pub NonEmpty<TrimmedString>);

impl std::fmt::Display for ArticleTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Deref for ArticleTitle {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleTitle {
    /// Attempts to create a new [`ArticleTitle`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The author or attribution line of an [`Article`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleByLine(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleByLine {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleByLine {
    /// Attempts to create a new [`ArticleByLine`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The full HTML or rich-text content of an [`Article`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleContent(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleContent {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleContent {
    /// Attempts to create a new [`ArticleContent`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The stripped, plain-text representation of an [`Article`]'s content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleTextContent(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleTextContent {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleTextContent {
    /// Attempts to create a new [`ArticleTextContent`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }

    /// Calculates the character length of the text content.
    ///
    /// # Panics
    ///
    /// Panics if the length is calculated as zero, which should be impossible given the
    /// [`NonEmpty`] guarantee of the underlying type.
    pub fn char_len(&self) -> NonZeroU32 {
        #[allow(clippy::cast_possible_truncation)]
        NonZeroU32::new(self.chars().count() as u32)
            .expect("article text content should not be empty")
    }
}

/// A short summary or extracted snippet from an [`Article`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleExcerpt(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleExcerpt {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleExcerpt {
    /// Attempts to create a new [`ArticleExcerpt`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The name of the website or publication where the [`Article`] was published.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleSiteName(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleSiteName {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleSiteName {
    /// Attempts to create a new [`ArticleSiteName`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The ISO 639-1 or BCP 47 language code of the [`Article`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleLang(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleLang {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleLang {
    /// Attempts to create a new [`ArticleLang`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// Represents the reading directionality of the article's text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum TextDirection {
    /// Right-to-Left (e.g., Arabic, Hebrew)
    Rtl,
    /// Left-to-Right (e.g., English, Spanish)
    Ltr,
}

/// Error returned when parsing an unrecognized [`TextDirection`] string.
#[derive(Debug, thiserror::Error)]
#[error("unknown text direction: {0}")]
pub struct UnknownTextDirection(String);

impl std::str::FromStr for TextDirection {
    type Err = UnknownTextDirection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rtl" => Ok(Self::Rtl),
            "ltr" => Ok(Self::Ltr),
            s => Err(UnknownTextDirection(s.to_owned())),
        }
    }
}

/// Represents the specific reason why an [`Article`] entered an error state.
///
/// Guaranteed to be a non-empty string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ArticleErrorReason(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ArticleErrorReason {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArticleErrorReason {
    /// Attempts to create a new [`ArticleErrorReason`]. Returns [`None`] if the input is entirely whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

/// The current status of an [`Article`] within the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(tag = "status", content = "error_reason", rename_all = "lowercase")]
pub enum ArticleStatus {
    /// The article has been newly discovered or registered.
    New,
    /// The article is currently queued or actively being processed.
    Pending,
    /// Processing failed. Contains the specific [`ArticleErrorReason`].
    Error(ArticleErrorReason),
    /// The article was successfully processed.
    Processed,
}

/// A thread-safe version of [`dom_smoothie::Article`] with additional invariants enforced.
#[derive(Debug, serde::Serialize)]
pub struct ReadabilityArticle {
    /// The extracted headline or title.
    pub title: ArticleTitle,
    /// The author or publication attribution.
    pub byline: Option<ArticleByLine>,
    /// The full HTML or rich content.
    pub content: ArticleContent,
    /// The stripped, plain-text representation.
    pub text_content: ArticleTextContent,
    /// The character length of the `text_content`.
    pub length: NonZeroU32,
    /// A short summary or snippet.
    pub excerpt: Option<ArticleExcerpt>,
    /// The site or publication name.
    pub site_name: Option<ArticleSiteName>,
    /// The text reading direction.
    pub dir: Option<TextDirection>,
    /// The primary language of the article.
    pub lang: Option<ArticleLang>,
    /// The publication timestamp, if provided by the source.
    pub published_time: Option<DateTime<Utc>>,
    /// The last modified timestamp, if provided by the source.
    pub modified_time: Option<DateTime<Utc>>,
    /// The URL of the primary feature image.
    pub image: Option<Url>,
    /// The URL of the site's favicon.
    pub favicon: Option<Url>,
    /// The canonical URL where the article is hosted.
    pub url: Url,
}

/// The core domain entity representing a scraped and parsed web article.
///
/// This struct holds all metadata, text content, and processing state related
/// to an individual article.
#[derive(Debug, serde::Serialize)]
pub struct Article {
    /// The unique, time-ordered identifier for the article.
    pub id: ArticleId,
    /// The timestamp when the article record was first created.
    pub created_at: DateTime<Utc>,
    /// The timestamp of the last update to the article record.
    pub updated_at: DateTime<Utc>,

    /// The ID of the RSS feed that this article belongs to.
    pub rss_feed_id: RssFeedId,

    /// The current processing state of the article.
    #[serde(flatten)]
    pub status: ArticleStatus,
    #[serde(flatten)]
    pub readability: ReadabilityArticle,
}

/// Custom database decoding logic for [`Article`].
///
/// Evaluates the `status` string and the nullable `error_reason` column
/// to reconstruct the [`ArticleStatus`] enum, enforcing valid state combinations.
impl<'r> sqlx::FromRow<'r, DatabaseRow> for Article {
    fn from_row(row: &'r DatabaseRow) -> Result<Self, sqlx::Error> {
        let raw_status: String = row.try_get("status")?;
        let raw_reason: Option<ArticleErrorReason> = row.try_get("error_reason")?;

        let status = decode_article_status(raw_status, raw_reason)
            .map_err(|error| sqlx::Error::Decode(error.into()))?;

        Ok(Self {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            status,
            rss_feed_id: row.try_get("rss_feed_id")?,
            readability: ReadabilityArticle {
                title: row.try_get("title")?,
                byline: row.try_get("byline")?,
                content: row.try_get("content")?,
                text_content: row.try_get("text_content")?,
                length: row.try_get("length")?,
                excerpt: row.try_get("excerpt")?,
                site_name: row.try_get("site_name")?,
                dir: row.try_get("dir")?,
                lang: row.try_get("lang")?,
                published_time: row.try_get("published_time")?,
                modified_time: row.try_get("modified_time")?,
                image: row.try_get("image")?,
                favicon: row.try_get("favicon")?,
                url: row.try_get("url")?,
            },
        })
    }
}

/// Errors that can occur when decoding an [`ArticleStatus`] from database columns.
#[derive(Debug, thiserror::Error)]
pub enum ArticleStatusDecodeError {
    /// The database contained an unrecognized status string.
    #[error("unknown status: {0}")]
    UnknownStatus(String),
    /// The combination of status string and error reason violated an invariant.
    /// (e.g., an 'error' status without a reason, or a 'processed' status with a reason).
    #[error("invariant broken: status={status} reason={reason:?}")]
    InvariantBroken {
        status: String,
        reason: Option<ArticleErrorReason>,
    },
}

/// Helper function to safely reconstruct an [`ArticleStatus`] and enforce invariants.
fn decode_article_status(
    raw_status: String,
    raw_reason: Option<ArticleErrorReason>,
) -> Result<ArticleStatus, ArticleStatusDecodeError> {
    match (raw_status.as_str(), raw_reason) {
        ("new", None) => Ok(ArticleStatus::New),
        ("pending", None) => Ok(ArticleStatus::Pending),
        ("processed", None) => Ok(ArticleStatus::Processed),
        ("error", Some(reason)) => Ok(ArticleStatus::Error(reason)),

        // Because match arms are evaluated top-to-bottom, reaching this arm
        // guarantees that a known status has an invalid reason combination.
        ("new" | "pending" | "processed" | "error", reason) => {
            Err(ArticleStatusDecodeError::InvariantBroken {
                status: raw_status,
                reason,
            })
        }

        _ => Err(ArticleStatusDecodeError::UnknownStatus(raw_status)),
    }
}

/// Aggregate statistics reflecting the current breakdown of article states.
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct ArticleStats {
    /// The total number of articles in the system.
    pub total: usize,
    /// Articles awaiting processing.
    pub new: usize,
    /// Articles currently being processed.
    pub pending: usize,
    /// Articles that failed processing.
    pub error: usize,
    /// Articles successfully processed.
    pub processed: usize,
}
