use tokio::sync::mpsc;
use types::{NonEmptyMaxLength, TrimmedString};

use crate::repos::articles::ArticleId;

/// Additional user-provided context used during article processing.
///
/// This is a strongly-typed wrapper around a validated string that ensures the
/// context is not empty, is stripped of leading/trailing whitespace, and does
/// not exceed 500 characters.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde ::Deserialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ProcessingUserContext(pub NonEmptyMaxLength<500, TrimmedString>);

impl std::ops::Deref for ProcessingUserContext {
    type Target = NonEmptyMaxLength<500, TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ProcessingUserContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A request payload sent to the background processor.
///
/// Contains the identifier of the article to be processed and any optional
/// context provided by the user to guide the processing logic.
#[derive(Debug)]
pub struct ProcessingRequest {
    /// The unique identifier of the article to process.
    pub article_id: ArticleId,
    /// Optional instructions or context from the user.
    pub context: Option<ProcessingUserContext>,
}

/// The transmitting end of the processing request channel.
///
/// This newtype wraps a [`mpsc::Sender`] to provide a domain-specific
/// type for dispatching [`ProcessingRequest`] messages.
pub struct ProcessingRequestSender {
    inner: mpsc::Sender<ProcessingRequest>,
}

impl std::ops::Deref for ProcessingRequestSender {
    type Target = mpsc::Sender<ProcessingRequest>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// The receiving end of the processing request channel.
///
/// This newtype wraps a [`mpsc::Receiver`] to provide a domain-specific
/// type for consuming [`ProcessingRequest`] messages in the background worker.
pub struct ProcessingRequestReceiver {
    inner: mpsc::Receiver<ProcessingRequest>,
}

impl std::ops::Deref for ProcessingRequestReceiver {
    type Target = mpsc::Receiver<ProcessingRequest>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for ProcessingRequestReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Creates a bounded, multi-producer single-consumer (mpsc) channel for processing requests.
pub fn create_processing_requests_channel() -> (ProcessingRequestSender, ProcessingRequestReceiver)
{
    let pair = mpsc::channel(100);
    (
        ProcessingRequestSender { inner: pair.0 },
        ProcessingRequestReceiver { inner: pair.1 },
    )
}
