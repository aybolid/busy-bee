use std::sync::Arc;

use dom_smoothie::Readability;
use lapin::{BasicProperties, Channel, Confirmation, options::BasicPublishOptions};
use redis::{
    AsyncTypedCommands, ExistenceCheck, RedisResult, SetExpiry, SetOptions,
    aio::MultiplexedConnection,
};
use reqwest::Client;
use tokio::{sync::Semaphore, task::JoinSet};
use tokio_util::sync::CancellationToken;

use crate::internal::config::FeedConfig;

pub struct FeedWorkerContext {
    config: FeedConfig,

    http_client: Client,
    redis_connection: MultiplexedConnection,
    amqp_channel: Channel,
    amqp_queue: Arc<String>,

    cancel_token: CancellationToken,
    request_semaphore: Semaphore,
}

impl FeedWorkerContext {
    pub fn new(
        config: FeedConfig,
        http_client: Client,
        redis_connection: MultiplexedConnection,
        amqp_channel: Channel,
        amqp_queue: Arc<String>,
        cancel_token: CancellationToken,
    ) -> Self {
        let request_semaphore = Semaphore::new(config.max_concurrent_requests());

        Self {
            config,
            http_client,
            redis_connection,
            amqp_channel,
            amqp_queue,
            cancel_token,
            request_semaphore,
        }
    }
}

#[tracing::instrument(level = "trace", skip_all, fields(feed_url = context.config.url()))]
pub async fn rss_worker(context: FeedWorkerContext) {
    tracing::info!("running");

    let context = Arc::new(context);

    loop {
        _ = process_rss_feed(&context).await;

        let sleep = async {
            let interval = context.config.interval();
            if !context.cancel_token.is_cancelled() {
                tracing::info!("sleeping for {interval:?}");
            }

            tokio::time::sleep(interval).await;
        };

        tokio::select! {
            () = sleep => {},
            () = context.cancel_token.cancelled() => {
                tracing::trace!("shutdown signal received");
                break;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ProcessFeedError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    RssError(#[from] rss::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_rss_feed(context: &Arc<FeedWorkerContext>) -> Result<(), ProcessFeedError> {
    tracing::trace!("fetching rss");
    let feed_response = context.http_client.get(context.config.url()).send().await?;
    let bytes = feed_response.bytes().await?;
    tracing::trace!("rss fetched");

    let channel = rss::Channel::read_from(&bytes[..])?;
    tracing::info!(channel_title = channel.title);
    let tasks = channel
        .items
        .into_iter()
        .map(|item| process_rss_feed_item(item, context.clone()))
        .collect::<JoinSet<_>>();

    tracing::info!("starting {} subtasks", tasks.len());
    tasks.join_all().await;
    tracing::info!("all subtasks finished");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
enum ProcessFeedItemError {
    #[error(transparent)]
    PermitError(#[from] tokio::sync::AcquireError),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ReadabilityError(#[from] dom_smoothie::ReadabilityError),
    #[error(transparent)]
    TaskError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[tracing::instrument(level = "trace", skip_all, fields(link = item.link), err)]
async fn process_rss_feed_item(
    item: rss::Item,
    context: Arc<FeedWorkerContext>,
) -> Result<(), ProcessFeedItemError> {
    let Some(link) = item.link() else {
        tracing::warn!("no article link to process");
        return Ok(());
    };

    let cache_response = cache_rss_item_by_link(context.redis_connection.clone(), link).await?;
    if cache_response == CacheResponse::AlreadyCached {
        tracing::trace!("article was processed before. skipping");
        return Ok(());
    }

    tracing::trace!("fetching article html");
    let permit = context.request_semaphore.acquire().await?;
    let link_response = context
        .http_client
        .get(link)
        .send()
        .await?
        .error_for_status()?;
    let html = link_response.text().await?;
    drop(permit);
    tracing::trace!("html fetched");

    tracing::trace!("parsing article");
    // Article parsing is a CPU-bound task so we need to do it on seperate thread
    // where blocking is acceptable. Othewise, it will freeze async executor.
    let owned_link = link.to_owned();
    let article = tokio::task::spawn_blocking(|| parse_article(html, owned_link)).await??;
    tracing::trace!("article parsed");

    tracing::trace!(article_title = article.title, article_len = article.length);

    tracing::trace!("serializing article json");
    let article_json_bytes = serde_json::to_vec(&article)?;
    tracing::trace!("json serialized");

    publish_article(
        &context.amqp_channel,
        &context.amqp_queue,
        &article_json_bytes,
    )
    .await?;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all, fields(queue), err)]
async fn publish_article(channel: &Channel, queue: &str, payload: &[u8]) -> lapin::Result<()> {
    let confirmation = channel
        .basic_publish(
            "".into(), // Default exchange
            queue.into(),
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await?
        .await?;
    tracing::trace!(?confirmation, "article message sent");
    assert_eq!(confirmation, Confirmation::NotRequested);

    Ok(())
}

/// Represents a return value of [`cache_rss_item_by_link`].
///
/// Although, a `bool` would work here it's nice to have a more descriptive
/// return value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CacheResponse {
    AddedToCache,
    AlreadyCached,
}

#[tracing::instrument(level = "trace", skip(redis_connection), ret, err)]
async fn cache_rss_item_by_link(
    mut redis_connection: MultiplexedConnection,
    link: &str,
) -> RedisResult<CacheResponse> {
    let key = format!("seen_article:{link}");
    let options = SetOptions::default()
        .conditional_set(ExistenceCheck::NX)
        .with_expiration(SetExpiry::EX(7 * 24 * 3600));

    redis_connection
        .set_options(key, "1", options)
        .await
        .map(|o| {
            if o.is_some() {
                CacheResponse::AddedToCache
            } else {
                CacheResponse::AlreadyCached
            }
        })
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn parse_article(
    html: String,
    link: String,
) -> Result<rss_reader::ParsedArticle, dom_smoothie::ReadabilityError> {
    let mut readability = Readability::new(html, Some(&link), None)?;
    readability.parse().map(Into::into)
}
