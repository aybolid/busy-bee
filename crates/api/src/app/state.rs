use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::{
    ai::Client,
    app::{config::Config, events::AppEventsBroadcaster},
    infra::db::DatabasePool,
    workers::article_processor::ArticleProcessingSender,
};

pub struct AppState {
    pub config: Config,

    pub db_pool: DatabasePool,
    pub ai_client: Client,

    pub article_processing_tx: ArticleProcessingSender,
    pub app_events_broadcaster: AppEventsBroadcaster,

    pub cancel_token: CancellationToken,
}

pub type SharedAppState = Arc<AppState>;
