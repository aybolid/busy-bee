use crate::{
    app::state::SharedAppState,
    workers::article_processing::{ProcessingRequestReceiver, processing::process_article},
};

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
