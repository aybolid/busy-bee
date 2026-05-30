use std::num::{NonZeroU8, NonZeroU32};

use axum::{extract::State, response::IntoResponse};
use sqlx::error::DatabaseError;
use types::Url;

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds,
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::ReqJson,
        resp::data,
    },
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_rss_feeds(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let feeds = rss_feeds::get_rss_feeds(&state.db_pool).await?;

    Ok(data(feeds))
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateRssFeedJson {
    url: Url,
    max_concurrent_requests: NonZeroU8,
    fetch_interval_seconds: NonZeroU32,
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn create_rss_feed(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<CreateRssFeedJson>,
) -> HandlerResult<impl IntoResponse> {
    let feed = rss_feeds::create_rss_feed(
        &state.db_pool,
        &json.url,
        json.max_concurrent_requests,
        json.fetch_interval_seconds,
    )
    .await
    .map_err(|error| -> HandlerError {
        if error
            .as_database_error()
            .is_some_and(DatabaseError::is_unique_violation)
        {
            HandlerError::validation_with_source(
                "rss feed with the given url already exists",
                "url",
            )
        } else {
            error.into()
        }
    })?;

    Ok(data(feed))
}
