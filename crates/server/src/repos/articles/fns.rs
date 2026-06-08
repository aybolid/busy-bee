use sqlx::QueryBuilder;
use types::NonEmpty;

use crate::{
    infra::db::{DatabaseExecutor, DatabaseQueryResult},
    repos::{
        Pagination,
        articles::{
            Article, ArticleErrorReason, ArticleId, ReadabilityArticle, types::ArticleStats,
        },
        rss_feeds::RssFeedId,
    },
};

/// Retrieves aggregate statistics for all articles in the database.
///
/// This performs a single table scan to calculate the total number of articles
/// alongside a breakdown of counts for each specific [`ArticleStatus`].
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn get_article_stats<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<ArticleStats> {
    let query = sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
        "
        SELECT
            COUNT(*),
            COALESCE(SUM(CASE WHEN status = 'new' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'error' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'processed' THEN 1 ELSE 0 END), 0)
        FROM articles;
        ",
    );

    let row = query.fetch_one(executor).await?;
    tracing::trace!("got articles stats");

    Ok(ArticleStats {
        total: row.0 as usize,
        new: row.1 as usize,
        pending: row.2 as usize,
        error: row.3 as usize,
        processed: row.4 as usize,
    })
}

/// Transitions an article's status to `pending`.
///
/// This indicates the article is currently being processed. It clears any previous
/// `error_reason`. To prevent race conditions, the update is only applied if
/// the article is not *already* in a `pending` state.
///
/// # Returns
///
/// Returns [`ArticleId`] if the article was found and updated, or [`None`]
/// if no article matched the criteria.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update fails.
#[tracing::instrument(level = "trace", skip_all, fields(article_id = %id.as_hyphenated()), err(Debug))]
pub async fn mark_article_as_pending<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
        SET
            status = 'pending',
            error_reason = NULL
        WHERE
            id = ? AND status != 'pending'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "article marked as pending"
            } else {
                "article to mark not found"
            }
        );
    })
}

/// Transitions an article's status to `error` and records the specific failure reason.
///
/// # Returns
///
/// Returns [`ArticleId`] if the article was found and updated, or [`None`]
/// if no article with the given ID exists.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update fails.
#[tracing::instrument(level = "trace", skip_all, fields(article_id = %id.as_hyphenated(), reason = error_reason.as_str()), err(Debug))]
pub async fn mark_article_as_error<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
    error_reason: &ArticleErrorReason,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
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

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "article marked as error"
            } else {
                "article to mark not found"
            }
        );
    })
}

/// Transitions an article's status to `processed`.
///
/// Enforces a strict state transition: an article can only be marked as processed
/// if its current status is `pending`. It also clears any lingering `error_reason`.
///
/// # Returns
///
/// Returns [`ArticleId`] if the article was found and updated, or [`None`]
/// if the article does not exist or was not in a `pending` state.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database update fails.
#[tracing::instrument(level = "trace", skip_all, fields(article_id = %id.as_hyphenated()), err(Debug))]
pub async fn mark_article_as_processed<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
        SET
            status = 'processed',
            error_reason = NULL
        WHERE
            id = ? AND status = 'pending'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "article marked as processed"
            } else {
                "article to mark not found"
            }
        );
    })
}

/// Counts the total number of articles stored in the database.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn count_articles<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<usize> {
    let query = sqlx::query_scalar("SELECT COUNT(*) FROM articles;");

    query
        .fetch_one(executor)
        .await
        .map(|count: i64| count as usize)
        .inspect(|count| tracing::trace!(count, "got articles count"))
}

/// Retrieves a paginated list of articles.
///
/// Articles are ordered chronologically by their creation date, with the newest first.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or if the resulting
/// rows cannot be decoded into [`Article`] instances.
#[tracing::instrument(level = "trace", skip_all, err(Debug))]
#[allow(clippy::cast_possible_wrap)]
pub async fn get_articles<'c>(
    executor: impl DatabaseExecutor<'c>,
    pagination: Pagination,
) -> sqlx::Result<Vec<Article>> {
    let (limit, offset) = pagination.as_limit_and_offset();

    let query = sqlx::query_as(
        "
        SELECT * FROM articles
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?;
        ",
    )
    .bind(limit)
    .bind(offset);

    query
        .fetch_all(executor)
        .await
        .inspect(|_| tracing::trace!("articles fetched from db"))
}

