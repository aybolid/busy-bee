use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::state::SharedAppState,
    repos::{
        Pagination,
        articles::{self, ArticleErrorReason, ArticleId},
    },
    workers::{
        api::{
            err::{HandlerError, HandlerResult},
            req::{ReqJson, ReqPath},
            resp::{Metadata, data, data_with_meta},
        },
        article_processing::{ProcessingRequest, ProcessingUserContext},
    },
};

/// Retrieves a paginated list of articles.
#[tracing::instrument(skip(state))]
pub async fn get_articles(
    State(state): State<SharedAppState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let data = articles::get_articles(&state.db_pool, pagination).await?;
    let count = articles::count_articles(&state.db_pool).await?;

    Ok(data_with_meta(
        data,
        Metadata::pagination(pagination, count),
    ))
}

/// Retrieves a specific article by its unique ID.
#[tracing::instrument(skip(state))]
pub async fn get_article(
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    let article = articles::get_article_by_id(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    Ok(data(article))
}

/// Deletes a specific article by its unique ID.
#[tracing::instrument(skip(state))]
pub async fn delete_article(
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    articles::delete_article_by_id(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Retrieves aggregate statistics across all articles.
#[tracing::instrument(skip(state))]
pub async fn get_article_stats(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let article_stats = articles::get_article_stats(&state.db_pool).await?;

    Ok(data(article_stats))
}

/// JSON payload containing optional user context for initiating
/// the processing of an article.
#[derive(Debug, serde::Deserialize)]
pub struct ProcessArticleJson {
    context: Option<ProcessingUserContext>,
}

/// Enqueues a specific article for asynchronous processing.
///
/// This handler marks the article as pending in the database and dispatches
/// a request to the background processing worker. If the dispatch fails,
/// the article is updated with an error state.
#[tracing::instrument(skip(state))]
pub async fn process_article(
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
    ReqJson(json): ReqJson<ProcessArticleJson>,
) -> HandlerResult<impl IntoResponse> {
    articles::mark_article_as_pending(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    let request = ProcessingRequest {
        article_id,
        context: json.context,
    };

    if let Err(error) = state.article_processing_tx.send(request).await {
        tracing::error!(%error);

        articles::mark_article_as_error(
            &state.db_pool,
            article_id,
            &ArticleErrorReason::from(&error),
        )
        .await?;

        Err(error.into())
    } else {
        Ok(StatusCode::ACCEPTED)
    }
}
