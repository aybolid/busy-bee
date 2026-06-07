use std::{
    collections::{HashMap, HashSet},
    num::{NonZeroU8, NonZeroU32},
    time::Duration,
};

use tokio::{
    sync::{Semaphore, watch},
    task::{AbortHandle, JoinSet},
};
use types::Url;

use crate::{
    app::state::SharedAppState,
    repos::rss_feeds::{self, RssFeed, RssFeedId},
};

mod read;

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
            id: value.id,
            url: value.url,
            max_concurrent_requests: value.max_concurrent_requests,
            fetch_interval_seconds: value.fetch_interval_seconds,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RssReaderError {}

pub async fn run_rss_reader(state: SharedAppState) -> Result<(), RssReaderError> {
    let (tx, rx) = watch::channel::<Vec<RssFeedConfig>>(vec![]);

    let mut subtasks = JoinSet::new();

    subtasks.spawn(db_poller(state.clone(), tx));
    subtasks.spawn(rss_reader_manager(state, rx));

    subtasks.join_all().await;

    Ok(())
}

async fn rss_reader_manager(state: SharedAppState, mut rx: watch::Receiver<Vec<RssFeedConfig>>) {
    let mut worker_tasks = JoinSet::new();
    let mut active_configs: HashMap<RssFeedId, (AbortHandle, RssFeedConfig)> = HashMap::new();

    let http_client = reqwest::Client::default();

    let mut iter_ids = HashSet::new();
    loop {
        {
            let configs_ref = rx.borrow_and_update();

            for config in configs_ref.iter() {
                iter_ids.insert(config.id);

                if let Some((abort_handle, old_config)) = active_configs.get(&config.id) {
                    if old_config != config {
                        abort_handle.abort();

                        let new_handle = worker_tasks.spawn(rss_feed_worker(
                            state.clone(),
                            config.clone(),
                            http_client.clone(),
                        ));

                        active_configs.insert(config.id, (new_handle, config.clone()));
                    }
                } else {
                    let new_handle = worker_tasks.spawn(rss_feed_worker(
                        state.clone(),
                        config.clone(),
                        http_client.clone(),
                    ));
                    active_configs.insert(config.id, (new_handle, config.clone()));
                }
            }

            active_configs.retain(|id, (abort_handle, _)| {
                if iter_ids.contains(id) {
                    true
                } else {
                    abort_handle.abort();
                    false
                }
            });

            iter_ids.clear();
        }

        tokio::select! {
            result = rx.changed() => {
                if result.is_err() {
                    break;
                }
            }
            () = state.cancel_token.cancelled() => {
                break;
            }
        }
    }
}

async fn rss_feed_worker(
    state: SharedAppState,
    config: RssFeedConfig,
    http_client: reqwest::Client,
) {
    let interval_duration = Duration::from_secs(u64::from(config.fetch_interval_seconds.get()));

    let mut interval = tokio::time::interval(interval_duration);

    let worker_state = read::SharedRssReaderWorkerState::new(read::RssReaderWorkerState {
        app_state: state,
        http_client,
        request_semaphore: Semaphore::new(usize::from(config.max_concurrent_requests.get())),
        config,
    });

    loop {
        interval.tick().await;
        read::read_rss_feed(worker_state.clone()).await;
    }
}

async fn db_poller(state: SharedAppState, tx: watch::Sender<Vec<RssFeedConfig>>) {
    let interval_duration = Duration::from_secs(10);

    let mut interval = tokio::time::interval(interval_duration);
    let mut last_known_configs = vec![];

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            () = state.cancel_token.cancelled() => {
                break;
            }
        }

        if let Ok(feeds) = rss_feeds::get_rss_feeds(&state.db_pool).await {
            let configs = feeds
                .into_iter()
                .map(RssFeedConfig::from)
                .collect::<Vec<_>>();

            if configs != last_known_configs {
                if tx.send(configs.clone()).is_ok() {
                    last_known_configs = configs;
                } else {
                    break;
                }
            }
        }
    }
}
