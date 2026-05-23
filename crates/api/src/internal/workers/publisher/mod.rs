use lapin::{BasicProperties, Channel, options::BasicPublishOptions, types::ShortString};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::internal::workers::article_processor::ArticleDeliveryPayload;

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

#[derive(Debug)]
pub struct Queues {
    article_processor_queue: ShortString,
}

impl Queues {
    pub fn new(article_processor_queue: ShortString) -> Self {
        Self {
            article_processor_queue,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {}

#[tracing::instrument(level = "trace", skip_all, err)]
pub async fn run_publisher(
    mut rx: Receiver<PublisherCommand>,
    channel: Channel,
    queues: Queues,
) -> Result<(), PublisherError> {
    while let Some(command) = rx.recv().await {
        tracing::trace!("got publish command");
        _ = process_command(command, &channel, &queues).await;
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
    command: PublisherCommand,
    channel: &Channel,
    queues: &Queues,
) -> Result<(), ProcessCommandError> {
    let (payload, queue) = match command {
        PublisherCommand::ProcessArticle(payload) => (
            serde_json::to_vec(&payload)?,
            queues.article_processor_queue.clone(),
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
