use std::{
    env::{self, VarError},
    ffi::OsStr,
    fmt::{Debug, Display},
    net::SocketAddr,
    str::FromStr,
};

use types::{NonEmpty, TrimmedString};
use url::Url;

pub struct Config {
    api_addr: SocketAddr,
    amqp_url: Url,
    rss_articles_queue: NonEmpty<TrimmedString>,
    article_processor_queue: NonEmpty<TrimmedString>,
    database_url: Url,
    ai_model: NonEmpty<TrimmedString>,
    ai_api_key: TrimmedString,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("api_addr", &self.api_addr)
            .field("amqp_url", &self.amqp_url.as_str())
            .field("rss_articles_queue", &self.rss_articles_queue)
            .field("article_processor_queue", &self.article_processor_queue)
            .field("database_url", &self.database_url.as_str())
            .field("ai_model", &self.ai_model)
            .field(
                "ai_api_key",
                if self.ai_api_key.is_empty() {
                    &""
                } else {
                    &"[REDACTED]"
                },
            )
            .finish()
    }
}

impl Config {
    pub fn api_addr(&self) -> SocketAddr {
        self.api_addr
    }

    pub fn amqp_url(&self) -> &str {
        self.amqp_url.as_ref()
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_ref()
    }

    pub fn ai_model(&self) -> &NonEmpty<TrimmedString> {
        &self.ai_model
    }

    pub fn ai_api_key(&self) -> &TrimmedString {
        &self.ai_api_key
    }

    pub fn rss_articles_queue(&self) -> &NonEmpty<TrimmedString> {
        &self.rss_articles_queue
    }

    pub fn article_processor_queue(&self) -> &NonEmpty<TrimmedString> {
        &self.article_processor_queue
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
    #[error(transparent)]
    ShortStringError(#[from] lapin::types::ShortStringError),
}

#[tracing::instrument(level = "trace", ret, err(Debug))]
pub(super) fn load_config() -> Result<Config, LoadConfigError> {
    load_dotenv();

    let api_addr = parse_or_else::<SocketAddr>("API_ADDR", || {
        SocketAddr::from_str("0.0.0.0:3000").expect("default api addr must parse")
    });

    let amqp_url = parse_or_else::<Url>("AMQP_URL", || {
        Url::parse("amqp://user:password@127.0.0.1:5672").expect("default amqp url must parse")
    });
    let rss_articles_queue = parse_or_else("RSS_ARTICLES_QUEUE", || {
        NonEmpty::new(TrimmedString::new("rss_articles"))
            .expect("default rss articles queue value is not empty")
    });
    let article_processor_queue = parse_or_else("ARTICLE_PROCESSOR_QUEUE", || {
        NonEmpty::new(TrimmedString::new("article_processor"))
            .expect("default article processor queue value is not empty")
    });

    let database_url = parse_or_else("DB_URL", || {
        Url::parse("sqlite://data.db").expect("default database url must parse")
    });

    let ai_model = parse_or_else("AI_MODEL", || {
        NonEmpty::new(TrimmedString::new("gemma4")).expect("default ai model value is not empty")
    });
    let ai_api_key = parse_or("AI_API_KEY", "");

    Ok(Config {
        api_addr,
        amqp_url,
        rss_articles_queue,
        article_processor_queue,
        database_url,
        ai_model,
        ai_api_key,
    })
}

#[tracing::instrument(level = "trace")]
fn load_dotenv() {
    if let Ok(env_file) = dotenvy::dotenv() {
        tracing::info!(env_file = %env_file.display(), "loaded env file");
    } else {
        tracing::warn!("env file not found. using existing environment");
    }
}

// #[tracing::instrument(level = "trace", skip_all, fields(key = ?key.as_ref()))]
// fn get_or(key: impl AsRef<OsStr>, default: &impl ToString) -> String {
//     env::var(key).unwrap_or_else(|error| {
//         if matches!(error, VarError::NotUnicode(_)) {
//             tracing::error!(?error);
//         } else {
//             tracing::warn!("not found");
//         }

//         let default = default.to_string();
//         tracing::warn!(default, "using default value");
//         default
//     })
// }

#[tracing::instrument(level = "trace", skip_all, fields(key = ?key.as_ref()))]
fn parse_or<T: FromStr + Debug + Display>(key: impl AsRef<OsStr>, default: impl Into<T>) -> T
where
    <T as FromStr>::Err: std::error::Error,
{
    env::var(key)
        .inspect_err(|error| {
            if matches!(error, VarError::NotUnicode(_)) {
                tracing::error!(?error);
            } else {
                tracing::warn!("not found");
            }
        })
        .ok()
        .and_then(|val| {
            val.parse()
                .inspect_err(|error| tracing::error!(?error))
                .ok()
        })
        .unwrap_or_else(|| {
            let default = default.into();
            tracing::warn!(%default, "using default value");
            default
        })
}

#[tracing::instrument(level = "trace", skip_all, fields(key = ?key.as_ref()))]
fn parse_or_else<T: FromStr + Debug + Display>(
    key: impl AsRef<OsStr>,
    default: impl FnOnce() -> T,
) -> T
where
    <T as FromStr>::Err: std::error::Error,
{
    env::var(key)
        .inspect_err(|error| {
            if matches!(error, VarError::NotUnicode(_)) {
                tracing::error!(?error);
            } else {
                tracing::warn!("not found");
            }
        })
        .ok()
        .and_then(|val| {
            val.parse()
                .inspect_err(|error| tracing::error!(?error))
                .ok()
        })
        .unwrap_or_else(|| {
            let default = default();
            tracing::warn!(%default, "using default value");
            default
        })
}
