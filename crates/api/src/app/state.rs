use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::{
    ai::Client,
    app::{config::Config, events::AppEventsBroadcaster},
    infra::db::DatabasePool,
    workers::article_processing::ProcessingRequestSender,
};

/// Represents the global application state and shared dependencies.
///
/// This struct acts as a central registry for all the core services, configuration,
/// and coordination primitives required across the application.
pub struct AppState {
    /// The application's configuration settings loaded at startup.
    pub config: Config,
    /// The connection pool used to manage and execute queries against the database.
    pub db_pool: DatabasePool,
    /// The client used to interact with external AI models and services.
    pub ai: Client,
    /// The sender half of a channel used to dispatch new tasks to the background article processing worker.
    pub article_processing_tx: ProcessingRequestSender,
    /// A channel broadcaster used to publish and subscribe to system-wide application events.
    pub app_events_broadcaster: AppEventsBroadcaster,
    /// A token used to signal and coordinate a graceful shutdown across asynchronous tasks.
    pub cancel_token: CancellationToken,
}

/// A thread-safe, reference-counted pointer to the [`AppState`].
///
/// This type alias is used to easily share the application state across multiple threads,
/// asynchronous tasks, and web server routes without requiring expensive clones of the underlying pools and clients.
pub type SharedAppState = Arc<AppState>;
