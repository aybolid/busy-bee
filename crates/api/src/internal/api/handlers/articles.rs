use axum::{extract::State, response::IntoResponse};

use crate::internal::{
    api::{
        err::{HandlerError, HandlerResult},
        req::ReqPath,
        resp::data,
        state::SharedApiState,
    },
    repos::articles::{self, ArticleId},
};

#[tracing::instrument(level = "trace", skip_all)]
pub async fn get_articles(State(state): State<SharedApiState>) -> HandlerResult<impl IntoResponse> {
    let articles = articles::get_articles(state.db_pool()).await?;
    Ok(data(articles))
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_article(
    State(state): State<SharedApiState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    let article = articles::get_article_by_id(state.db_pool(), article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;
    Ok(data(article))
}
