use axum::{extract::State, response::IntoResponse};
use types::Url;

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds,
    workers::api::{err::HandlerResult, req::ReqJson, resp::data},
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_rss_feeds(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let feeds = rss_feeds::get_rss_feeds(state.db_pool()).await?;

    Ok(data(feeds))
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateRssFeedJson {
    url: Url,
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn create_rss_feed(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<CreateRssFeedJson>,
) -> HandlerResult<impl IntoResponse> {
    let feed = rss_feeds::create_rss_feed(state.db_pool(), &json.url, 5, 300).await?;

    Ok(data(feed))
}
