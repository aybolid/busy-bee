use crate::{
    app::state::SharedAppState,
    workers::article_processing::{ProcessingRequestReceiver, processing::process_article},
};

#[derive(Debug, thiserror::Error)]
#[error("mpsc channel closed")]
pub struct ProcessingRequestChannelClosed;

/// Runs the background worker loop for processing articles.
///
/// This asynchronous function continuously listens for incoming processing requests
/// over a multi-producer, single-consumer (mpsc) channel. When a request is received,
/// it delegates the actual processing to the [`process_article`] function.
///
/// The loop multiplexes (via [`tokio::select!`]) between receiving new requests and
/// listening for a graceful shutdown signal.
///
/// # Exit Conditions
///
/// This function runs indefinitely until one of two things happens:
///
/// 1. **Graceful Shutdown:** The `cancel_token` on the shared state is triggered (e.g., via Ctrl+C).
/// 2. **Channel Closure:** All `ProcessingRequestSender` instances are dropped, causing the channel to close.
#[tracing::instrument(name = "article_processing", skip_all)]
pub async fn run_article_processing(
    state: SharedAppState,
    mut rx: ProcessingRequestReceiver,
) -> Result<(), ProcessingRequestChannelClosed> {
    tracing::info!("listening to the processing requests");

    loop {
        tokio::select! {
            Some(request) = rx.recv() => {
                process_article(&state, request).await;
            }
            () = state.cancel_token.cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
            else => {
                return Err(ProcessingRequestChannelClosed);
            }
        }
    }

    Ok(())
}
