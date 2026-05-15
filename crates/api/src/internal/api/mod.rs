use std::io;

use axum::Router;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

use crate::internal::api::{
    routers::articles,
    state::{ApiState, SharedApiState},
};

mod err;
mod handlers;
mod req;
mod resp;
mod routers;
pub mod state;

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_api_server(state: ApiState, cancel_token: CancellationToken) -> io::Result<()> {
    let listener = TcpListener::bind(state.config().api_addr()).await?;
    tracing::info!("listening on {}", state.config().api_addr());

    let router = create_api_router(state);

    // Will never actually return an error
    _ = axum::serve(listener, router)
        .with_graceful_shutdown(
            async move {
                cancel_token.cancelled().await;
                tracing::trace!("got shutdown signal");
            }
            .instrument(tracing::Span::current()),
        )
        .await;

    Ok(())
}

fn create_api_router(state: ApiState) -> Router {
    let state = SharedApiState::new(state);

    let router = Router::new().merge(articles::router());

    Router::new().nest("/api", router).with_state(state)
}
