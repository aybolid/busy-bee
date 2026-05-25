use std::sync::Arc;

use lapin::Connection;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::{
    ai::Client, app::config::Config, infra::db::DatabasePool, workers::publisher::PublisherCommand,
};

pub struct AppState {
    config: Config,

    db_pool: DatabasePool,
    amqp_connection: Connection,
    ai_client: Client,

    publisher_tx: Sender<PublisherCommand>,

    cancel_token: CancellationToken,
}

impl AppState {
    pub fn new(
        config: Config,

        db_pool: DatabasePool,
        amqp_connection: Connection,
        ai_client: Client,

        publisher_tx: Sender<PublisherCommand>,

        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            config,

            db_pool,
            amqp_connection,
            ai_client,

            publisher_tx,

            cancel_token,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db_pool(&self) -> &DatabasePool {
        &self.db_pool
    }

    pub fn amqp_connection(&self) -> &Connection {
        &self.amqp_connection
    }

    pub fn ai_client(&self) -> &Client {
        &self.ai_client
    }

    pub fn publisher_tx(&self) -> &Sender<PublisherCommand> {
        &self.publisher_tx
    }

    pub fn cancel_token(&self) -> &CancellationToken {
        &self.cancel_token
    }
}

pub type SharedAppState = Arc<AppState>;
