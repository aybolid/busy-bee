use std::io;

use axum::Router;
use tokio::net::TcpListener;

use crate::{
    app::state::SharedAppState,
    workers::api::routers::{articles, notifications, outputs, rss_feeds},
};

mod err;
mod handlers;
mod req;
mod resp;
mod routers;

pub async fn run_api_server(state: SharedAppState) -> io::Result<()> {
    let cancel_token = state.cancel_token.clone();

    let listener = TcpListener::bind(state.config.api_addr).await?;

    let router = create_api_router(state);

    // Will never actually return an error
    _ = axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            cancel_token.cancelled().await;
        })
        .await;

    Ok(())
}

fn create_api_router(state: SharedAppState) -> Router {
    let router = Router::new()
        .merge(notifications::router())
        .merge(rss_feeds::router())
        .merge(articles::router())
        .merge(outputs::router());

    Router::new().nest("/api", router).with_state(state)
}
