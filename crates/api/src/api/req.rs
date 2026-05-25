#![allow(dead_code, clippy::match_same_arms)]

use std::num::NonZeroU8;

use axum::{
    extract::{
        FromRequest, FromRequestParts, Request,
        path::ErrorKind,
        rejection::{JsonRejection, PathRejection},
    },
    http::StatusCode,
};
use serde::de::DeserializeOwned;

use crate::api::err::HandlerError;

/// Axum extractor for path parameters that maps rejections to [`HandlerError`].
pub struct ReqPath<T>(pub T);

impl<S: Send + Sync, T: DeserializeOwned + Send> FromRequestParts<S> for ReqPath<T> {
    type Rejection = HandlerError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(match rejection {
                PathRejection::FailedToDeserializePathParams(err) => {
                    let kind = err.into_kind();

                    match &kind {
                        ErrorKind::WrongNumberOfParameters { .. } => HandlerError::validation(kind),
                        ErrorKind::ParseErrorAtKey { key, .. } => {
                            HandlerError::validation_with_source(kind.to_string(), key)
                        }
                        ErrorKind::ParseErrorAtIndex { index, .. } => {
                            HandlerError::validation_with_source(kind.to_string(), index)
                        }
                        ErrorKind::ParseError { .. } => HandlerError::validation(kind),
                        ErrorKind::InvalidUtf8InPathParam { key } => {
                            HandlerError::validation_with_source(kind.to_string(), key)
                        }
                        ErrorKind::UnsupportedType { .. } => {
                            HandlerError::message(StatusCode::INTERNAL_SERVER_ERROR, kind)
                        }
                        ErrorKind::Message(message) => HandlerError::validation(message),
                        ErrorKind::DeserializeError { key, message, .. } => {
                            HandlerError::validation_with_source(message, key)
                        }
                        _ => HandlerError::message(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("unhandled path deserialization error: {kind}"),
                        ),
                    }
                }
                PathRejection::MissingPathParams(err) => {
                    HandlerError::message(StatusCode::INTERNAL_SERVER_ERROR, err)
                }
                _ => HandlerError::message(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("unhandled path rejection: {rejection}"),
                ),
            }),
        }
    }
}

/// Axum extractor for JSON request bodies that maps rejections to [`HandlerError`].
pub struct ReqJson<T>(pub T);

impl<S: Send + Sync, T> FromRequest<S> for ReqJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = HandlerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(match rejection {
                JsonRejection::MissingJsonContentType(err) => {
                    HandlerError::message(StatusCode::UNSUPPORTED_MEDIA_TYPE, err)
                }
                JsonRejection::BytesRejection(err) => {
                    HandlerError::message(StatusCode::BAD_REQUEST, err)
                }
                JsonRejection::JsonDataError(err) => downcast_error::<DeserializeJsonError>(&err)
                    .map_or_else(
                        || HandlerError::validation(err.to_string()),
                        |path_err| {
                            HandlerError::validation_with_source(path_err.inner(), path_err.path())
                        },
                    ),
                JsonRejection::JsonSyntaxError(err) => {
                    HandlerError::message(StatusCode::BAD_REQUEST, err)
                }
                _ => HandlerError::message(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("unhandled json rejection: {rejection}"),
                ),
            }),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct Pagination {
    page_index: usize,
    limit: NonZeroU8,
}

impl Pagination {
    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> NonZeroU8 {
        self.limit
    }
}

/// Helper type alias for dynamic errors.
type SomeError<'a> = &'a (dyn std::error::Error + 'static);
type DeserializeJsonError = serde_path_to_error::Error<serde_json::Error>;

/// Downcasts a dynamic error to a specific type by traversing the error chain.
fn downcast_error<T: std::error::Error + 'static>(err: SomeError<'_>) -> Option<&T> {
    let mut current = Some(err);

    while let Some(e) = current {
        if let Some(specific) = e.downcast_ref::<T>() {
            return Some(specific);
        }
        current = e.source();
    }

    None
}
