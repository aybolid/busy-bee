use crate::{
    app::state::SharedAppState,
    workers::article_processing::{ProcessingRequestReceiver, processing::process_article},
};

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
///
/// # Returns
///
/// Returns `Ok(())` upon exiting. The [`std::convert::Infallible`] error type indicates that this
/// background task handles its own internal errors and will never bubble up a fatal crash.
pub async fn run_article_processing(
    state: SharedAppState,
    mut rx: ProcessingRequestReceiver,
) -> Result<(), std::convert::Infallible> {
    loop {
        tokio::select! {
            Some(request) = rx.recv() => {
                process_article(&state, request).await;
            }
            () = state.cancel_token.cancelled() => {
                tracing::info!("got shutdown signal");
                break;
            }
            else => {
                tracing::error!("mpsc channel closed");
                break;
            }
        }
    }

    Ok(())
}
