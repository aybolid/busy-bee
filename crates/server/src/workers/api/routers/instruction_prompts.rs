use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{app::state::SharedAppState, workers::api::handlers::instruction_prompts};

/// Creates a [`Router`] that handles `/instruction_prompts` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", post(instruction_prompts::create_instruction_prompt))
        .route("/", get(instruction_prompts::get_instruction_prompts))
        .nest(
            "/{instruction_prompt_id}",
            Router::new()
                .route("/", delete(instruction_prompts::delete_instruction_prompt))
                .route("/", get(instruction_prompts::get_instruction_prompt))
                .route("/", patch(instruction_prompts::update_instruction_prompt)),
        );

    Router::new().nest("/instruction_prompts", router)
}
