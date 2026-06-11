use axum::{extract::State, response::IntoResponse};

use crate::{
    app::state::SharedAppState,
    repos::system_prompts::{self, SystemPromptText, SystemPromptTitle},
    workers::api::{err::HandlerResult, req::ReqJson, resp::data},
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
