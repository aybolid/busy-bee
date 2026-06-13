use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    app::state::SharedAppState,
    repos::{
        Pagination,
        outputs::{self, OutputId, OutputIds, OutputText, OutputUpdateData},
    },
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{ReqJson, ReqPath},
        resp::{Metadata, data, data_with_meta},
    },
};

/// Retrieves a paginated list of processing outputs.
#[tracing::instrument(skip(state))]
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
#[tracing::instrument(skip(state))]
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
#[tracing::instrument(skip(state))]
pub async fn delete_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<OutputId>,
) -> HandlerResult<impl IntoResponse> {
    outputs::delete_output_by_id(&state.db_pool, output_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Request JSON containing the data to update an output with.
#[derive(Debug, serde::Deserialize)]
pub struct UpdateOutputJson {
    text: Option<OutputText>,
}

/// Updates a specific output by its unique ID using [`UpdateOutputJson`].
#[tracing::instrument(skip(state))]
pub async fn update_output(
    State(state): State<SharedAppState>,
    ReqPath(output_id): ReqPath<OutputId>,
    ReqJson(json): ReqJson<UpdateOutputJson>,
) -> HandlerResult<impl IntoResponse> {
    let update_data = OutputUpdateData::new(output_id).text(json.text.as_ref());

    let output = outputs::update_output_by_id(&state.db_pool, &update_data)
        .await?
        .ok_or_else(|| HandlerError::not_found("output not found"))?;

    Ok(data(output))
}

/// JSON payload containing IDs of outputs to delete.
#[derive(Debug, serde::Deserialize)]
pub struct BulkDeleteOutputsJson {
    ids: OutputIds,
}

/// Bulk deletes outputs using their IDs.
pub async fn bulk_delete_outputs(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<BulkDeleteOutputsJson>,
) -> HandlerResult<impl IntoResponse> {
    outputs::bulk_delete_outputs(&state.db_pool, &json.ids).await?;

    Ok(StatusCode::NO_CONTENT)
}
