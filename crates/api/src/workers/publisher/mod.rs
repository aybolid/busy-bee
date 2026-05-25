use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{app::state::SharedAppState, workers::article_processor::ArticleDeliveryPayload};

#[tracing::instrument(level = "trace")]
pub fn create_publisher_mpsc_channel() -> (Sender<PublisherCommand>, Receiver<PublisherCommand>) {
    let channel_buffer_size = 100;
    tracing::trace!(channel_buffer_size);
    mpsc::channel(channel_buffer_size)
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
    mut rx: Receiver<PublisherCommand>,
    state: SharedAppState,
) -> Result<(), PublisherError> {
    let channel = state.amqp_connection().create_channel().await?;

    while let Some(command) = rx.recv().await {
        tracing::trace!("got publish command");
        _ = process_command(&state, command, &channel).await;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum ProcessCommandError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
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
