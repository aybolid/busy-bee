use crate::repos::articles::{
    ArticleByLine, ArticleContent, ArticleExcerpt, ArticleLang, ArticleSiteName,
    ArticleTextContent, ArticleTitle, ReadabilityArticle,
};

/// Errors that can occur when converting a [`dom_smoothie::Article`] into an [`ReadabilityArticle`].
#[derive(Debug, thiserror::Error)]
pub enum FromDomSmoothieArticleError {
    #[error("got empty string in the {0} field")]
    EmptyString(&'static str),
    #[error("article url is missing or invalid")]
    MissingOrInvalidArticleUrl,
}

/// Helper function to parse optional string fields and log a warning if parsing fails.
///
/// This encapsulates the boilerplate of unpacking an `Option<String>`, attempting a parse,
/// emitting a `tracing::warn!` on failure, and falling back to `None`.
fn parse_optional_field<T: std::str::FromStr>(
    value: Option<String>,
    field_name: &'static str,
) -> Option<T>
where
    <T as std::str::FromStr>::Err: std::error::Error,
{
    value.and_then(|s| {
        s.parse()
            .inspect_err(|error| tracing::warn!(?error, field = field_name))
            .ok()
    })
}

/// Attempts to map a raw scraped [`dom_smoothie::Article`] into thread-safe [`ReadabilityArticle`].
///
/// # Errors
///
/// Returns a [`FromDomSmoothieArticleError`] if strictly required fields
/// (like `title`, `content`, `text_content`, or `url`) are missing or empty.
impl TryFrom<dom_smoothie::Article> for ReadabilityArticle {
    type Error = FromDomSmoothieArticleError;

    fn try_from(value: dom_smoothie::Article) -> Result<Self, Self::Error> {
        let title = ArticleTitle::new(value.title)
            .ok_or(FromDomSmoothieArticleError::EmptyString("title"))?;
        let content = ArticleContent::new(value.content.to_string())
            .ok_or(FromDomSmoothieArticleError::EmptyString("content"))?;
        let text_content = ArticleTextContent::new(value.text_content.to_string())
            .ok_or(FromDomSmoothieArticleError::EmptyString("text_content"))?;
        let length = text_content.char_len();

        let url = value
            .url
            .and_then(|s| {
                s.parse()
                    .inspect_err(|error| tracing::error!(?error, field = "url"))
                    .ok()
            })
            .ok_or(FromDomSmoothieArticleError::MissingOrInvalidArticleUrl)?;

        let byline = value.byline.and_then(ArticleByLine::new);
        let excerpt = value.excerpt.and_then(ArticleExcerpt::new);
        let site_name = value.site_name.and_then(ArticleSiteName::new);
        let lang = value.lang.and_then(ArticleLang::new);

        let dir = parse_optional_field(value.dir, "dir");
        let published_time = parse_optional_field(value.published_time, "published_time");
        let modified_time = parse_optional_field(value.modified_time, "modified_time");
        let image = parse_optional_field(value.image, "image");
        let favicon = parse_optional_field(value.favicon, "favicon");

        Ok(Self {
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
