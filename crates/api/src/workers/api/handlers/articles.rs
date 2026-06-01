use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use types::{TrimmedString, nonempty_trimmed_string};

use crate::{
    app::{
        events::{NotificationData, NotificationString},
        state::SharedAppState,
    },
    repos::articles::{self, ArticleErrorReason, ArticleId},
    workers::{
        api::{
            err::{HandlerError, HandlerResult},
            req::{Pagination, ReqJson, ReqPath},
            resp::{Metadata, data, data_with_meta},
        },
        article_processor::{ArticleProcessingRequest, ProcessArticleUserContext},
    },
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_articles(
    State(state): State<SharedAppState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let page_index = pagination.page_index();
    let limit = pagination.limit();

    let data = articles::get_articles(&state.db_pool, page_index, limit).await?;
    let count = articles::count_articles(&state.db_pool).await?;

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
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    let article = articles::get_article_by_id(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    Ok(data(article))
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn delete_article(
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
) -> HandlerResult<impl IntoResponse> {
    articles::delete_article_by_id(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_article_stats(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let article_stats = articles::get_article_stats(&state.db_pool).await?;

    Ok(data(article_stats))
}

#[derive(Debug, serde::Deserialize)]
pub struct ProcessArticleJson {
    context: Option<ProcessArticleUserContext>,
}

#[tracing::instrument(level = "trace", skip(state))]
pub async fn process_article(
    State(state): State<SharedAppState>,
    ReqPath(article_id): ReqPath<ArticleId>,
    ReqJson(json): ReqJson<ProcessArticleJson>,
) -> HandlerResult<impl IntoResponse> {
    articles::mark_article_as_pending(&state.db_pool, article_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("article not found"))?;

    let request = ArticleProcessingRequest {
        article_id,
        context: json.context,
    };

    if let Err(error) = state.article_processing_tx.send(request).await {
        articles::mark_article_as_error(
            &state.db_pool,
            article_id,
            ArticleErrorReason::new(TrimmedString::from(error.to_string())).as_ref(),
        )
        .await?;

        state.app_events_broadcaster.send_notification(
            NotificationData::error(NotificationString(nonempty_trimmed_string!(
                "Failed to process article"
            )))
            .with_description(NotificationString::new(
                "Article was not processed successfully",
            )),
        );

        Err(error.into())
    } else {
        Ok(StatusCode::ACCEPTED)
    }
}
