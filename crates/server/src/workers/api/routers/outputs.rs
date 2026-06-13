use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{app::state::SharedAppState, workers::api::handlers::outputs};

/// Creates a [`Router`] that handles `/outputs` routes.
pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", get(outputs::get_outputs))
        .nest(
            "/{output_id}",
            Router::new()
                .route("/", get(outputs::get_output))
                .route("/", delete(outputs::delete_output))
                .route("/", patch(outputs::update_output)),
        )
        .nest(
            "/bulk",
            Router::new().route("/delete", post(outputs::bulk_delete_outputs)),
        );

    Router::new().nest("/outputs", router)
}
