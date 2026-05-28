use axum::{Router, routing::get};

use crate::{app::state::SharedAppState, workers::api::handlers::rss_feeds};

pub fn router() -> Router<SharedAppState> {
    tracing::info!("register /rss_feeds router");

    let router = Router::new().route("/", get(rss_feeds::get_rss_feeds));

    Router::new().nest("/rss_feeds", router)
}
