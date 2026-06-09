use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

use crate::workers::api::err::{HandlerError, HandlerResult};

pub mod articles;
pub mod notifications;
pub mod outputs;
pub mod rss_feeds;
pub mod system_prompts;

/// 404 fallback handler.
#[allow(clippy::unused_async)]
pub async fn not_found() -> HandlerResult<()> {
    tracing::warn!("route not found");
    Err(HandlerError::not_found("route not found"))
}

/// Route panic handler
#[allow(clippy::needless_pass_by_value)]
pub fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let panic: String = err.downcast_ref().map_or_else(
        || {
            err.downcast_ref::<&str>()
                .map_or_else(|| "unknown panic message".to_owned(), ToString::to_string)
        },
        Clone::clone,
    );

    tracing::error!(panic);

    HandlerError::obfuscated(
        StatusCode::INTERNAL_SERVER_ERROR,
        "some unexpected error has occurred",
        panic,
    )
    .into_response()
}
