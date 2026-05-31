use std::{
    env::{self, VarError},
    ffi::OsStr,
    fmt::{Debug, Display},
    net::SocketAddr,
    str::FromStr,
};

use types::{Url, nonempty_trimmed_string};

use crate::ai;

pub struct AiConfig {
    pub model: ai::ModelName,
    pub api_key: ai::ApiKey,
}

pub struct Config {
    pub api_addr: SocketAddr,
    pub database_url: Url,
    pub ai: AiConfig,
}

#[tracing::instrument(level = "trace")]
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

#[tracing::instrument(level = "trace")]
fn load_ai_config() -> AiConfig {
    AiConfig {
        model: ai::ModelName(parse_or_else("AI_MODEL", || {
            nonempty_trimmed_string!("gemma4")
        })),
        api_key: ai::ApiKey(parse_or("AI_API_KEY", "")),
    }
}

#[tracing::instrument(level = "trace")]
fn load_dotenv() {
    if let Ok(env_file) = dotenvy::dotenv() {
        tracing::info!(env_file = %env_file.display(), "loaded env file");
    } else {
        tracing::warn!("env file not found. using existing environment");
    }
}

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
