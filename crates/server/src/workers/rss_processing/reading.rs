use std::sync::Arc;

use dom_smoothie::Readability;
use tokio::{sync::Semaphore, task::JoinSet};
use types::{NonEmpty, nonempty_trimmed_string};

use crate::{
    app::{
        events::{NotificationData, NotificationString, RefetchTriggerType},
        state::SharedAppState,
    },
    repos::{
        articles::{self, FromDomSmoothieArticleError, ReadabilityArticle},
        rss_feeds::{self, RssFeedErrorReason},
    },
    workers::rss_processing::config::RssFeedConfig,
};

/// The execution context for an individual RSS feed reader worker.
///
/// This struct holds the necessary state, configuration, and shared resources
/// required to fetch, parse, and store articles from a specific RSS feed.
pub struct RssReaderState {
    /// Shared application state.
    pub app_state: SharedAppState,
    /// An asynchronous HTTP client reused across feed and article requests.
    pub http_client: reqwest::Client,
    /// The configuration for this specific RSS feed (URL, limits, intervals).
    pub config: RssFeedConfig,
    /// A concurrency primitive to limit the number of simultaneous HTTP requests
    /// made when fetching individual articles from this feed.
    pub request_semaphore: Semaphore,
}

/// A thread-safe, reference-counted pointer to the [`RssReaderState`].
pub type SharedRssReaderState = Arc<RssReaderState>;

/// Orchestrates the fetching, parsing, and storing of an entire RSS feed.
///
/// **Workflow:**
/// 1. Fetches the RSS channel XML.
/// 2. Spawns asynchronous tasks to process each RSS item concurrently (respecting the semaphore).
/// 3. Waits for all item tasks to complete using a `JoinSet`.
/// 4. If all items fail, marks the feed state as errored in the database.
/// 5. Otherwise, bulk-inserts successfully parsed articles into the database.
/// 6. Marks the feed state as healthy and broadcasts appropriate notifications.
#[tracing::instrument(skip_all, fields(id = %state.config.id.as_hyphenated()))]
pub async fn read_rss_feed(state: SharedRssReaderState) {
    match get_rss_channel(&state).await {
        Ok(channel) => {
            let read_tasks: JoinSet<_> = channel
                .items
                .into_iter()
                .map(|item| read_rss_feed_item(state.clone(), item))
                .collect();

            let results = read_tasks.join_all().await;

            // If we attempted to process articles but every single one failed, flag the feed.
            if !results.is_empty() && results.iter().all(Result::is_err) {
                _ = rss_feeds::mark_rss_feed_as_error(
                    &state.app_state.db_pool,
                    state.config.id,
                    &RssFeedErrorReason(nonempty_trimmed_string!(
                        "No articles were parsed successfully during the latest fetch"
                    )),
                )
                .await;

                tracing::warn!("no articles were parsed successfully during the latest fetch");

                broadcast_fail(&state);

                return;
            }

            let readability_articles: Vec<_> = results
                .into_iter()
                .filter_map(Result::ok)
                .flatten()
                .collect();

            let Some(readability_articles) = NonEmpty::new(readability_articles) else {
                return; // Should be unreachable given the logic above, but guards against empty lists
            };

            let result = articles::create_articles_bulk(
                &state.app_state.db_pool,
                &readability_articles,
                state.config.id,
            )
            .await;

            match result {
                Ok(query_result) => {
                    tracing::trace!(
                        new_articles_len = query_result.rows_affected(),
                        "new articles created"
                    );

                    _ = rss_feeds::mark_rss_feed_as_healthy(
                        &state.app_state.db_pool,
                        state.config.id,
                    )
                    .await;

                    broadcast_success(&state);
                }
                Err(error) => {
                    tracing::error!(%error);

                    _ = rss_feeds::mark_rss_feed_as_error(
                        &state.app_state.db_pool,
                        state.config.id,
                        &RssFeedErrorReason::from(&error),
                    )
                    .await;

                    broadcast_fail(&state);
                }
            }
        }
        Err(error) => {
            tracing::error!(%error);

            _ = rss_feeds::mark_rss_feed_as_error(
                &state.app_state.db_pool,
                state.config.id,
                &RssFeedErrorReason::from(&error),
            )
            .await;

            broadcast_fail(&state);
        }
    }
}

/// Errors that can occur when fetching and parsing the top-level RSS channel XML.
#[derive(Debug, thiserror::Error)]
enum RssChannelError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Rss(#[from] rss::Error),
}

impl From<&RssChannelError> for RssFeedErrorReason {
    fn from(value: &RssChannelError) -> Self {
        Self::new(value.to_string())
            .expect("rss channel error string should not be an empty string")
    }
}

