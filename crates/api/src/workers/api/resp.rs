#![allow(dead_code, clippy::needless_pass_by_value)]

use std::num::NonZeroU8;

use axum::{Json, http::StatusCode, response::IntoResponse};

/// Creates a new [`HandlerOk`] with a simple message response.
///
/// This is a convenience function for returning plain text messages
/// from API handlers, such as success confirmations or status updates.
pub fn message(s: impl ToString) -> HandlerOk<MessagePayload> {
    HandlerOk::new_ok(MessagePayload {
        message: s.to_string(),
    })
}

/// Creates a new [`HandlerOk`] with a generic data payload.
///
/// This is a convenience function for returning structured data from
/// API handlers. The data is wrapped in a JSON object under the `data` key.
pub const fn data<T>(data: T) -> HandlerOk<DataPayload<T>> {
    HandlerOk::new_ok(DataPayload { data })
}

/// Creates a new [`HandlerOk`] with a generic data payload and some metadata.
pub const fn data_with_meta<T>(data: T, meta: Metadata) -> HandlerOk<DataWithMetaPayload<T>> {
    HandlerOk::new_ok(DataWithMetaPayload { data, meta })
}

/// A generic successful response wrapper for Axum handlers.
///
/// This type wraps any payload and converts it into an Axum response
/// with a configurable HTTP status code. It uses JSON serialization
/// via [`serde::Serialize`] and flattens the payload into the response body.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct HandlerOk<T> {
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(flatten)]
    payload: T,
}

impl<T> HandlerOk<T> {
    /// Creates a new [`HandlerOk`] with HTTP 200 OK status.
    pub const fn new_ok(payload: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            payload,
        }
    }

    /// Sets a custom HTTP status code for the response.
    ///
    /// This allows overriding the default 200 OK status with any
    /// other valid HTTP status code (e.g., 201 Created, 202 Accepted).
    pub const fn status_code(mut self, code: StatusCode) -> Self {
        self.status_code = code;
        self
    }
}

impl<T: serde::Serialize> IntoResponse for HandlerOk<T> {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}

/// A simple message response payload.
///
/// This struct is used with the [`message`] function to return
/// plain text messages from API endpoints.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MessagePayload {
    message: String,
}

/// A generic data response payload.
///
/// This struct is used with the [`data`] function to return
/// structured data from API endpoints. The data is placed under
/// a `data` key in the JSON response.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataPayload<T> {
    data: T,
}

/// A generic data response payload that includes some metadata.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataWithMetaPayload<T> {
    data: T,
    meta: Metadata,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Metadata {
    Pagination {
        page_index: usize,
        limit: NonZeroU8,
        total_pages: usize,
        total: usize,
    },
}
