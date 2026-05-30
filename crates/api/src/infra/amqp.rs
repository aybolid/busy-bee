use lapin::{
    Channel, Connection, ConnectionProperties, Queue,
    options::QueueDeclareOptions,
    protocol::constants::REPLY_SUCCESS,
    types::{FieldTable, ShortString},
};

use crate::app::config::Config;

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn amqp_connect(config: &Config) -> lapin::Result<Connection> {
    Connection::connect(config.amqp_url.as_str(), ConnectionProperties::default())
        .await
        .inspect(|_| tracing::info!("amqp connection created"))
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn amqp_close(amqp_connection: &Connection) -> lapin::Result<()> {
    amqp_connection
        .close(REPLY_SUCCESS, "bye".into())
        .await
        .inspect(|()| tracing::info!("amqp connection closed"))
}

#[tracing::instrument(level = "trace", skip(channel), err(Debug))]
pub async fn declare_durable_queue(channel: &Channel, queue: ShortString) -> lapin::Result<Queue> {
    channel
        .queue_declare(queue, QueueDeclareOptions::durable(), FieldTable::default())
        .await
        .inspect(|_| tracing::trace!("durable queue declared"))
}
