use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::internal::{
    api::{
        err::HandlerResult,
        req::Pagination,
        resp::{Metadata, data_with_meta},
        state::SharedApiState,
    },
    repos::article_processing_outputs,
};

#[tracing::instrument(level = "trace", skip(state))]
pub async fn get_article_processing_outputs(
    State(state): State<SharedApiState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let page_index = pagination.page_index();
    let limit = pagination.limit();

    let data = article_processing_outputs::get_article_processing_outputs(
        state.db_pool(),
        page_index,
        limit,
    )
    .await?;
    let count =
        article_processing_outputs::count_article_processing_outputs(state.db_pool()).await?;

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
