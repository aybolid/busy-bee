use std::{
    collections::{HashMap, HashSet},
    num::{NonZeroU8, NonZeroU32},
    time::Duration,
};

use tokio::{
    sync::watch,
    task::{AbortHandle, JoinSet},
};
use types::Url;

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds::{self, RssFeed, RssFeedId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RssFeedConfig {
    id: RssFeedId,
    url: Url,
    max_concurrent_requests: NonZeroU8,
    fetch_interval_seconds: NonZeroU32,
}

impl From<RssFeed> for RssFeedConfig {
    fn from(value: RssFeed) -> Self {
        Self {
            id: value.id(),
            max_concurrent_requests: value.max_concurrent_requests(),
            fetch_interval_seconds: value.fetch_interval_seconds(),
            url: value.into_url(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RssReaderError {}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn run_rss_reader(state: SharedAppState) -> Result<(), RssReaderError> {
    tracing::trace!("started");

    let (tx, rx) = watch::channel::<Vec<RssFeedConfig>>(vec![]);
    tracing::trace!("watch channel created");

    let mut subtasks = JoinSet::new();

    subtasks.spawn(db_poller(state.clone(), tx));
    subtasks.spawn(rss_reader_manager(state, rx));

    subtasks.join_all().await;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn rss_reader_manager(state: SharedAppState, mut rx: watch::Receiver<Vec<RssFeedConfig>>) {
    let mut worker_tasks = JoinSet::new();
    let mut active_configs: HashMap<RssFeedId, (AbortHandle, RssFeedConfig)> = HashMap::new();

    let mut iter_ids = HashSet::new();
    loop {
        {
            let configs_ref = rx.borrow_and_update();

            for config in configs_ref.iter() {
                iter_ids.insert(config.id);

                if let Some((abort_handle, old_config)) = active_configs.get(&config.id) {
                    if old_config != config {
                        tracing::trace!(id = ?config.id, "rss feed config changed");
                        abort_handle.abort();
                        tracing::trace!("previous worker aborted");

                        let new_handle =
                            worker_tasks.spawn(rss_feed_worker(state.clone(), config.clone()));

                        active_configs.insert(config.id, (new_handle, config.clone()));
                    }
                } else {
                    tracing::trace!(id = ?config.id, "got new config");
                    let new_handle =
                        worker_tasks.spawn(rss_feed_worker(state.clone(), config.clone()));
                    active_configs.insert(config.id, (new_handle, config.clone()));
                }
            }

            active_configs.retain(|id, (abort_handle, _)| {
                if iter_ids.contains(id) {
                    true
                } else {
                    tracing::trace!(?id, "config removed");
                    abort_handle.abort();
                    false
                }
            });

            iter_ids.clear();
        }

        tokio::select! {
            result = rx.changed() => {
                if result.is_err() {
                    tracing::trace!("rx channel closed");
                    break;
                }
            }
            () = state.cancel_token().cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
        }
    }
}

#[tracing::instrument(level = "trace", skip_all, fields(id = ?config.id, url = config.url.as_str()))]
async fn rss_feed_worker(_state: SharedAppState, config: RssFeedConfig) {
    tracing::trace!("started");

    let interval_duration = Duration::from_secs(u64::from(config.fetch_interval_seconds.get()));
    tracing::trace!(?interval_duration);

    let mut interval = tokio::time::interval(interval_duration);

    loop {
        interval.tick().await;

        tracing::trace!("doing smth!"); // TODO
    }
}

#[tracing::instrument(level = "trace", skip_all)]
async fn db_poller(state: SharedAppState, tx: watch::Sender<Vec<RssFeedConfig>>) {
    tracing::trace!("started");

    let interval_duration = Duration::from_secs(10);
    tracing::trace!(?interval_duration);

    let mut interval = tokio::time::interval(interval_duration);
    let mut last_known_configs = vec![];

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            () = state.cancel_token().cancelled() => {
                tracing::trace!("got shutdown signal");
                break;
            }
        }

        match rss_feeds::get_rss_feeds(state.db_pool()).await {
            Ok(feeds) => {
                let configs = feeds
                    .into_iter()
                    .map(RssFeedConfig::from)
                    .collect::<Vec<_>>();

                if configs != last_known_configs {
                    tracing::trace!("rss feeds change detected");
                    if tx.send(configs.clone()).is_ok() {
                        last_known_configs = configs;
                    } else {
                        tracing::trace!("channel closed. exiting");
                        break;
                    }
                }
            }
            Err(error) => tracing::error!(?error),
        }
    }
}
