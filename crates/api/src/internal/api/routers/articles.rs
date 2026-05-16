use axum::{Router, routing::get};

use crate::internal::api::{handlers::articles, state::SharedApiState};

pub fn router() -> Router<SharedApiState> {
    tracing::info!("register /articles router");

    let router = Router::new()
        .route("/", get(articles::get_articles))
        .route("/{article_id}", get(articles::get_article));

    Router::new().nest("/articles", router)
}
