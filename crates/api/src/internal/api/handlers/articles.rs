use axum::{Json, extract::State, response::IntoResponse};

use crate::internal::{api::state::SharedApiState, repos::articles};

#[tracing::instrument(level = "trace", skip_all)]
pub async fn get_articles(State(state): State<SharedApiState>) -> impl IntoResponse {
    let articles = articles::get_articles(state.db_pool()).await.unwrap();
    Json(articles)
}
