use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::internal::{
    app::config::Config, infra::db::DatabasePool, workers::publisher::PublisherCommand,
};

pub type SharedApiState = Arc<ApiState>;

pub struct ApiState {
    config: Config,
    db_pool: DatabasePool,
    amqp_tx: Sender<PublisherCommand>,
}

impl ApiState {
    pub fn new(config: Config, db_pool: DatabasePool, amqp_tx: Sender<PublisherCommand>) -> Self {
        Self {
            config,
            db_pool,
            amqp_tx,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db_pool(&self) -> &DatabasePool {
        &self.db_pool
    }

    pub fn amqp_tx(&self) -> &Sender<PublisherCommand> {
        &self.amqp_tx
    }
}
