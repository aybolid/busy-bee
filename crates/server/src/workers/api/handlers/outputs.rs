use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    app::state::SharedAppState,
    repos::{
        Pagination,
        outputs::{self, OutputId},
    },
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::ReqPath,
        resp::{Metadata, data, data_with_meta},
    },
};

/// Retrieves a paginated list of processing outputs.
#[tracing::instrument(skip_all)]
pub async fn get_outputs(
    State(state): State<SharedAppState>,
    Query(pagination): Query<Pagination>,
) -> HandlerResult<impl IntoResponse> {
    let data = outputs::get_outputs(&state.db_pool, pagination).await?;
    let count = outputs::count_outputs(&state.db_pool).await?;

    Ok(data_with_meta(
        data,
        Metadata::pagination(pagination, count),
    ))
}

/// Retrieves a specific processing output by its unique ID.
#[tracing::instrument(skip_all)]
pub async fn get_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<OutputId>,
) -> HandlerResult<impl IntoResponse> {
    let output = outputs::get_output_by_id(&state.db_pool, output_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(data(output))
}

/// Deletes a specific output by its unique ID.
#[tracing::instrument(skip_all)]
pub async fn delete_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<OutputId>,
) -> HandlerResult<impl IntoResponse> {
    outputs::delete_output_by_id(&state.db_pool, output_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(StatusCode::NO_CONTENT)
}
