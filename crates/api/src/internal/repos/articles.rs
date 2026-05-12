use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::internal::infra::db::{DatabaseExecutor, DatabaseQueryResult};

#[derive(Debug, serde::Serialize)]
pub struct Article {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    title: String,
    byline: Option<String>,
    content: String,
    text_content: String,
    length: usize,
    excerpt: Option<String>,
    site_name: Option<String>,
    dir: Option<String>,
    lang: Option<String>,
    published_time: Option<DateTime<Utc>>,
    modified_time: Option<DateTime<Utc>>,
    image: Option<String>,
    favicon: Option<String>,
    url: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum FromParsedArticeError {
    #[error(transparent)]
    DateParseError(#[from] chrono::ParseError),
}

impl TryFrom<rss_reader::ParsedArticle> for Article {
    type Error = FromParsedArticeError;

    fn try_from(value: rss_reader::ParsedArticle) -> Result<Self, Self::Error> {
        let now = Utc::now();

        let published_time = value.published_time.map(|s| s.parse()).transpose()?;
        let modified_time = value.modified_time.map(|s| s.parse()).transpose()?;

        Ok(Self {
            id: Uuid::now_v7(),
            created_at: now,
            updated_at: now,

            title: value.title,
            byline: value.byline,
            content: value.content,
            text_content: value.text_content,
            length: value.length,
            excerpt: value.excerpt,
            site_name: value.site_name,
            dir: value.dir,
            lang: value.lang,
            published_time,
            modified_time,
            image: value.image,
            favicon: value.favicon,
            url: value.url,
        })
    }
}

#[tracing::instrument(level = "trace", skip_all, ret, err)]
pub async fn create_article<'c>(
    executor: impl DatabaseExecutor<'c>,
    article: &Article,
) -> sqlx::Result<DatabaseQueryResult> {
    let query = sqlx::query(
        "
        INSERT INTO articles (
            id, title, byline,
            content, text_content, length,
            excerpt, site_name, dir,
            lang, published_time, modified_time,
            image, favicon, url
        )
        VALUES (
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?
        )
        ",
    )
    .bind(article.id)
    .bind(&article.title)
    .bind(&article.byline)
    .bind(&article.content)
    .bind(&article.text_content)
    .bind(
        #[allow(clippy::cast_possible_wrap)]
        {
            article.length as i64
        },
    )
    .bind(&article.excerpt)
    .bind(&article.site_name)
    .bind(&article.dir)
    .bind(&article.lang)
    .bind(article.published_time)
    .bind(article.modified_time)
    .bind(&article.image)
    .bind(&article.favicon)
    .bind(&article.url);

    query.execute(executor).await
}
