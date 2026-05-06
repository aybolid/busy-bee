use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    time::Duration,
};

/// Application configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Redis configuration.
    redis: RedisConfig,
    /// RSS feed configurations.
    feeds: Vec<FeedConfig>,
}

impl Config {
    /// Returns the Redis configuration.
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    /// Consumes `self` and returns an owned [`Vec`] of [`FeedConfig`]s.
    pub fn into_feeds(self) -> Vec<FeedConfig> {
        self.feeds
    }
}

/// Redis configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RedisConfig {
    /// Redis URL.
    url: String,
}

impl RedisConfig {
    /// Returns the Redis URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// RSS feed configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FeedConfig {
    /// RSS feed URL.
    url: String,
    /// Interval between feed updates.
    interval: Duration,
    /// Maximum number of concurrent requests to articles links.
    max_concurrent_requests: usize,
}

impl FeedConfig {
    /// Returns the RSS feed URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the interval between feed updates.
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// Returns the maximum number of concurrent requests to articles links.
    pub fn max_concurrent_requests(&self) -> usize {
        self.max_concurrent_requests
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when loading a [`Config`] from a file.
pub enum LoadConfigError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
}

/// Reads a file at `path` and parses it as a [`Config`]. TOML is used as the deserialization format.
#[tracing::instrument(level = "trace", skip_all, fields(path = %path.as_ref().display()), ret, err)]
pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, LoadConfigError> {
    let bytes = fs::read(path)?;
    toml::from_slice(&bytes).map_err(Into::into)
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when writing a [`Config`] to a file.
pub enum WriteConfigError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    SerializeError(#[from] toml::ser::Error),
}

/// Writes the [`Config`] to a file at `path` using TOML as the serialization format.
///
/// - Destination file should not already exist.
/// - Parent directories of `path` will be created if they don't exist.
/// - Config will be serialized as a "pretty" TOML string.
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

/// Creates a new "default" [`Config`].
///
/// Note: [`Config`] doesn't implement [`std::default::Default`] and return value of this
/// function should not be treated as a default value of config. It's just an example config.
pub fn new_default_config() -> Config {
    Config {
        redis: RedisConfig {
            url: "redis://127.0.0.1:6379".to_owned(),
        },
        feeds: vec![FeedConfig {
            url: "https://news.ycombinator.com/rss".to_owned(),
            interval: Duration::from_secs(60),
            max_concurrent_requests: 15,
        }],
    }
}
