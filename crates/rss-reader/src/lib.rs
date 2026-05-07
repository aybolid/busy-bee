/// Represnts a parsed article from RSS feed.
///
/// It contains the same fields as [`dom_smoothie::Article`] but can be safely
/// shared between threads.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ParsedArticle {
    /// The title
    pub title: String,
    /// The author
    pub byline: Option<String>,
    /// The relevant HTML content
    pub content: String,
    /// The relevant text content
    pub text_content: String,
    /// The text length
    pub length: usize,
    /// The excerpt
    pub excerpt: Option<String>,
    /// The name of the site
    pub site_name: Option<String>,
    /// The text direction
    pub dir: Option<String>,
    /// The document language
    pub lang: Option<String>,
    /// The published time of the document
    pub published_time: Option<String>,
    /// The modified time of the document
    pub modified_time: Option<String>,
    /// The image of the document
    pub image: Option<String>,
    /// The favicon of the document
    pub favicon: Option<String>,
    /// The metadata's url
    pub url: Option<String>,
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
