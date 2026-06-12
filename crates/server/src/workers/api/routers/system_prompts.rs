use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{app::state::SharedAppState, workers::api::handlers::system_prompts};

/// Creates a [`Router`] that handles `/system_prompts` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", post(system_prompts::create_system_prompt))
        .route("/", get(system_prompts::get_system_prompts))
        .nest(
            "/{system_prompt_id}",
            Router::new()
                .route("/", delete(system_prompts::delete_system_prompt))
                .route("/", get(system_prompts::get_system_prompt)),
        );

    Router::new().nest("/system_prompts", router)
}
