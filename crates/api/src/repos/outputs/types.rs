use chrono::{DateTime, Utc};
use types::{NonEmpty, TrimmedString};
use uuid::Uuid;

use crate::{
    ai::{Message, ModelName, Usage},
    repos::articles::ArticleId,
    workers::article_processor::ProcessArticleUserContext,
};

/// A unique identifier for an AI-generated output.
///
/// This is a transparent wrapper around a [`Uuid`] (specifically `UUIDv7`).
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
pub struct OutputId(Uuid);

impl std::ops::Deref for OutputId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OutputId {
    /// Generates a new, time-ordered [`OutputId`] using `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

/// A strictly non-empty, trimmed text representing the actual output.
///
/// By wrapping `NonEmpty<TrimmedString>`, it guarantees that empty or whitespace-only
/// outputs cannot be constructed or stored in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OutputText(pub NonEmpty<TrimmedString>);

impl OutputText {
    /// Attempts to create a new [`OutputText`] from a string-like type.
    ///
    /// Returns [`None`] if the input string is empty or contains only whitespace
    /// after being trimmed.
    #[allow(clippy::needless_pass_by_value, dead_code)]
    pub fn new(s: impl ToString) -> Option<Self> {
        NonEmpty::new(TrimmedString::from(s.to_string())).map(Self)
    }
}

impl From<Message> for OutputText {
    /// Converts an AI [`Message`] directly into [`OutputText`].
    fn from(value: Message) -> Self {
        Self(value.0)
    }
}

impl std::ops::Deref for OutputText {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a complete, persisted AI generation record.
///
/// This struct holds the generated text alongside its associated metadata,
/// such as database timestamps, the related article, the context in which it was requested,
/// and AI usage telemetry.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Output {
    /// The unique database identifier for this output.
    pub id: OutputId,
    /// The UTC timestamp of when this output was first created.
    pub created_at: DateTime<Utc>,
    /// The UTC timestamp of when this output was last updated.
    pub updated_at: DateTime<Utc>,
    /// The ID of the article this output is associated with, if applicable.
    pub article_id: Option<ArticleId>,
    /// The context parameters provided by the user when requesting the article processing.
    pub user_context: Option<ProcessArticleUserContext>,
    /// The core generated text content.
    pub text: OutputText,
    /// The specific AI model used to generate this output.
    pub model: ModelName,
    /// Token usage metrics for the AI generation, stored as a JSON column in the database.
    #[sqlx(json)]
    pub usage: Usage,
}
