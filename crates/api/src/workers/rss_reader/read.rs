use std::sync::Arc;

use dom_smoothie::Readability;
use tokio::{sync::Semaphore, task::JoinSet};
use types::{NonEmpty, TrimmedString, nonempty_trimmed_string};

use crate::{
    app::{
        events::{NotificationData, NotificationString, RefetchTriggerType},
        state::SharedAppState,
    },
    repos::{
        articles::{self, FromDomSmoothieArticleError, ReadabilityArticle},
        rss_feeds::{self, RssFeedErrorReason},
    },
    workers::rss_reader::RssFeedConfig,
};

pub struct RssReaderWorkerState {
    pub app_state: SharedAppState,
    pub http_client: reqwest::Client,
    pub config: RssFeedConfig,
    pub request_semaphore: Semaphore,
}

pub type SharedRssReaderWorkerState = Arc<RssReaderWorkerState>;

#[tracing::instrument(level = "trace", skip_all)]
#[allow(clippy::too_many_lines)]
pub async fn read_rss_feed(state: SharedRssReaderWorkerState) {
    match get_rss_channel(&state).await {
        Ok(rss_channel) => {
            let tasks = rss_channel
                .items
                .into_iter()
                .map(|item| process_rss_feed_item(state.clone(), item))
                .collect::<JoinSet<_>>();

            let results = tasks.join_all().await;

            if !results.is_empty() && results.iter().all(Result::is_err) {
                _ = rss_feeds::mark_rss_feed_as_error(
                    &state.app_state.db_pool,
                    state.config.id,
                    &RssFeedErrorReason(nonempty_trimmed_string!(
                        "No articles were parsed successfully during the latest fetch"
                    )),
                )
                .await;

                state
                    .app_state
                    .app_events_broadcaster
                    .send_refetch_trigger(RefetchTriggerType::Articles);

                state.app_state.app_events_broadcaster.send_notification(
                    NotificationData::error(NotificationString(nonempty_trimmed_string!(
                        "RSS feed error"
                    )))
                    .with_description(NotificationString::new(
                        "RSS was read but no articles were parsed successfully",
                    )),
                );
            }

            let feed_articles = results
                .into_iter()
                .filter_map(Result::ok)
                .flatten()
                .collect::<Vec<_>>();

            #[allow(clippy::collapsible_if)]
            if let Some(feed_articles) = NonEmpty::new(feed_articles) {
                match articles::create_articles_bulk(
                    &state.app_state.db_pool,
                    &feed_articles,
                    state.config.id,
                )
                .await
                {
                    Ok(query_result) => {
                        _ = rss_feeds::mark_rss_feed_as_healthy(
                            &state.app_state.db_pool,
                            state.config.id,
                        )
                        .await;

                        let new_count = query_result.rows_affected();
                        if new_count > 0 {
                            state
                                .app_state
                                .app_events_broadcaster
                                .send_refetch_trigger(RefetchTriggerType::Articles);

                            state.app_state.app_events_broadcaster.send_notification(
                                NotificationData::info(NotificationString(
                                    nonempty_trimmed_string!("New articles from RSS feed"),
                                ))
                                .with_description(
                                    NotificationString::new(format!(
                                        "{new_count} article(s) created",
                                    )),
                                ),
                            );
                        }
                    }
                    Err(error) => {
                        _ = rss_feeds::mark_rss_feed_as_error(
                            &state.app_state.db_pool,
                            state.config.id,
                            &RssFeedErrorReason::new(TrimmedString::from(error.to_string()))
                                .unwrap_or_else(|| {
                                    RssFeedErrorReason(nonempty_trimmed_string!(
                                        "Some error occurred during the latest RSS feed read"
                                    ))
                                }),
                        )
                        .await;

                        state.app_state.app_events_broadcaster.send_notification(
                            NotificationData::error(NotificationString(nonempty_trimmed_string!(
                                "RSS feed error"
                            )))
                            .with_description(
                                NotificationString::new(
                                    "Something went wrong during latest RSS feed read",
                                ),
                            ),
                        );
                    }
                }
            };
        }
        Err(error) => {
            // error logged by `tracing::instrument`
            _ = rss_feeds::mark_rss_feed_as_error(
                &state.app_state.db_pool,
                state.config.id,
                &RssFeedErrorReason::new(TrimmedString::from(error.to_string())).unwrap_or_else(
                    || {
                        RssFeedErrorReason(nonempty_trimmed_string!(
                            "Some error occurred during the latest RSS feed read"
                        ))
                    },
                ),
            )
            .await;

            state.app_state.app_events_broadcaster.send_notification(
                NotificationData::error(NotificationString(nonempty_trimmed_string!(
                    "RSS feed error"
                )))
                .with_description(NotificationString::new(
                    "Something went wrong during latest RSS feed read",
                )),
            );
        }
    }

    state
        .app_state
        .app_events_broadcaster
        .send_refetch_trigger(RefetchTriggerType::RssFeeds);
}

