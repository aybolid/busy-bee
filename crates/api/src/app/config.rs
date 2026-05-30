use std::{
    env::{self, VarError},
    ffi::OsStr,
    fmt::{Debug, Display},
    net::SocketAddr,
    str::FromStr,
};

use types::{NonEmpty, TrimmedString, Url};

pub struct Config {
    pub api_addr: SocketAddr,
    pub database_url: Url,
    pub ai_model: NonEmpty<TrimmedString>,
    pub ai_api_key: TrimmedString,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("api_addr", &self.api_addr)
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

#[tracing::instrument(level = "trace", ret)]
pub(super) fn load_config() -> Config {
    load_dotenv();

    let api_addr = parse_or_else::<SocketAddr>("API_ADDR", || {
        SocketAddr::from_str("0.0.0.0:3000").expect("default api addr must parse")
    });

    let database_url = parse_or_else("DB_URL", || {
        Url::try_new("sqlite://data.db").expect("default database url must parse")
    });

    let ai_model = parse_or_else("AI_MODEL", || {
        NonEmpty::new(TrimmedString::new("gemma4")).expect("default ai model value is not empty")
    });
    let ai_api_key = parse_or("AI_API_KEY", "");

    Config {
        api_addr,
        database_url,
        ai_model,
        ai_api_key,
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
