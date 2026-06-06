use axum::{Router, routing::get};

use crate::{app::state::SharedAppState, workers::api::handlers::outputs};

pub fn router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", get(outputs::get_outputs))
        .route("/{output_id}", get(outputs::get_output));

    Router::new().nest("/outputs", router)
}