/// Fetches a single article by its unique identifier.
///
/// # Returns
///
/// Returns the [`Article`] if found, or [`None`] if no article matches the given ID.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails or decoding fails.
#[tracing::instrument(level = "trace", skip_all, fields(article_id = %id.as_hyphenated()), err(Debug))]
pub async fn get_article_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<Article>> {
    let query = sqlx::query_as("SELECT * FROM articles WHERE id = ?;").bind(id);

    query.fetch_optional(executor).await.inspect(|article| {
        tracing::trace!(
            "{}",
            if article.is_some() {
                "article fetched from db"
            } else {
                "article not found"
            }
        );
    })
}

/// Deletes an article from the database by its unique identifier.
///
/// As a safety constraint, articles currently in a `pending` state (actively
/// being processed) cannot be deleted.
///
/// # Returns
///
/// Returns [`ArticleId`] if the article was successfully deleted, or [`None`]
/// if no such article existed or if it was currently `pending`.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database deletion operation fails.
#[tracing::instrument(level = "trace", skip_all, fields(article_id = %id.as_hyphenated()), err(Debug))]
pub async fn delete_article_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        DELETE FROM articles
        WHERE
            id = ? AND status != 'pending'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await.inspect(|id| {
        tracing::trace!(
            "{}",
            if id.is_some() {
                "article deleted"
            } else {
                "article to delete not found"
            }
        );
    })
}

/// Performs a bulk insert of multiple articles into the database.
///
/// This utilizes a [`QueryBuilder`] to construct a single `INSERT` statement.
/// If an article with the same `url` already exists, it triggers an
/// `ON CONFLICT DO NOTHING` clause, silently ignoring the duplicate to maintain uniqueness.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the batch execution fails.
#[tracing::instrument(level = "trace", skip_all, fields(rss_feed_id = %rss_feed_id.as_hyphenated()), err(Debug))]
pub async fn create_articles_bulk<'c>(
    executor: impl DatabaseExecutor<'c>,
    articles: &NonEmpty<Vec<ReadabilityArticle>>,
    rss_feed_id: RssFeedId,
) -> sqlx::Result<DatabaseQueryResult> {
    let mut query_builder = QueryBuilder::new(
        "
        INSERT INTO articles (
            id, status, rss_feed_id, title, byline,
            content, text_content, length,
            excerpt, site_name, dir,
            lang, published_time, modified_time,
            image, favicon, url
        )
        ",
    );

    query_builder.push_values(articles.iter(), |mut b, article| {
        b.push_bind(ArticleId::new())
            .push_bind("new")
            .push_bind(rss_feed_id)
            .push_bind(&article.title)
            .push_bind(&article.byline)
            .push_bind(&article.content)
            .push_bind(&article.text_content)
            .push_bind(article.length)
            .push_bind(&article.excerpt)
            .push_bind(&article.site_name)
            .push_bind(article.dir)
            .push_bind(&article.lang)
            .push_bind(article.published_time)
            .push_bind(article.modified_time)
            .push_bind(&article.image)
            .push_bind(&article.favicon)
            .push_bind(&article.url);
    });

    query_builder.push(" ON CONFLICT (url) DO NOTHING;");

    let query = query_builder.build();

    query
        .execute(executor)
        .await
        .inspect(|result| tracing::trace!(count = result.rows_affected(), "articles created"))
}

/// Checks whether an article with the specified canonical URL already exists.
///
/// # Returns
///
/// Returns `true` if an article with the matching URL exists, otherwise `false`.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if the database query fails.
#[tracing::instrument(level = "trace", skip_all, fields(url = %url), err(Debug))]
pub async fn check_article_exists_by_url<'c>(
    executor: impl DatabaseExecutor<'c>,
    url: &str,
) -> sqlx::Result<bool> {
    let query =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM articles WHERE url = ?);").bind(url);

    query
        .fetch_one(executor)
        .await
        .inspect(|exists| tracing::trace!(exists, "checked article by url"))
}
