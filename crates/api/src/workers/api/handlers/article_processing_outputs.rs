use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    app::state::SharedAppState,
    repos::article_processing_outputs::{self, ArticleProcessingOutputId},
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{Pagination, ReqPath},
        resp::{Metadata, data, data_with_meta},
    },
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_article_processing_outputs(
    State(state): State<SharedAppState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let page_index = pagination.page_index();
    let limit = pagination.limit();

    let data = article_processing_outputs::get_article_processing_outputs(
        &state.db_pool,
        page_index,
        limit,
    )
    .await?;
    let count =
        article_processing_outputs::count_article_processing_outputs(&state.db_pool).await?;

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
pub async fn get_article_processing_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<ArticleProcessingOutputId>,
) -> HandlerResult<impl IntoResponse> {
    let output =
        article_processing_outputs::get_article_processing_output_by_id(&state.db_pool, output_id)
            .await?
            .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(data(output))
}
