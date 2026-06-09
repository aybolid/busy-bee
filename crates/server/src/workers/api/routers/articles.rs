use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{app::state::SharedAppState, workers::api::handlers::articles};

/// Creates a [`Router`] that handles `/articles` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", get(articles::get_articles))
        .route("/stats", get(articles::get_article_stats))
        .nest(
            "/{article_id}",
            Router::new()
                .route("/", get(articles::get_article))
                .route("/", delete(articles::delete_article))
                .route("/process", post(articles::process_article)),
        )
        .nest(
            "/bulk",
            Router::new().route("/delete", post(articles::bulk_delete_articles)),
        );

    Router::new().nest("/articles", router)
}
