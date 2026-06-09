use chrono::{DateTime, Utc};
use types::{NonEmpty, NonEmptyMaxLength, TrimmedString};
use uuid::Uuid;

use crate::{ai::Message, repos::VersionNumber};

/// A strongly-typed, globally unique identifier for an [`SystemPrompt`].
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
pub struct SystemPromptId(Uuid);

impl std::ops::Deref for SystemPromptId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SystemPromptId {
    /// Creates a new [`SystemPromptId`] using a time-ordered `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

/// A strongly-typed, user-friendly prompt title.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct SystemPromptTitle(pub NonEmptyMaxLength<{ u8::MAX as usize }, TrimmedString>);

impl std::ops::Deref for SystemPromptTitle {
    type Target = NonEmptyMaxLength<{ u8::MAX as usize }, TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A strongly-typed markdown content of a prompt.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct SystemPromptText(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for SystemPromptText {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The core domain entity representing a system prompt used for LLM conversations.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SystemPrompt {
    /// The unique, time-ordered identifier for the prompt.
    pub id: SystemPromptId,
    /// The timestamp of when the prompt was first registered in the system.
    pub created_at: DateTime<Utc>,
    /// The timestamp of when the prompt's data was last modified.
    pub updated_at: DateTime<Utc>,
    /// The prompt title.
    pub title: SystemPromptTitle,
    /// The prompt markdown content.
    pub text: SystemPromptText,
    /// The current prompt version number.
    pub version: VersionNumber,
}

impl From<SystemPrompt> for Message {
    fn from(value: SystemPrompt) -> Self {
        Self(value.text.0)
    }
}
