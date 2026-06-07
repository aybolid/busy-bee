use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use tokio::{
    sync::{Semaphore, watch},
    task::{AbortHandle, JoinSet},
};

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds::RssFeedId,
    workers::rss_processing::{
        config::RssFeedConfig,
        reading::{RssReaderState, read_rss_feed},
        run::RssProcessingError,
    },
};

/// A control handle for a currently running RSS reader background task.
///
/// This struct wraps the `AbortHandle` for the asynchronous task along with the
/// configuration that was used to spawn it. This allows the manager to compare
/// existing configurations with new ones and cleanly terminate tasks if their
/// configurations change or if the feed is deleted.
#[derive(Debug)]
struct ReaderHandle {
    /// The handle used to send a cancellation signal to the spawned task.
    handle: AbortHandle,
    /// The configuration payload the task is currently using.
    config: RssFeedConfig,
}

impl ReaderHandle {
    /// Cancels the running asynchronous task associated with this handle.
    fn abort(&self) {
        tracing::trace!(id = %self.config.id.as_hyphenated(), "aborting reader");
        self.handle.abort();
    }
}

/// Orchestrates the lifecycle of multiple concurrent RSS feed reader tasks.
///
/// This manager utilizes a [`JoinSet`] to efficiently run and monitor multiple asynchronous
/// reader tasks. It maps each [`RssFeedId`] to its corresponding [`ReaderHandle`], enabling
/// granular control to spawn, update, or remove specific readers without interrupting others.
#[derive(Debug, Default)]
struct ReadingManager {
    /// The collection of running async tasks, automatically cancelling them if the [`JoinSet`] is dropped.
    readers: JoinSet<()>,
    /// A registry mapping active feed IDs to their control handles.
    readers_map: HashMap<RssFeedId, ReaderHandle>,
}

impl ReadingManager {
    /// Retrieves the control handle for a running reader by its feed ID, if it exists.
    fn get_reader_handle(&self, id: &RssFeedId) -> Option<&ReaderHandle> {
        self.readers_map.get(id)
    }

    /// Retains only the readers whose IDs are present in the provided set.
    ///
    /// Any running reader whose ID is **not** in the `to_retain` set will be aborted
    /// and removed from the manager's registry.
    fn retain_readers(&mut self, to_retain: &HashSet<RssFeedId>) {
        self.readers_map.retain(|id, reader_handle| {
            if to_retain.contains(id) {
                true
            } else {
                reader_handle.abort();
                false
            }
        });
    }

    /// Spawns a new reader task and registers its control handle.
    fn spawn_reader(
        &mut self,
        config: RssFeedConfig,
        reader_task: impl Future<Output = ()> + Send + 'static,
    ) {
        tracing::trace!(id = %config.id.as_hyphenated(), "spawning new reader");
        self.readers_map.insert(
            config.id,
            ReaderHandle {
                handle: self.readers.spawn(reader_task),
                config,
            },
        );
    }
}

/// The main entry point for the background worker that manages RSS feed reading.
///
/// This function listens for updates to the active RSS feed configurations via a
/// `watch::Receiver`. When a change is detected, it synchronizes the running tasks:
///
/// * **Spawns** tasks for newly added feeds.
/// * **Restarts** tasks if a feed's configuration has been updated.
/// * **Aborts** tasks for feeds that have been removed or disabled.
///
/// It also monitors for application shutdown signals and panics/errors within
/// individual reader tasks.
///
/// # Errors
///
/// Returns an [`RssProcessingError`] if the watch channel closes unexpectedly, or if
/// an underlying reader task panics/errors out.
#[tracing::instrument(name = "rss_reading_manager", skip_all, err(Debug))]
pub async fn run_rss_reading_manager(
    state: SharedAppState,
    mut rx: watch::Receiver<Vec<RssFeedConfig>>,
) -> Result<(), RssProcessingError> {
    let mut manager = ReadingManager::default();

    // Reusing a single HTTP client across all readers is highly recommended for connection pooling.
    let http_client = reqwest::Client::default();

    let mut iteration_ids: HashSet<RssFeedId> = HashSet::new();
    loop {
        {
            let configs_ref = rx.borrow_and_update();

            for config in configs_ref.iter() {
                iteration_ids.insert(config.id);

                let mut spawn_new = false;

                if let Some(existing_handle) = manager.get_reader_handle(&config.id) {
                    // If the config changed (e.g., poll interval updated), restart the reader
                    if existing_handle.config != *config {
                        existing_handle.abort();
                        spawn_new = true;
                    }
                } else {
                    spawn_new = true; // No existing handle means it's a completely new feed
                }

                if spawn_new {
                    manager.spawn_reader(
                        config.clone(),
                        rss_reader(state.clone(), config.clone(), http_client.clone()),
                    );
                }
            }

            // Clean up any tasks running for feeds that are no longer in the configurations list
            manager.retain_readers(&iteration_ids);
            iteration_ids.clear();
        }

        tokio::select! {
            // Wait for the next configuration update broadcast
            result = rx.changed() => {
                if result.is_err() && !state.cancel_token.is_cancelled() {
                    return Err(RssProcessingError::WatchChannelClosed);
                }
            }
            // Graceful shutdown signal received
            () = state.cancel_token.cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
            // Monitor the active reader tasks; if one panics/fails, propagate the error
            Some(result) = manager.readers.join_next(), if !manager.readers.is_empty() => {
                if let Err(error) = result && !error.is_cancelled() {
                    return Err(RssProcessingError::Join(error))
                }
            }
        }
    }

    Ok(())
}

/// The isolated background loop for processing a single RSS feed.
///
/// This function executes continuously on a specific interval defined by the feed's configuration.
/// It wraps the configuration, HTTP client, and concurrency semaphore into an [`RssReaderState`]
/// before delegating to the actual fetching logic.
async fn rss_reader(state: SharedAppState, config: RssFeedConfig, http_client: reqwest::Client) {
    let interval_duration = Duration::from_secs(u64::from(config.fetch_interval_seconds.get()));
    let mut interval = tokio::time::interval(interval_duration);

    let request_semaphore = Semaphore::new(usize::from(config.max_concurrent_requests.get()));

    let reader_state = Arc::new(RssReaderState {
        app_state: state,
        config,
        http_client,
        request_semaphore,
    });

    loop {
        interval.tick().await;
        read_rss_feed(reader_state.clone()).await;
    }
}
