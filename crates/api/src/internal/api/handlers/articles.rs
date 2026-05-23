use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::internal::{
    api::{
        err::{HandlerError, HandlerResult},
        req::{Pagination, ReqJson, ReqPath},
        resp::{Metadata, data, data_with_meta},
        state::SharedApiState,
    },
    repos::articles::{self, ArticleId},
    workers::{
        article_processor::{AdditionalContext, ArticleDeliveryPayload},
        publisher::PublisherCommand,
    },
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_articles(
    State(state): State<SharedApiState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let page_index = pagination.page_index();
    let limit = pagination.limit();

    let data = articles::get_articles(state.db_pool(), page_index, limit).await?;
    let count = articles::count_articles(state.db_pool()).await?;

    Ok(data_with_meta(
        data,
        Metadata::Pagination {
            page_index,
            limit,
            total_pages: count.div_ceil(usize::from(limit.get())),
            total: count,
        },
    ))
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

#[tracing::instrument(level = "trace", skip(state))]
pub async fn delete_article(
    State(state): State<SharedApiState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    articles::delete_article_by_id(state.db_pool(), article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_article_stats(
    State(state): State<SharedApiState>,
) -> HandlerResult<impl IntoResponse> {
    let article_stats = articles::get_article_stats(state.db_pool()).await?;

    Ok(data(article_stats))
}

#[derive(Debug, serde::Deserialize)]
pub struct ProcessArticleJson {
    context: Option<AdditionalContext>,
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn process_article(
    State(state): State<SharedApiState>,
    ReqPath(article_id): ReqPath<ArticleId>,
    ReqJson(json): ReqJson<ProcessArticleJson>,
) -> HandlerResult<impl IntoResponse> {
    articles::mark_article_as_pending(state.db_pool(), article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    let command = PublisherCommand::ProcessArticle(ArticleDeliveryPayload {
        article_id,
        context: json.context,
    });

    if let Err(error) = state.amqp_tx().send(command).await {
        articles::mark_article_as_error(state.db_pool(), article_id).await?;
        Err(error.into())
    } else {
        Ok(StatusCode::ACCEPTED)
    }
}
