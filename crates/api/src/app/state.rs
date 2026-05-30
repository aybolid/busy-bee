use std::sync::Arc;

use lapin::Connection;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::{
    ai::Client, app::config::Config, infra::db::DatabasePool, workers::publisher::PublisherCommand,
};

pub struct AppState {
    pub config: Config,

    pub db_pool: DatabasePool,
    pub amqp_connection: Connection,
    pub ai_client: Client,

    pub publisher_tx: Sender<PublisherCommand>,

    pub cancel_token: CancellationToken,
}

pub type SharedAppState = Arc<AppState>;
