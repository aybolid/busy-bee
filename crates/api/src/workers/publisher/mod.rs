use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{app::state::SharedAppState, workers::article_processor::ArticleDeliveryPayload};

const MPSC_CHANNEL_BUFFER_SIZE: usize = 100;

#[tracing::instrument(level = "trace")]
pub fn create_publisher_mpsc_channel() -> (Sender<PublisherCommand>, Receiver<PublisherCommand>) {
    tracing::trace!(MPSC_CHANNEL_BUFFER_SIZE);
    mpsc::channel(MPSC_CHANNEL_BUFFER_SIZE)
}

#[derive(Debug)]
pub enum PublisherCommand {
    ProcessArticle(ArticleDeliveryPayload),
}

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_publisher(
    state: SharedAppState,
    mut rx: Receiver<PublisherCommand>,
) -> Result<(), PublisherError> {
    tracing::trace!("publisher started");

    let channel = state.amqp_connection().create_channel().await?;
    tracing::trace!("publisher amqp channel created");

    loop {
        tokio::select! {
            command = rx.recv() => {
                if let Some(command) = command {
                    tracing::trace!(?command);
                    // Error logged by `tracing::instrument`
                    _ = process_command(&state, command, &channel).await;
                } else {
                    tracing::trace!("receiver has been closed and there are no more messages in the buffer");
                    break;
                }
            },
            () = state.cancel_token().cancelled() => {
                tracing::trace!("got shutdown signal");
                rx.close();
                tracing::trace!("publisher mpsc receiver closed");
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum ProcessCommandError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Amqp(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_command(
    state: &SharedAppState,

    command: PublisherCommand,
    channel: &Channel,
) -> Result<(), ProcessCommandError> {
    let (payload, queue) = match command {
        PublisherCommand::ProcessArticle(payload) => (
            serde_json::to_vec(&payload)?,
            state.config().article_processor_queue().clone(),
        ),
    };
    tracing::trace!(payload_bytes = payload.len(), ?queue);

    channel
        .basic_publish(
            "".into(),
            queue,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?
        .await?;
    tracing::trace!("message published");

    Ok(())
}
