use std::time::Duration;

use axum::{
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};

use crate::app::{events::AppEvent, state::SharedAppState};

#[tracing::instrument(level = "trace", skip_all)]
#[allow(clippy::unused_async)]
pub async fn sse(
    State(state): State<SharedAppState>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.app_events_broadcaster.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(event) => {
            tracing::trace!(?event);
            match event {
                AppEvent::Notification(data) => {
                    if let Ok(json) = serde_json::to_string(&data) {
                        Some(Ok(Event::default().event("notification").data(json)))
                    } else {
                        None
                    }
                }
                AppEvent::RefetchTrigger(trigger_type) => Some(Ok(Event::default()
                    .event("refetch_trigger")
                    .data(trigger_type))),
            }
        }
        Err(error) => {
            tracing::error!(?error);
            None
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
