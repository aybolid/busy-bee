use std::time::Duration;

use axum::{
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};

use crate::app::state::SharedAppState;

/// Establishes a Server-Sent Events (SSE) stream for real-time application updates.
///
/// Subscribes the client to the global application events broadcaster. The stream
/// silently ignores broadcast errors (such as lagged messages) and includes a
/// keep-alive "ping" sent every 15 seconds to prevent idle connections from dropping.
#[tracing::instrument(skip_all)]
#[allow(clippy::unused_async)]
pub async fn sse(
    State(state): State<SharedAppState>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.app_events_broadcaster.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(event) => event.into_sse_event().map(Ok),
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
