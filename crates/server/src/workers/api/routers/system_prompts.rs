use axum::{Router, routing::post};

use crate::{app::state::SharedAppState, workers::api::handlers::system_prompts};

/// Creates a [`Router`] that handles `/system_prompts` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new().route("/", post(system_prompts::create_system_prompt));

    Router::new().nest("/system_prompts", router)
}
