use std::{
    env::{self},
    ffi::OsStr,
    fmt::Debug,
    net::SocketAddr,
    str::FromStr,
};

use types::{Url, nonempty_trimmed_string};

use crate::ai::{ApiKey, ModelName};

/// Configuration specific to the AI services.
pub struct AiConfig {
    /// The specific model to use for AI generations.
    pub model: ModelName,
    /// The API key required to authenticate with the AI service provider.
    pub api_key: Option<ApiKey>,
}

/// The global application configuration state.
pub struct Config {
    /// The socket address the API server will bind to.
    pub api_addr: SocketAddr,
    /// The connection string used to connect to the database.
    pub database_url: Url,
    /// The AI-specific configuration settings.
    pub ai: AiConfig,
}

/// Loads the application configuration from the environment.
///
/// This function attempts to load variables from a `.env` file first. It then parses
/// the necessary configuration parameters, falling back to default values if the
/// environment variables are missing or fail to parse.
#[tracing::instrument]
pub(super) fn load_config() -> Config {
    load_dotenv();

    let api_addr = parse_or_else::<SocketAddr>("API_ADDR", || {
        SocketAddr::from_str("0.0.0.0:3000").expect("default api addr must parse")
    });

    let database_url = parse_or_else("DB_URL", || {
        Url::try_new("sqlite://data.db").expect("default database url must parse")
    });

    Config {
        api_addr,
        database_url,
        ai: load_ai_config(),
    }
}

/// Loads the AI-specific configuration settings.
fn load_ai_config() -> AiConfig {
    AiConfig {
        model: parse_or_else("AI_MODEL", || ModelName(nonempty_trimmed_string!("gemma4"))),
        api_key: parse_optional("AI_API_KEY"),
    }
}

/// Attempts to load environment variables from a `.env` file into the current process.
#[tracing::instrument]
fn load_dotenv() {
    if let Ok(path) = dotenvy::dotenv() {
        tracing::info!(path = %path.display(), "env file loaded");
    } else {
        tracing::warn!("env file not found. using existing environment");
    }
}

/// Helper to parse an environment variable or fall back to a specific value.
///
/// This is a convenience wrapper around [`parse_or_else`] that takes a direct value
/// rather than a closure.
#[allow(dead_code)]
fn parse_or<T: FromStr + Debug>(key: impl AsRef<OsStr>, default: impl Into<T>) -> T
where
    <T as FromStr>::Err: std::error::Error,
{
    parse_or_else(key, || default.into())
}

/// Helper to parse an environment variable or lazily evaluate a fallback closure.
///
/// If parsing fails or the variable is missing,
/// the `default` closure is executed and its value is logged and returned.
fn parse_or_else<T: FromStr + Debug>(key: impl AsRef<OsStr>, default: impl FnOnce() -> T) -> T
where
    <T as FromStr>::Err: std::error::Error,
{
    parse_optional(key).unwrap_or_else(default)
}

/// Helper to parse an optional environment variable.
///
/// It handles logging errors if the environment variable contains invalid unicode or
/// fails to parse into the target type `T`.
fn parse_optional<T: FromStr>(key: impl AsRef<OsStr>) -> Option<T>
where
    <T as FromStr>::Err: std::error::Error,
{
    env::var(key).ok().and_then(|val| val.parse().ok())
}
