use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    time::Duration,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    redis: RedisConfig,
    amqp: AmqpConfig,
    feeds: Vec<FeedConfig>,
}

impl Config {
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    pub fn into_feeds(self) -> Vec<FeedConfig> {
        self.feeds
    }

    pub fn amqp(&self) -> &AmqpConfig {
        &self.amqp
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AmqpConfig {
    url: String,
    queue: String,
}

impl AmqpConfig {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn queue(&self) -> &str {
        &self.queue
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RedisConfig {
    url: String,
}

impl RedisConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FeedConfig {
    url: String,
    interval: Duration,
    max_concurrent_requests: usize,
}

impl FeedConfig {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn max_concurrent_requests(&self) -> usize {
        self.max_concurrent_requests
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
}

#[tracing::instrument(level = "trace", skip_all, fields(path = %path.as_ref().display()), ret, err)]
pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, LoadConfigError> {
    let bytes = fs::read(path)?;
    toml::from_slice(&bytes).map_err(Into::into)
}

#[derive(Debug, thiserror::Error)]
pub enum WriteConfigError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    SerializeError(#[from] toml::ser::Error),
}

#[tracing::instrument(level = "trace", skip_all, fields(path = %path.as_ref().display()), err)]
pub fn write_config_into_file<P: AsRef<Path>>(
    path: P,
    config: &Config,
) -> Result<(), WriteConfigError> {
    let path = path.as_ref();

    if let Some(parent_path) = path.parent() {
        // Create parent directories if they don't exist.
        fs::create_dir_all(parent_path)?;
    }

    let mut file = File::create_new(path)?;
    let pretty_toml = toml::to_string_pretty(config)?;
    file.write_all(pretty_toml.as_bytes()).map_err(Into::into)
}

/// Note: [`Config`] doesn't implement [`std::default::Default`] and return value of this
/// function should not be treated as a default value of config. It's just an example config.
pub fn new_default_config() -> Config {
    Config {
        redis: RedisConfig {
            url: "redis://127.0.0.1:6379".to_owned(),
        },
        amqp: AmqpConfig {
            url: "amqp://user:password@127.0.0.1:5672".to_owned(),
            queue: "rss_articles".to_owned(),
        },
        feeds: vec![FeedConfig {
            url: "https://news.ycombinator.com/rss".to_owned(),
            interval: Duration::from_mins(1),
            max_concurrent_requests: 15,
        }],
    }
}
