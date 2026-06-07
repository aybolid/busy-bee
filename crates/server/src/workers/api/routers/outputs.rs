use axum::{
    Router,
    routing::{delete, get},
};

use crate::{app::state::SharedAppState, workers::api::handlers::outputs};

/// Creates a [`Router`] that handles `/outputs` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new().route("/", get(outputs::get_outputs)).nest(
        "/{output_id}",
        Router::new()
            .route("/", get(outputs::get_output))
            .route("/", delete(outputs::delete_output)),
    );

    Router::new().nest("/outputs", router)
}
