use std::num::{NonZeroU8, NonZeroU32};

use types::Url;

use crate::repos::rss_feeds::{RssFeed, RssFeedId};

/// Represents the configuration required to fetch and process an RSS feed.
///
/// This struct contains the essential parameters needed by a background worker
/// or fetcher service to safely poll an RSS feed at the correct intervals
/// without exceeding concurrency limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RssFeedConfig {
    /// The unique identifier for this RSS feed.
    pub id: RssFeedId,
    /// The target endpoint URL where the RSS feed is hosted.
    pub url: Url,
    /// The maximum number of concurrent HTTP requests allowed for this specific feed.
    pub max_concurrent_requests: NonZeroU8,
    /// The interval, in seconds, at which the feed should be polled for updates.
    pub fetch_interval_seconds: NonZeroU32,
}

/// Converts a full [`RssFeed`] domain or database model into an operational [`RssFeedConfig`].
///
/// This is useful for extracting just the configuration parameters needed by the
/// fetching logic, leaving behind any extra metadata stored in the `RssFeed` struct.
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
