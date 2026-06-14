use chrono::{DateTime, Utc};
use types::{NonEmptyMaxLength, TrimmedString};
use uuid::Uuid;

use crate::repos::VersionNumber;

/// A strongly-typed, globally unique identifier for an [`InstructionPrompt`].
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
pub struct InstructionPromptId(Uuid);

impl std::ops::Deref for InstructionPromptId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InstructionPromptId {
    /// Creates a new [`InstructionPromptId`] using a time-ordered `UUIDv7`.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

/// A strongly-typed, user-friendly instruction prompt title.
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
pub struct InstructionPromptTitle(pub NonEmptyMaxLength<{ u8::MAX as usize }, TrimmedString>);

impl std::ops::Deref for InstructionPromptTitle {
    type Target = NonEmptyMaxLength<{ u8::MAX as usize }, TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A strongly-typed content of a prompt.
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
pub struct InstructionPromptText(pub NonEmptyMaxLength<500, TrimmedString>);

impl std::ops::Deref for InstructionPromptText {
    type Target = NonEmptyMaxLength<500, TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The core domain entity representing a instruction prompt used for LLM conversations.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct InstructionPrompt {
    /// The unique, time-ordered identifier for the prompt.
    pub id: InstructionPromptId,
    /// The timestamp of when the prompt was first registered in the system.
    pub created_at: DateTime<Utc>,
    /// The timestamp of when the prompt's data was last modified.
    pub updated_at: DateTime<Utc>,
    /// The prompt title.
    pub title: InstructionPromptTitle,
    /// The prompt content.
    pub text: InstructionPromptText,
    /// The current prompt version number.
    pub version: VersionNumber,
}
