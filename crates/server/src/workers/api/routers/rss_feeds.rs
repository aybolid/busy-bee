use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{app::state::SharedAppState, workers::api::handlers::rss_feeds};

/// Creates a [`Router`] that handles `/rss_feeds` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", get(rss_feeds::get_rss_feeds))
        .route("/", post(rss_feeds::create_rss_feed))
        .nest(
            "/{rss_feed_id}",
            Router::new().route("/", delete(rss_feeds::delete_rss_feed)),
        );

    Router::new().nest("/rss_feeds", router)
}
