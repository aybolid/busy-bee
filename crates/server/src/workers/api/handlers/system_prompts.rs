use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    app::state::SharedAppState,
    repos::{
        VersionNumber,
        system_prompts::{
            self, SystemPromptId, SystemPromptText, SystemPromptTitle, SystemPromptUpdateData,
        },
    },
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{ReqJson, ReqPath},
        resp::data,
    },
};

/// JSON payload containing data for creating a new system prompt.
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

/// Retrieves a system prompt by ID.
#[tracing::instrument(skip(state))]
pub async fn get_system_prompt(
    State(state): State<SharedAppState>,
    ReqPath(system_prompt_id): ReqPath<SystemPromptId>,
) -> HandlerResult<impl IntoResponse> {
    let prompt = system_prompts::get_system_prompt(&state.db_pool, system_prompt_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("system prompt not found"))?;

    Ok(data(prompt))
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

/// Request JSON containing the data to update a system prompt with.
#[derive(Debug, serde::Deserialize)]
pub struct UpdateSystemPromptJson {
    version: VersionNumber,
    title: Option<SystemPromptTitle>,
    text: Option<SystemPromptText>,
}

/// Updates a specific system prompt by its unique ID using [`UpdateSystemPromptJson`].
#[tracing::instrument(skip(state))]
pub async fn update_system_prompt(
    State(state): State<SharedAppState>,
    ReqPath(system_prompt_id): ReqPath<SystemPromptId>,
    ReqJson(json): ReqJson<UpdateSystemPromptJson>,
) -> HandlerResult<impl IntoResponse> {
    let prompt_to_update = system_prompts::get_system_prompt(&state.db_pool, system_prompt_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("system prompt not found"))?;

    // `update_system_prompt_by_id` below also checks version so this one is just
    // for better UX.
    if prompt_to_update.version != json.version {
        return Err(HandlerError::validation_with_source(
            "version mismatch",
            "version",
        ));
    }

    let update_data = SystemPromptUpdateData::new(system_prompt_id, json.version)
        .title(json.title.as_ref())
        .text(json.text.as_ref());

    let prompt = system_prompts::update_system_prompt_by_id(&state.db_pool, &update_data)
        .await?
        .ok_or_else(|| HandlerError::not_found("system prompt not found"))?;

    Ok(data(prompt))
}
