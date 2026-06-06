use std::num::{NonZeroU8, NonZeroU32};

use types::Url;

use crate::{
    infra::db::DatabaseExecutor,
    repos::rss_feeds::{RssFeed, RssFeedErrorReason, RssFeedId},
};

/// Retrieves all registered RSS feeds from the database.
///
/// The returned feeds are ordered chronologically by their creation time ([`RssFeed::created_at`]).
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// rows cannot be decoded into [`RssFeed`] instances.
pub async fn get_rss_feeds<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<Vec<RssFeed>> {
    let query = sqlx::query_as(
        "
        SELECT * FROM rss_feeds
        ORDER BY created_at;
        ",
    );

    query.fetch_all(executor).await
}

/// Transitions an RSS feed's status to `healthy`.
///
/// This operation also clears any previously recorded [`RssFeed::error_reason`],
/// enforcing the invariant that a healthy feed cannot have an error attached.
///
/// # Returns
///
/// Returns [`RssFeedId`] if the feed was found and updated, or [`None`]
/// if no feed with the given ID exists.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update fails.
pub async fn mark_rss_feed_as_healthy<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: RssFeedId,
) -> sqlx::Result<Option<RssFeedId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE rss_feeds
        SET
            status = 'healthy',
            error_reason = NULL
        WHERE
            id = ?
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await
}

/// Transitions an RSS feed's status to `error` and records the specific failure reason.
///
/// # Returns
///
/// Returns [`RssFeedId`] if the feed was found and updated, or [`None`]
/// if no feed with the given ID exists.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update fails.
pub async fn mark_rss_feed_as_error<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: RssFeedId,
    error_reason: &RssFeedErrorReason,
) -> sqlx::Result<Option<RssFeedId>> {
    let query = sqlx::query_scalar(
        "
            UPDATE rss_feeds
            SET
                status = 'error',
                error_reason = ?
            WHERE
                id = ?
            RETURNING id;
            ",
    )
    .bind(error_reason)
    .bind(id);

    query.fetch_optional(executor).await
}

/// Registers a newly submitted RSS feed in the database.
///
/// The new feed is automatically assigned a newly generated [`RssFeedId`] and
/// its initial processing status is hardcoded to `'new'`.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database insert fails (e.g., due to a unique
/// constraint violation on the URL) or if the returned row fails to decode.
pub async fn create_rss_feed<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &Url,
    max_concurrent_requests: NonZeroU8,
    fetch_interval_seconds: NonZeroU32,
) -> sqlx::Result<RssFeed> {
    let query = sqlx::query_as(
        "
        INSERT INTO rss_feeds (
            id,
            url,
            max_concurrent_requests,
            fetch_interval_seconds,
            status
        )
        VALUES (?, ?, ?, ?, 'new')
        RETURNING *;
        ",
    )
    .bind(RssFeedId::new())
    .bind(url)
    .bind(max_concurrent_requests)
    .bind(fetch_interval_seconds);

    query.fetch_one(executor).await
}

/// Deletes an RSS feed from the database by its unique identifier.
///
/// # Returns
///
/// Returns [`RssFeedId`] if the feed was found and successfully deleted,
/// or [`None`] if no feed with that ID existed.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database deletion operation fails.
pub async fn delete_rss_feed_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: RssFeedId,
) -> sqlx::Result<Option<RssFeedId>> {
    let query = sqlx::query_scalar(
        "
        DELETE FROM rss_feeds
        WHERE id = ?
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await
}
