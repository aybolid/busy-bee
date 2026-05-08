#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ParsedArticle {
    pub title: String,
    pub byline: Option<String>,
    pub content: String,
    pub text_content: String,
    pub length: usize,
    pub excerpt: Option<String>,
    pub site_name: Option<String>,
    pub dir: Option<String>,
    pub lang: Option<String>,
    pub published_time: Option<String>,
    pub modified_time: Option<String>,
    pub image: Option<String>,
    pub favicon: Option<String>,
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
