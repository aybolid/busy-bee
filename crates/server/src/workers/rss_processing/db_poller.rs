use std::time::Duration;

use tokio::sync::watch;

use crate::{
    app::state::SharedAppState,
    infra::db::DatabaseExecutor,
    repos::rss_feeds,
    workers::rss_processing::{config::RssFeedConfig, run::RssProcessingError},
};

/// The duration to wait between consecutive database polls.
const INTERVAL_DURATION: Duration = Duration::from_secs(10);

/// A background worker that continuously polls the database for changes to RSS feed configurations.
///
/// This function runs in an infinite loop, fetching the current list of [`RssFeedConfig`]s
/// from the database every [`INTERVAL_DURATION`]. If it detects a change from the previously known
/// state, it broadcasts the updated list via the provided `watch::Sender` channel.
///
/// Returns an [`RssProcessingError::WatchChannelClosed`] if all receivers of the
/// watch channel have been dropped, indicating that the worker should terminate.
#[tracing::instrument(name = "db_poller", skip_all, err(Debug))]
pub async fn run_db_poller(
    state: SharedAppState,
    tx: watch::Sender<Vec<RssFeedConfig>>,
) -> Result<(), RssProcessingError> {
    let mut interval = tokio::time::interval(INTERVAL_DURATION);
    let mut last_known_configs: Vec<RssFeedConfig> = vec![];

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            () = state.cancel_token.cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
        }

        let new_configs = match get_rss_feed_configs(&state.db_pool).await {
            Ok(configs) => {
                // Deduplicate broadcasts: only proceed if the database state has actually changed.
                if configs == last_known_configs {
                    continue;
                }
                configs
            }
            Err(error) => {
                tracing::error!(%error);
                continue;
            }
        };

        tracing::trace!("sending new rss feed configs");

        match tx.send(new_configs.clone()) {
            Ok(()) => last_known_configs = new_configs,
            Err(_) => return Err(RssProcessingError::WatchChannelClosed),
        }
    }

    Ok(())
}

/// Retrieves all active RSS feeds from the database and maps them into operational configurations.
///
/// This is a convenience helper that queries the raw database models via `rss_feeds::get_rss_feeds`
/// and converts them into [`RssFeedConfig`] objects tailored for the processing worker.
///
/// # Errors
///
/// Returns an `sqlx::Result` error if the underlying database query fails.
pub async fn get_rss_feed_configs<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<Vec<RssFeedConfig>> {
    let feeds = rss_feeds::get_rss_feeds(executor).await?;
    Ok(feeds.into_iter().map(RssFeedConfig::from).collect())
}
