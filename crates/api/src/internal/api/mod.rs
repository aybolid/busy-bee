use std::io;

use axum::Router;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

use crate::internal::{
    api::state::{AppState, SharedAppState},
    app::config::Config,
};

mod state;

pub struct ApiContext {
    pub config: Config,
    pub cancel_token: CancellationToken,
}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_api_server(context: ApiContext) -> io::Result<()> {
    let listener = TcpListener::bind(context.config.api_addr()).await?;
    tracing::info!("listening on {}", context.config.api_addr());

    let cancel_token = context.cancel_token.clone();

    let router = create_api_router(context.into());

    // Will never actually return an error
    _ = axum::serve(listener, router)
        .with_graceful_shutdown(
            async move {
                cancel_token.cancelled().await;
                tracing::trace!("got shutdown signal")
            }
            .instrument(tracing::Span::current()),
        )
        .await;

    Ok(())
}

fn create_api_router(state: AppState) -> Router {
    let state = SharedAppState::new(state);

    Router::new().with_state(state)
}
