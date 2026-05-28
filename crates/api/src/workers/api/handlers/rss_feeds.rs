use axum::{extract::State, response::IntoResponse};

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds,
    workers::api::{err::HandlerResult, resp::data},
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_rss_feeds(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let feeds = rss_feeds::get_rss_feeds(state.db_pool()).await?;

    Ok(data(feeds))
}
