use axum::{Router, routing::get};

use crate::{api::handlers::article_processing_outputs, app::state::SharedAppState};

pub fn router() -> Router<SharedAppState> {
    tracing::info!("register /article_processing_outputs router");

    let router = Router::new()
        .route(
            "/",
            get(article_processing_outputs::get_article_processing_outputs),
        )
        .route(
            "/{output_id}",
            get(article_processing_outputs::get_article_processing_output),
        );

    Router::new().nest("/article_processing_outputs", router)
}
