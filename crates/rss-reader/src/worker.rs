use std::sync::Arc;

use dom_smoothie::Readability;
use redis::{
    AsyncTypedCommands, ExistenceCheck, RedisResult, SetExpiry, SetOptions,
    aio::MultiplexedConnection,
};
use reqwest::Client;
use tokio::{sync::Semaphore, task::JoinSet};
use tokio_util::sync::CancellationToken;

use crate::config::FeedConfig;

/// A context expected by [`rss_worker`].
pub struct FeedWorkerContext {
    /// RSS feed configuration.
    config: FeedConfig,

    /// Client to make HTTP requests.
    http_client: Client,
    /// Redis connection.
    redis_connection: MultiplexedConnection,

    /// Token to signal a worker when it must exit.
    cancel_token: CancellationToken,
    /// Semaphore to rate limit requests to the RSS feed articles.
    request_semaphore: Semaphore,
}

impl FeedWorkerContext {
    pub fn new(
        config: FeedConfig,
        http_client: Client,
        redis_connection: MultiplexedConnection,
        cancel_token: CancellationToken,
    ) -> Self {
        let request_semaphore = Semaphore::new(config.max_concurrent_requests());

        Self {
            config,
            http_client,
            redis_connection,
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
                tracing::info!("sleeping for {interval:?}")
            }

            tokio::time::sleep(interval).await
        };

        tokio::select! {
            _ = sleep => {},
            _ = context.cancel_token.cancelled() => {
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
        .map(|item| process_rss_feed_item(item, context.clone()));

    tracing::info!("starting {} subtasks", tasks.len());
    JoinSet::from_iter(tasks).join_all().await;
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
    let link_response = context.http_client.get(link).send().await?;
    let html = link_response.text().await?;
    drop(permit);
    tracing::trace!("html fetched");

    tracing::trace!("parsing article");
    // Article parsing is a CPU-bound task so we need to do it on seperate thread
    // where blocking is acceptable. Othewise, it will freeze async executor.
    let article = tokio::task::spawn_blocking(|| parse_article(html)).await??;
    tracing::trace!("article parsed");

    tracing::trace!(article_title = article.title, article_len = article.length);

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

/// Represnts a parsed article from RSS feed.
///
/// It contains the same fields as [`dom_smoothie::Article`] but can be safely
/// shared between threads.
#[derive(Debug, serde::Serialize)]
struct ParsedArticle {
    /// The title
    title: String,
    /// The author
    byline: Option<String>,
    /// The relevant HTML content
    content: String,
    /// The relevant text content
    text_content: String,
    /// The text length
    length: usize,
    /// The excerpt
    excerpt: Option<String>,
    /// The name of the site
    site_name: Option<String>,
    /// The text direction
    dir: Option<String>,
    /// The document language
    lang: Option<String>,
    /// The published time of the document
    published_time: Option<String>,
    /// The modified time of the document
    modified_time: Option<String>,
    /// The image of the document
    image: Option<String>,
    /// The favicon of the document
    favicon: Option<String>,
    /// The metadata's url
    url: Option<String>,
}

impl From<dom_smoothie::Article> for ParsedArticle {
    fn from(value: dom_smoothie::Article) -> Self {
        Self {
            title: value.title,
            byline: value.byline,
            content: value.content.to_string(),
            length: value.length,
            dir: value.dir,
            excerpt: value.excerpt,
            favicon: value.favicon,
            image: value.image,
            lang: value.lang,
            modified_time: value.modified_time,
            published_time: value.published_time,
            site_name: value.site_name,
            text_content: value.text_content.to_string(),
            url: value.url,
        }
    }
}

/// A **CPU-bound** function that returns a [`ParsedArticle`] parsed from the `html` string.
#[tracing::instrument(level = "trace", skip_all, err)]
fn parse_article(html: String) -> Result<ParsedArticle, dom_smoothie::ReadabilityError> {
    let mut readability = Readability::new(html, None, None)?;
    readability.parse().map(Into::into)
}
