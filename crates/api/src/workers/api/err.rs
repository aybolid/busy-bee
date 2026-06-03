#![allow(dead_code, clippy::needless_pass_by_value)]

use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use uuid::Uuid;

pub type HandlerResult<T> = Result<T, HandlerError>;

/// A unified error type for API route handlers.
///
/// `HandlerError` implements [`IntoResponse`], allowing it to be returned directly
/// from axum handlers. It guarantees a consistent JSON structure for the frontend
/// and safely handles internal server errors by obfuscating sensitive data.
#[derive(Debug, serde::Serialize)]
pub struct HandlerError {
    #[serde(skip)]
    status_code: StatusCode,
    message: String,
    #[serde(flatten)]
    payload: ErrorPayload,
    timestamp: DateTime<Utc>,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        if matches!(self.payload, ErrorPayload::Obfuscated { .. }) {
            tracing::error!(error = ?self);
        }
        (self.status_code, Json(self)).into_response()
    }
}

impl HandlerError {
    /// Creates a new `HandlerError` with a custom payload.
    pub fn new(status_code: StatusCode, message: impl ToString, payload: ErrorPayload) -> Self {
        Self {
            status_code,
            message: message.to_string(),
            payload,
            timestamp: Utc::now(),
        }
    }

    /// Creates a standard, safe error without any additional payload data.
    pub fn message(status_code: StatusCode, message: impl ToString) -> Self {
        Self::new(status_code, message, ErrorPayload::JustMessage {})
    }

    /// Convenience method for generating a `404 Not Found` error.
    pub fn not_found(message: impl ToString) -> Self {
        Self::message(StatusCode::NOT_FOUND, message)
    }

    /// Creates a validation error.
    pub fn validation(message: impl ToString) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            message,
            ErrorPayload::Validation { source: None },
        )
    }

    /// Creates a validation error for a specific source (e.g., a field name or header).
    pub fn validation_with_source(message: impl ToString, source: impl ToString) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            message,
            ErrorPayload::Validation {
                source: Some(source.to_string()),
            },
        )
    }

    /// Creates a secure error that hides the true cause from the client.
    ///
    /// The `real_cause` is logged alongside a generated `trace_id`.
    /// The client only receives the `trace_id` and the safe `message`.
    pub fn obfuscated(
        status_code: StatusCode,
        message: impl ToString,
        real_cause: impl ToString,
    ) -> Self {
        Self::new(
            status_code,
            message,
            ErrorPayload::Obfuscated {
                real_cause: real_cause.to_string(),
                trace_id: Uuid::now_v7(),
            },
        )
    }
}

/// The specific data associated with an error response.
#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ErrorPayload {
    /// A simple error containing no extra data.
    JustMessage {},
    /// An error where details are hidden from the client.
    Obfuscated {
        /// The underlying cause of the error (e.g., a database constraint violation).
        ///
        /// This field is skipped during serialization and never sent to the client.
        #[serde(skip)] // do not expose real cause to client
        real_cause: String,
        /// A unique identifier allowing to find the `real_cause` in server logs.
        trace_id: Uuid,
    },
    /// A validation failure resulting from invalid client input.
    ///
    /// # Architecture Note
    ///
    /// Because this API uses `serde` to strictly parse data rather than validating
    /// mutable structs after the fact, the deserializer will abort on the very first
    /// malformed field. Consequently, this variant only returns a single `source` error
    /// per request, rather than an array of all possible form errors.
    Validation {
        /// The specific field, header, or property that failed to parse (e.g., "`email_address`").
        source: Option<String>,
    },
}

impl From<sqlx::Error> for HandlerError {
    fn from(value: sqlx::Error) -> Self {
        Self::obfuscated(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database operation failed",
            value,
        )
    }
}

impl<T> From<mpsc::error::SendError<T>> for HandlerError {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        Self::obfuscated(
            StatusCode::INTERNAL_SERVER_ERROR,
            "mpsc send operation failed",
            value,
        )
    }
}
