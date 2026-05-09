use std::sync::Arc;

use crate::internal::{api::ApiContext, app::config::Config};

pub type SharedAppState = Arc<AppState>;

pub(super) struct AppState {
    config: Config,
}

impl AppState {
    pub(super) fn config(&self) -> &Config {
        &self.config
    }
}

impl From<ApiContext> for AppState {
    fn from(value: ApiContext) -> Self {
        Self {
            config: value.config,
        }
    }
}
