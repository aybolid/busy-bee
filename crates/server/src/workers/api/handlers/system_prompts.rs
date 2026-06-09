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

/// Creates a new RSS feed configuration.
#[tracing::instrument(skip(state))]
pub async fn create_system_prompt(
    State(state): State<SharedAppState>,
    ReqJson(json): ReqJson<CreateSystemPromptJson>,
) -> HandlerResult<impl IntoResponse> {
    let prompt =
        system_prompts::create_system_prompt(&state.db_pool, &json.title, &json.text).await?;

    Ok(data(prompt))
}
