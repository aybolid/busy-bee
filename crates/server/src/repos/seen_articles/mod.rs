mod fns;
mod types;

pub use fns::{check_if_seen_article, create_seen_article};
#[allow(unused_imports)]
pub use types::{SeenArticle, SeenArticleId};
