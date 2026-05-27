use std::num::NonZeroU8;

use chrono::{DateTime, Utc};
use types::{NonEmpty, TrimmedString};
use uuid::Uuid;

use crate::infra::db::{DatabaseExecutor, DatabaseQueryResult};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ArticleId(Uuid);

impl ArticleId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

pub type ArticleTitle = NonEmpty<TrimmedString>;
pub type ArticleByLine = NonEmpty<TrimmedString>;
pub type ArticleContent = NonEmpty<TrimmedString>;
pub type ArticleTextContent = NonEmpty<TrimmedString>;
pub type ArticleExcerpt = NonEmpty<TrimmedString>;
pub type ArticleSiteName = NonEmpty<TrimmedString>;
pub type ArticleLang = NonEmpty<TrimmedString>;
pub type ArticleImage = NonEmpty<TrimmedString>;
pub type ArticleFavicon = NonEmpty<TrimmedString>;
pub type ArticleUrl = NonEmpty<TrimmedString>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum TextDirection {
    Rtl,
    Ltr,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown text direction: {0}")]
pub struct UnknownTextDirection(String);

impl std::str::FromStr for TextDirection {
    type Err = UnknownTextDirection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rtl" => Ok(Self::Rtl),
            "ltr" => Ok(Self::Ltr),
            s => Err(UnknownTextDirection(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum ArticleStatus {
    New,
    Pending,
    Error,
    Processed,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Article {
    id: ArticleId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    status: ArticleStatus,

    title: ArticleTitle,
    byline: Option<ArticleByLine>,
    content: ArticleContent,
    text_content: ArticleTextContent,
    length: i64, // Using i64 so it can be stored in Sqlite
    excerpt: Option<ArticleExcerpt>,
    site_name: Option<ArticleSiteName>,
    dir: Option<TextDirection>,
    lang: Option<ArticleLang>,
    published_time: Option<DateTime<Utc>>,
    modified_time: Option<DateTime<Utc>>,
    image: Option<ArticleImage>,
    favicon: Option<ArticleFavicon>,
    url: Option<ArticleUrl>,
}

impl Article {
    pub fn id(&self) -> ArticleId {
        self.id
    }

    pub fn text_content(&self) -> &TrimmedString {
        &self.text_content
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FromParsedArticeError {
    #[error(transparent)]
    DateParseError(#[from] chrono::ParseError),
    #[error("got empty string in the {0} field")]
    EmptyString(&'static str),
    #[error(transparent)]
    LengthError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    UnknownTextDirection(#[from] UnknownTextDirection),
}

impl TryFrom<rss_reader::ParsedArticle> for Article {
    type Error = FromParsedArticeError;

    fn try_from(value: rss_reader::ParsedArticle) -> Result<Self, Self::Error> {
        let now = Utc::now();

        let title = ArticleTitle::try_new(TrimmedString::from(value.title))
            .map_err(|_| FromParsedArticeError::EmptyString("title"))?;

        let byline = value
            .byline
            .and_then(|s| ArticleByLine::new(TrimmedString::from(s)));

        let content = ArticleTitle::try_new(TrimmedString::from(value.content))
            .map_err(|_| FromParsedArticeError::EmptyString("content"))?;
        let text_content = ArticleTitle::try_new(TrimmedString::from(value.text_content))
            .map_err(|_| FromParsedArticeError::EmptyString("text_content"))?;

        let length = i64::try_from(text_content.chars().count())?;

        let excerpt = value
            .excerpt
            .and_then(|s| ArticleExcerpt::new(TrimmedString::from(s)));

        let site_name = value
            .site_name
            .and_then(|s| ArticleSiteName::new(TrimmedString::from(s)));

        let dir = value.dir.map(|s| s.parse()).transpose()?;

        let lang = value
            .lang
            .and_then(|s| ArticleLang::new(TrimmedString::from(s)));

        let published_time = value.published_time.map(|s| s.parse()).transpose()?;
        let modified_time = value.modified_time.map(|s| s.parse()).transpose()?;

        let image = value
            .image
            .and_then(|s| ArticleImage::new(TrimmedString::from(s)));

        let favicon = value
            .favicon
            .and_then(|s| ArticleFavicon::new(TrimmedString::from(s)));

        let url = value
            .url
            .and_then(|s| ArticleUrl::new(TrimmedString::from(s)));

        Ok(Self {
            id: ArticleId::new(),
            created_at: now,
            updated_at: now,

            status: ArticleStatus::New,

            title,
            byline,
            content,
            text_content,
            length,
            excerpt,
            site_name,
            dir,
            lang,
            published_time,
            modified_time,
            image,
            favicon,
            url,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct ArticleStats {
    total: usize,
    new: usize,
    pending: usize,
    error: usize,
    processed: usize,
}

#[tracing::instrument(level = "trace", skip_all, err, ret)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn get_article_stats<'c>(
    executor: impl DatabaseExecutor<'c>,
) -> sqlx::Result<ArticleStats> {
    let row: (i64, i64, i64, i64, i64) = sqlx::query_as(
        "
        SELECT
            COUNT(*),
            COALESCE(SUM(CASE WHEN status = 'new' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'error' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'processed' THEN 1 ELSE 0 END), 0)
        FROM articles;
        ",
    )
    .fetch_one(executor)
    .await?;

    Ok(ArticleStats {
        total: row.0 as usize,
        new: row.1 as usize,
        pending: row.2 as usize,
        error: row.3 as usize,
        processed: row.4 as usize,
    })
}

#[tracing::instrument(level = "trace", skip(executor), err, ret)]
pub async fn mark_article_as_pending<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
        SET
            status = 'pending'
        WHERE
            id = ? AND status != 'pending'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip(executor), err, ret)]
pub async fn mark_article_as_error<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
        SET
            status = 'error'
        WHERE
            id = ? AND status != 'error'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip(executor), err, ret)]
pub async fn mark_article_as_processed<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<ArticleId>> {
    let query = sqlx::query_scalar(
        "
        UPDATE articles
        SET
            status = 'processed'
        WHERE
            id = ? AND status == 'pending'
        RETURNING id;
        ",
    )
    .bind(id);

    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip_all, err, ret)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub async fn count_articles<'c>(executor: impl DatabaseExecutor<'c>) -> sqlx::Result<usize> {
    let query = sqlx::query_scalar("SELECT COUNT(*) FROM articles;");
    query
        .fetch_one(executor)
        .await
        .map(|count: i64| count as usize)
}

#[tracing::instrument(level = "trace", skip_all, err)]
#[allow(clippy::cast_possible_wrap)]
pub async fn get_articles<'c>(
    executor: impl DatabaseExecutor<'c>,
    page_index: usize,
    limit: NonZeroU8,
) -> sqlx::Result<Vec<Article>> {
    let limit = limit.get();
    let offset = page_index * usize::from(limit);
    tracing::trace!(limit, offset);

    let query = sqlx::query_as("SELECT * FROM articles ORDER BY created_at DESC LIMIT ? OFFSET ?;")
        .bind(i64::from(limit))
        .bind(offset as i64);

    query.fetch_all(executor).await
}

#[tracing::instrument(level = "trace", skip(executor), err)]
pub async fn get_article_by_id<'c>(
    executor: impl DatabaseExecutor<'c>,
    id: ArticleId,
) -> sqlx::Result<Option<Article>> {
    let query = sqlx::query_as("SELECT * FROM articles WHERE id = ?;").bind(id);
    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip(executor), ret, err)]
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

    query.fetch_optional(executor).await
}

#[tracing::instrument(level = "trace", skip_all, ret, err)]
pub async fn create_article<'c>(
    executor: impl DatabaseExecutor<'c>,
    article: &Article,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query(
        "
        INSERT INTO articles (
            id, status, title, byline,
            content, text_content, length,
            excerpt, site_name, dir,
            lang, published_time, modified_time,
            image, favicon, url
        )
        VALUES (
            ?, ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?
        )
        ",
    )
    .bind(article.id)
    .bind(article.status)
    .bind(&article.title)
    .bind(&article.byline)
    .bind(&article.content)
    .bind(&article.text_content)
    .bind(article.length)
    .bind(&article.excerpt)
    .bind(&article.site_name)
    .bind(article.dir)
    .bind(&article.lang)
    .bind(article.published_time)
    .bind(article.modified_time)
    .bind(&article.image)
    .bind(&article.favicon)
    .bind(&article.url);

    query.execute(executor).await
}
