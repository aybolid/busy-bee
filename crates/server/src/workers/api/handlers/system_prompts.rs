use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    app::state::SharedAppState,
    repos::system_prompts::{self, SystemPromptId, SystemPromptText, SystemPromptTitle},
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{ReqJson, ReqPath},
        resp::data,
    },
};

/// JSON payload containing data for creating a new system prompt feed.
#[derive(Debug, serde::Deserialize)]
pub struct CreateSystemPromptJson {
    title: SystemPromptTitle,
    text: SystemPromptText,
}

/// Creates a new system prompt.
#[tracing::instrument(skip(state))]
pub async fn create_system_prompt(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<CreateSystemPromptJson>,
) -> HandlerResult<impl IntoResponse> {
    let prompt =
        system_prompts::create_system_prompt(&state.db_pool, &json.title, &json.text).await?;

    Ok(data(prompt))
}

/// Retrieves a complete list of all system prompts.
#[tracing::instrument(skip(state))]
pub async fn get_system_prompts(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let prompts = system_prompts::get_system_prompts(&state.db_pool).await?;

    Ok(data(prompts))
}

/// Deletes a specific system prompt by its unique ID.
#[tracing::instrument(skip(state))]
pub async fn delete_system_prompt(
    State(state): State<SharedAppState>,
    ReqPath(system_prompt_id): ReqPath<SystemPromptId>,
) -> HandlerResult<impl IntoResponse> {
    system_prompts::delete_system_prompt_by_id(&state.db_pool, system_prompt_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("system prompt not found"))?;

    Ok(StatusCode::NO_CONTENT)
}
