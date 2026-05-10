use std::sync::Arc;

use crate::internal::app::config::Config;

pub type SharedApiState = Arc<ApiState>;

pub struct ApiState {
    config: Config,
}

impl ApiState {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
