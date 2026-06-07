use axum::{Router, routing::get};

use crate::{app::state::SharedAppState, workers::api::handlers::outputs};

/// Creates a [`Router`] that handles `/outputs` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new().route("/", get(outputs::get_outputs)).nest(
        "/{output_id}",
        Router::new().route("/", get(outputs::get_output)),
    );

    Router::new().nest("/outputs", router)
}
