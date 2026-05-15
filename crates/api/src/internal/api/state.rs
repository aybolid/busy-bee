use std::sync::Arc;

use crate::internal::{app::config::Config, infra::db::DatabasePool};

pub type SharedApiState = Arc<ApiState>;

pub struct ApiState {
    config: Config,
    db_pool: DatabasePool,
}

impl ApiState {
    pub fn new(config: Config, db_pool: DatabasePool) -> Self {
        Self { config, db_pool }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db_pool(&self) -> &DatabasePool {
        &self.db_pool
    }
}