#[derive(Debug, thiserror::Error)]
enum ProcessFeedItemError {
    #[error("article link is missing")]
    MissingLink,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Permit(#[from] tokio::sync::AcquireError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Parsing(#[from] ParseArticleError),
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
}

#[tracing::instrument(level = "trace", skip_all, fields(link = item.link()), err(Debug))]
async fn process_rss_feed_item(
    state: SharedRssReaderWorkerState,
    item: rss::Item,
) -> Result<Option<ReadabilityArticle>, ProcessFeedItemError> {
    let Some(link) = item.link else {
        tracing::warn!("no article link to process");
        return Err(ProcessFeedItemError::MissingLink);
    };

    if articles::check_article_exists_by_url(&state.app_state.db_pool, &link).await? {
        tracing::trace!("article already exists");
        return Ok(None);
    }

    tracing::trace!("fetching article html");
    let permit = state.request_semaphore.acquire().await?;
    let link_response = state
        .http_client
        .get(&link)
        .send()
        .await?
        .error_for_status()?;
    let html = link_response.text().await?;
    drop(permit);
    tracing::trace!("html fetched");

    tracing::trace!("parsing article");
    // Article parsing is a CPU-bound task so we need to do it on seperate thread
    // where blocking is acceptable. Othewise, it will freeze async executor.
    let article = tokio::task::spawn_blocking(|| parse_article(html, link)).await??;
    tracing::trace!("article parsed");

    tracing::trace!(
        article_title = article.title.as_str(),
        article_len = article.length
    );

    Ok(Some(article))
}

#[derive(Debug, thiserror::Error)]
enum ParseArticleError {
    #[error(transparent)]
    ReadabilityError(#[from] dom_smoothie::ReadabilityError),
    #[error(transparent)]
    Convert(#[from] FromDomSmoothieArticleError),
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
fn parse_article(html: String, link: String) -> Result<ReadabilityArticle, ParseArticleError> {
    let mut readability = Readability::new(html, Some(&link), None)?;

    let mut article = readability.parse()?;
    if article.url.is_none() {
        article.url = Some(link);
    }

    ReadabilityArticle::try_from(article).map_err(ParseArticleError::from)
}

#[derive(Debug, thiserror::Error)]
enum RssChannelError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Rss(#[from] rss::Error),
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
async fn get_rss_channel(
    state: &SharedRssReaderWorkerState,
) -> Result<rss::Channel, RssChannelError> {
    tracing::trace!("fetching rss");
    let feed_response = state
        .http_client
        .get(state.config.url.as_str())
        .send()
        .await?;
    let bytes = feed_response.bytes().await?;
    tracing::trace!("rss fetched");

    let channel = rss::Channel::read_from(&bytes[..])?;
    tracing::info!(channel_title = channel.title);

    Ok(channel)
}
