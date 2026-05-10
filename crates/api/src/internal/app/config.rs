use std::{
    env::{self, VarError},
    ffi::OsStr,
    fmt::{Debug, Display},
    net::SocketAddr,
    str::FromStr,
};

use lapin::types::ShortString;
use url::Url;

pub struct Config {
    api_addr: SocketAddr,
    amqp_url: Url,
    rss_articles_queue: ShortString,
    database_url: Url,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("api_addr", &self.api_addr)
            .field("amqp_url", &self.amqp_url.as_str())
            .field("rss_articles_queue", &self.rss_articles_queue)
            .field("database_url", &self.database_url.as_str())
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

    pub fn rss_articles_queue(&self) -> &ShortString {
        &self.rss_articles_queue
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_ref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
    #[error(transparent)]
    ShortStringError(#[from] lapin::types::ShortStringError),
}

#[tracing::instrument(level = "trace", ret, err)]
pub(super) fn load_config() -> Result<Config, LoadConfigError> {
    load_dotenv();

    let api_addr = parse_or_else::<SocketAddr>("API_ADDR", || {
        SocketAddr::from_str("0.0.0.0:3000").expect("default api addr must parse")
    });

    let amqp_url = parse_or_else::<Url>("AMQP_URL", || {
        Url::parse("amqp://user:password@127.0.0.1:5672").expect("default amqp url must parse")
    });
    let rss_articles_queue = ShortString::try_new(get_or("RSS_ARTICLES_QUEUE", "rss_articles"))?;

    let database_url = parse_or_else("DB_URL", || {
        Url::parse("sqlite://data.db").expect("default database url must parse")
    });

    Ok(Config {
        api_addr,
        amqp_url,
        rss_articles_queue,
        database_url,
    })
}

#[tracing::instrument(level = "trace")]
fn load_dotenv() {
    match dotenvy::dotenv() {
        Ok(env_file) => tracing::info!(env_file = %env_file.display(), "loaded env file"),
        Err(_) => tracing::warn!("env file not found. using existing environment"),
    }
}

#[tracing::instrument(level = "trace", skip_all, fields(key = ?key.as_ref()))]
fn get_or(key: impl AsRef<OsStr>, default: impl ToString) -> String {
    env::var(key).unwrap_or_else(|error| {
        if matches!(error, VarError::NotUnicode(_)) {
            tracing::error!(%error);
        } else {
            tracing::warn!("not found");
        };

        let default = default.to_string();
        tracing::warn!(default, "using default value");
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
                tracing::error!(%error)
            } else {
                tracing::warn!("not found");
            }
        })
        .ok()
        .and_then(|val| {
            val.parse()
                .inspect_err(|error| tracing::error!(%error))
                .ok()
        })
        .unwrap_or_else(|| {
            let default = default();
            tracing::warn!(%default, "using default value");
            default
        })
}
