use std::io;

use axum::Router;
use tokio::net::TcpListener;
use tracing::Instrument;

use crate::{
    app::state::SharedAppState,
    workers::api::routers::{article_processing_outputs, articles, notifications, rss_feeds},
};

mod err;
mod handlers;
mod req;
mod resp;
mod routers;

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn run_api_server(state: SharedAppState) -> io::Result<()> {
    let cancel_token = state.cancel_token.clone();

    let listener = TcpListener::bind(state.config.api_addr).await?;
    tracing::info!("listening on {}", state.config.api_addr);

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

fn create_api_router(state: SharedAppState) -> Router {
    let router = Router::new()
        .merge(notifications::router())
        .merge(rss_feeds::router())
        .merge(articles::router())
        .merge(article_processing_outputs::router());

    Router::new().nest("/api", router).with_state(state)
}
