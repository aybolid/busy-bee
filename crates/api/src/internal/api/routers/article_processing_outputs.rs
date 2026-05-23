use axum::{Router, routing::get};

use crate::internal::api::{handlers::article_processing_outputs, state::SharedApiState};

pub fn router() -> Router<SharedApiState> {
    tracing::info!("register /article_processing_outputs router");

    let router = Router::new().route(
        "/",
        get(article_processing_outputs::get_article_processing_outputs),
    );

    Router::new().nest("/article_processing_outputs", router)
}
