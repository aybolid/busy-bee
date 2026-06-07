use tokio::{sync::watch, task::JoinSet};

use crate::{
    app::state::SharedAppState,
    workers::rss_processing::{
        config::RssFeedConfig, db_poller::run_db_poller, reading_manager::run_rss_reading_manager,
    },
};

/// Errors that can occur during the execution of the RSS processing background workers.
#[derive(Debug, thiserror::Error)]
pub enum RssProcessingError {
    /// An asynchronous background task panicked or was unexpectedly cancelled.
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    /// The inter-task communication channel for RSS configurations was closed.
    /// This typically indicates that either the database poller or the reading manager
    /// has unexpectedly terminated, breaking the configuration pipeline.
    #[error("watch channel closed")]
    WatchChannelClosed,
}

/// Initializes and runs the core background worker tasks for RSS feed processing.
///
/// This function acts as the supervisor for the RSS processing subsystem. It sets up an
/// asynchronous communication channel (`tokio::sync::watch`) and spawns two tightly coupled
/// subtasks:
///
/// 1. **The Database Poller ([`run_db_poller`])**: Periodically queries the database for the
///    latest RSS feed configurations and broadcasts them via the `watch` sender.
/// 2. **The Reading Manager ([`run_rss_reading_manager`])**: Listens to the `watch` receiver
///    and dynamically spawns, updates, or terminates individual feed reader tasks based on
///    the broadcasted configurations.
///
/// These tasks run concurrently within a `JoinSet` until the application receives a graceful
/// shutdown signal or an unrecoverable error occurs in one of the subtasks.
///
/// # Errors
///
/// Returns an [`RssProcessingError`] if any of the spawned subtasks fail, panic, or if the
/// internal communication channel is severed.
#[tracing::instrument(name = "rss_processing", skip_all, err(Debug))]
pub async fn run_rss_processing(state: SharedAppState) -> Result<(), RssProcessingError> {
    // Create a watch channel to broadcast the latest configurations from the poller to the manager.
    // It initializes with an empty vector.
    let (tx, rx) = watch::channel::<Vec<RssFeedConfig>>(vec![]);

    let mut subtasks = JoinSet::new();

    // Spawn the producer: fetches configurations from the database and sends them.
    subtasks.spawn(run_db_poller(state.clone(), tx));
    // Spawn the consumer: listens for configurations and manages individual feed tasks.
    subtasks.spawn(run_rss_reading_manager(state.clone(), rx));

    tracing::info!("running rss processing");

    // Monitor the subtasks. If either task finishes (via shutdown or error),
    // propagate the result. If one fails, the `JoinSet` handles cancelling the other.
    while let Some(result) = subtasks.join_next().await {
        result??;
    }

    Ok(())
}
