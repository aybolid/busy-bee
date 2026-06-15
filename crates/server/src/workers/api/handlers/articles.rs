use std::num::NonZeroU8;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::state::SharedAppState,
    repos::{
        Pagination, SearchString,
        articles::{
            self, ArticleErrorReason, ArticleId, ArticleIds, ArticleStatusTag, GetArticlesFilters,
        },
        instruction_prompts::InstructionPromptIds,
        rss_feeds::RssFeedId,
        system_prompts::SystemPromptId,
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

/// Contains search query values supported by [`get_articles`] hanlder.
#[derive(Debug, serde::Deserialize)]
pub struct GetArticlesQuery {
    page_index: usize,
    limit: NonZeroU8,

    /// An optional text-based search query.
    query: Option<SearchString>,
    /// An optional identifier to filter articles by their source.
    rss_feed_id: Option<RssFeedId>,
    /// An optional tag to filter articles by their current state.
    status: Option<ArticleStatusTag>,
}

impl GetArticlesQuery {
    pub fn into_structs(self) -> (Pagination, GetArticlesFilters) {
        (
            Pagination {
                page_index: self.page_index,
                limit: self.limit,
            },
            GetArticlesFilters {
                query: self.query,
                rss_feed_id: self.rss_feed_id,
                status: self.status,
            },
        )
    }
}

/// Retrieves a paginated list of articles.
#[tracing::instrument(skip(state))]
pub async fn get_articles(
    State(state): State<SharedAppState>,
    Query(query): Query<GetArticlesQuery>,
) -> HandlerResult<impl IntoResponse> {
    let (pagination, filters) = query.into_structs();

    let data = articles::get_articles(&state.db_pool, pagination, &filters).await?;
    let count = articles::count_articles(&state.db_pool, &filters).await?;

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

/// JSON payload containing data for initiating
/// the processing of an article.
#[derive(Debug, serde::Deserialize)]
pub struct ProcessArticleJson {
    system_prompt_id: SystemPromptId,
    instruction_prompt_ids: Option<InstructionPromptIds>,
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
        system_prompt_id: json.system_prompt_id,
        instruction_prompt_ids: json.instruction_prompt_ids,
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

/// JSON payload containing IDs of articles to delete.
#[derive(Debug, serde::Deserialize)]
pub struct BulkDeleteArticlesJson {
    ids: ArticleIds,
}

/// Bulk deletes articles using their IDs.
pub async fn bulk_delete_articles(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<BulkDeleteArticlesJson>,
) -> HandlerResult<impl IntoResponse> {
    articles::bulk_delete_articles(&state.db_pool, &json.ids).await?;

    Ok(StatusCode::NO_CONTENT)
}
