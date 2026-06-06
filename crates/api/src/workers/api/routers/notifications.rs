use axum::{Router, routing::get};

use crate::{app::state::SharedAppState, workers::api::handlers::notifications};

pub fn router() -> Router<SharedAppState> {
    let router = Router::new().route("/", get(notifications::sse));

    Router::new().nest("/sse", router)
}
