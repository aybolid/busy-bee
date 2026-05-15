use axum::{extract::State, response::IntoResponse};

use crate::internal::{
    api::{err::HandlerResult, resp::data, state::SharedApiState},
    repos::articles,
};

#[tracing::instrument(level = "trace", skip_all)]
pub async fn get_articles(State(state): State<SharedApiState>) -> HandlerResult<impl IntoResponse> {
    let articles = articles::get_articles(state.db_pool()).await?;
    Ok(data(articles))
}
