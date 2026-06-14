use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    app::state::SharedAppState,
    repos::{
        VersionNumber,
        instruction_prompts::{
            self, InstructionPromptId, InstructionPromptText, InstructionPromptTitle,
            InstructionPromptUpdateData,
        },
    },
    workers::api::{
        err::{HandlerError, HandlerResult},
        req::{ReqJson, ReqPath},
        resp::data,
    },
};

/// JSON payload containing data for creating a new instruction prompt.
#[derive(Debug, serde::Deserialize)]
pub struct CreateInstructionPromptJson {
    title: InstructionPromptTitle,
    text: InstructionPromptText,
}

/// Creates a new instruction prompt.
#[tracing::instrument(skip(state))]
pub async fn create_instruction_prompt(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<CreateInstructionPromptJson>,
) -> HandlerResult<impl IntoResponse> {
    let prompt =
        instruction_prompts::create_instruction_prompt(&state.db_pool, &json.title, &json.text)
            .await?;

    Ok(data(prompt))
}

/// Retrieves a complete list of all instruction prompts.
#[tracing::instrument(skip(state))]
pub async fn get_instruction_prompts(
    State(state): State<SharedAppState>,
) -> HandlerResult<impl IntoResponse> {
    let prompts = instruction_prompts::get_instruction_prompts(&state.db_pool).await?;

    Ok(data(prompts))
}

/// Retrieves an instruction prompt by ID.
#[tracing::instrument(skip(state))]
pub async fn get_instruction_prompt(
    State(state): State<SharedAppState>,
    ReqPath(instruction_prompt_id): ReqPath<InstructionPromptId>,
) -> HandlerResult<impl IntoResponse> {
    let prompt = instruction_prompts::get_instruction_prompt(&state.db_pool, instruction_prompt_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("instruction prompt not found"));

    Ok(data(prompt))
}

/// Deletes a specific instruction prompt by its unique ID.
#[tracing::instrument(skip(state))]
pub async fn delete_instruction_prompt(
    State(state): State<SharedAppState>,
    ReqPath(instruction_prompt_id): ReqPath<InstructionPromptId>,
) -> HandlerResult<impl IntoResponse> {
    instruction_prompts::delete_instruction_prompt_by_id(&state.db_pool, instruction_prompt_id)
        .await?
        .ok_or_else(|| HandlerError::not_found("instruction prompt not found"))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Request JSON containing the data to update an instruction prompt with.
#[derive(Debug, serde::Deserialize)]
pub struct UpdateInstructionPromptJson {
    version: VersionNumber,
    title: Option<InstructionPromptTitle>,
    text: Option<InstructionPromptText>,
}

/// Updates a specific instruction prompt by its unique ID using [`UpdateInstructionPromptJson`].
#[tracing::instrument(skip(state))]
pub async fn update_instruction_prompt(
    State(state): State<SharedAppState>,
    ReqPath(instruction_prompt_id): ReqPath<InstructionPromptId>,
    ReqJson(json): ReqJson<UpdateInstructionPromptJson>,
) -> HandlerResult<impl IntoResponse> {
    let prompt_to_update =
        instruction_prompts::get_instruction_prompt(&state.db_pool, instruction_prompt_id)
            .await?
            .ok_or_else(|| HandlerError::not_found("instruction prompt not found"))?;

    // `update_instruction_prompt_by_id` below also checks version so this one is just
    // for better UX.
    if prompt_to_update.version != json.version {
        return Err(HandlerError::validation_with_source(
            "version mismatch",
            "version",
        ));
    }

    let update_data = InstructionPromptUpdateData::new(instruction_prompt_id, json.version)
        .title(json.title.as_ref())
        .text(json.text.as_ref());

    let prompt = instruction_prompts::update_instruction_prompt_by_id(&state.db_pool, &update_data)
        .await?
        .ok_or_else(|| HandlerError::not_found("instruction prompt not found"))?;

    Ok(data(prompt))
}
