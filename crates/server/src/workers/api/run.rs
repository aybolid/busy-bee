use std::{io, time::Duration};

use axum::{Router, extract::Request, http::HeaderName};
use reqwest::StatusCode;
use tokio::net::TcpListener;
use tower_http::{
    catch_panic::CatchPanicLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Instrument;

use crate::{
    app::state::SharedAppState,
    workers::api::{
        handlers::{handle_panic, not_found},
        routers::{articles, notifications, outputs, rss_feeds, system_prompts},
    },
};

const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-header-id");

/// Initializes and runs the main API server.
///
/// Binds to the configured TCP address, sets up the application router, and
/// continuously serves requests until a cancellation signal triggers a graceful shutdown.
#[tracing::instrument(name = "http_api", skip_all)]
pub async fn run_http_api(state: SharedAppState) -> io::Result<()> {
    let cancel_token = state.cancel_token.clone();

    let listener = TcpListener::bind(state.config.api_addr).await?;
    tracing::info!("listening on {:?}", state.config.api_addr);

    let router = create_api_router(state);

    // Will never actually return an error
    _ = axum::serve(listener, router)
        .with_graceful_shutdown(
            async move {
                cancel_token.cancelled().await;
                tracing::info!("got shutdown signal");
            }
            .instrument(tracing::Span::current()),
        )
        .await;

    Ok(())
}

/// Constructs the main application router.
///
/// Merges domain-specific routes under the `/api` namespace, attaches the shared state,
/// and applies global middleware for request ID tracking, tracing, timeouts, and panic handling.
fn create_api_router(state: SharedAppState) -> Router {
    let router = Router::new()
        .merge(notifications::router())
        .merge(rss_feeds::router())
        .merge(articles::router())
        .merge(outputs::router())
        .merge(system_prompts::router());

    let global_middleware = (
        SetRequestIdLayer::new(X_REQUEST_ID, MakeRequestUuid),
        TraceLayer::new_for_http().make_span_with(|req: &Request| {
            let method = req.method();
            let uri = req.uri();
            req.headers().get(X_REQUEST_ID).map_or_else(
                || {
                    tracing::error!("could not extract request id");
                    tracing::info_span!("http_request", ?method, ?uri)
                },
                |id| tracing::info_span!("http_request", ?id, ?method, ?uri),
            )
        }),
        PropagateRequestIdLayer::new(X_REQUEST_ID),
        TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        CatchPanicLayer::custom(handle_panic),
    );

    Router::new()
        .nest("/api", router)
        .with_state(state)
        .fallback(not_found)
        .layer(global_middleware)
}
