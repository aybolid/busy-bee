use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    app::state::SharedAppState,
    repos::outputs::{self, OutputId},
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{Pagination, ReqPath},
        resp::{Metadata, data, data_with_meta},
    },
};

pub async fn get_outputs(
    State(state): State<SharedAppState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let page_index = pagination.page_index();
    let limit = pagination.limit();

    let data = outputs::get_outputs(&state.db_pool, page_index, limit).await?;
    let count = outputs::count_outputs(&state.db_pool).await?;

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

pub async fn get_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<OutputId>,
) -> HandlerResult<impl IntoResponse> {
    let output = outputs::get_output_by_id(&state.db_pool, output_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(data(output))
}