/// Errors that can occur while processing an individual RSS feed item.
#[derive(Debug, thiserror::Error)]
enum ReadFeedItemError {
    #[error("article link is missing")]
    MissingLink,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Permit(#[from] tokio::sync::AcquireError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Parsing(#[from] ParseReadabilityArticleError),
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
}

/// Processes a single RSS feed item.
///
/// **Workflow:**
/// 1. Validates the item has a target URL.
/// 2. Checks the database to skip articles that have already been processed.
/// 3. Acquires a permit from the concurrency semaphore.
/// 4. Downloads the raw HTML of the article.
/// 5. Offloads the CPU-intensive Readability parsing to a blocking thread pool.
///
/// # Errors
/// Returns a [`ReadFeedItemError`] if network requests fail, database queries fail,
/// or HTML parsing fails.
#[tracing::instrument(skip_all, fields(link = ?item.link))]
async fn read_rss_feed_item(
    state: SharedRssReaderState,
    item: rss::Item,
) -> Result<Option<ReadabilityArticle>, ReadFeedItemError> {
    let Some(link) = item.link else {
        return Err(ReadFeedItemError::MissingLink);
    };

    // Prevent duplicate processing by checking the DB first
    // FIXME: TOCTOU
    if articles::check_article_exists_by_url(&state.app_state.db_pool, &link).await? {
        return Ok(None);
    }

    let permit = state.request_semaphore.acquire().await?;
    let link_response = state
        .http_client
        .get(&link)
        .send()
        .await?
        .error_for_status()?;
    let html = link_response.text().await?;
    // Release the concurrency permit as early as possible before CPU-bound work
    drop(permit);

    // Article parsing is a CPU-bound task so we need to do it on a separate thread
    // where blocking is acceptable. Otherwise, it will freeze the async executor.
    let article = tokio::task::spawn_blocking(|| parse_readability_article(html, link)).await??;

    Ok(Some(article))
}

/// Errors that can occur during the `dom_smoothie` Readability parsing process.
#[derive(Debug, thiserror::Error)]
enum ParseReadabilityArticleError {
    #[error(transparent)]
    ReadabilityError(#[from] dom_smoothie::ReadabilityError),
    #[error(transparent)]
    Convert(#[from] FromDomSmoothieArticleError),
}

/// Parses raw HTML into a clean, readable article format.
///
/// This uses the `dom_smoothie` crate (a Rust port of Mozilla's Readability library)
/// to strip out navigation, ads, and boilerplate, leaving only the core content.
///
/// # Errors
/// Returns a [`ParseReadabilityArticleError`] if the HTML cannot be processed
/// or if it fails conversion into the internal domain model.
#[tracing::instrument(skip_all, err(Debug))]
fn parse_readability_article(
    html: String,
    link: String,
) -> Result<ReadabilityArticle, ParseReadabilityArticleError> {
    let mut readability = Readability::new(html, Some(&link), None)?;

    let mut article = readability.parse()?;

    // Ensure the article model retains the original source URL
    if article.url.is_none() {
        article.url = Some(link);
    }

    ReadabilityArticle::try_from(article).map_err(ParseReadabilityArticleError::from)
}

/// Fetches and parses the top-level XML of an RSS feed.
///
/// # Errors
/// Returns an [`RssChannelError`] if the network request fails or if the response
/// is not valid RSS XML.
#[tracing::instrument(skip_all, err(Debug))]
async fn get_rss_channel(state: &SharedRssReaderState) -> Result<rss::Channel, RssChannelError> {
    let feed_response = state
        .http_client
        .get(state.config.url.as_str())
        .send()
        .await?;
    let bytes = feed_response.bytes().await?;

    let channel = rss::Channel::read_from(&bytes[..])?;

    tracing::trace!(channel_title = channel.title);

    Ok(channel)
}

/// Broadcasts a failure notification to the client application.
///
/// This triggers an error toast/notification and asks the client to refetch the
/// RSS feeds list to observe the updated error state.
fn broadcast_fail(state: &SharedRssReaderState) {
    let notification = NotificationData::error(NotificationString(nonempty_trimmed_string!(
        "RSS feed error"
    )))
    .with_description(NotificationString::new(
        "Something went wrong during latest RSS feed read",
    ));

    state
        .app_state
        .app_events_broadcaster
        .send_notification(notification)
        .send_refetch_trigger(RefetchTriggerType::RssFeeds);
}

/// Broadcasts a success notification to the client application.
///
/// This triggers an info toast/notification and asks the client to refetch both
/// the RSS feeds list and the Articles list to display the newly fetched content.
fn broadcast_success(state: &SharedRssReaderState) {
    let notification = NotificationData::info(NotificationString(nonempty_trimmed_string!(
        "New articles from RSS feed"
    )))
    .with_description(NotificationString::new("New articles were added"));

    state
        .app_state
        .app_events_broadcaster
        .send_notification(notification)
        .send_refetch_triggers([RefetchTriggerType::RssFeeds, RefetchTriggerType::Articles]);
}
