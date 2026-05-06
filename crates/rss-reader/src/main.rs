use std::{
    fs::{self, File},
    io::Write,
    sync::Arc,
    time::Duration,
};

use clap::Parser;
use dom_smoothie::{Article, Readability, ReadabilityError};
use redis::{
    AsyncTypedCommands, ExistenceCheck, RedisResult, SetOptions, aio::MultiplexedConnection,
};
use tokio::{sync::Semaphore, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    cli::{Args, Command},
    config::{Config, FeedConfig},
};

mod cli;
mod config;

#[tracing::instrument(level = "trace", skip_all, fields(url = config.url))]
async fn rss_worker(
    client: reqwest::Client,
    config: FeedConfig,
    redis: Arc<MultiplexedConnection>,
    cancel_token: CancellationToken,
) {
    tracing::info!("started");
    let config = Arc::new(config);
    loop {
        // NOTE: Error is logged by `tracing::instrument`
        _ = process_rss_feed(&client, &config, redis.clone()).await;

        let sleep = async {
            if !cancel_token.is_cancelled() {
                tracing::trace!("sleeping for {:?}", config.interval);
            }
            tokio::time::sleep(config.interval).await
        };

        tokio::select! {
            _ = sleep => {}
            _ = cancel_token.cancelled() => {
                tracing::trace!("shutdown signal received. exiting");
                break;
            }
        }
    }
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn process_rss_feed(
    client: &reqwest::Client,
    config: &FeedConfig,
    redis: Arc<MultiplexedConnection>,
) -> anyhow::Result<()> {
    let response = client.get(&config.url).send().await?;
    tracing::trace!(headers = ?response.headers());
    let bytes = response.bytes().await?;

    let channel = rss::Channel::read_from(&bytes[..])?;
    tracing::info!(channel_title = channel.title, items = channel.items.len());

    let semaphore = Arc::new(Semaphore::new(config.max_concurrent_requests));
    let tasks =
        JoinSet::from_iter(channel.items.into_iter().map(|item| {
            process_rss_feed_item(client.clone(), item, semaphore.clone(), redis.clone())
        }));

    // NOTE: Errors are logged by `tracing::instrument`
    tasks.join_all().await;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct ParsedArticle {
    /// The title
    title: String,
    /// The author
    byline: Option<String>,
    /// The relevant HTML content
    content: String,
    /// The relevant text content
    text_content: String,
    /// The text length
    length: usize,
    /// The excerpt
    excerpt: Option<String>,
    /// The name of the site
    site_name: Option<String>,
    /// The text direction
    dir: Option<String>,
    /// The document language
    lang: Option<String>,
    /// The published time of the document
    published_time: Option<String>,
    /// The modified time of the document
    modified_time: Option<String>,
    /// The image of the document
    image: Option<String>,
    /// The favicon of the document
    favicon: Option<String>,
    /// The metadata's url
    url: Option<String>,
}

impl From<Article> for ParsedArticle {
    fn from(value: Article) -> Self {
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

#[tracing::instrument(level = "trace", skip_all, fields(item_title = item.title, item_link = item.link), err)]
async fn process_rss_feed_item(
    client: reqwest::Client,
    item: rss::Item,
    semaphore: Arc<Semaphore>,
    redis: Arc<MultiplexedConnection>,
) -> anyhow::Result<()> {
    let Some(link) = item.link() else {
        tracing::warn!("no article link found");
        return Ok(());
    };

    let was_set = cache_rss_item(&redis, link).await?;
    if !was_set {
        tracing::trace!("skipping already seen article");
        return Ok(());
    }

    // Acquire a permit to limit a number of concurrent requests.
    let permit = semaphore.acquire().await?;
    let response = client.get(link).send().await?;
    let text = response.text().await?;
    drop(permit);

    // Parsing HTML to get article text is a CPU-bound job that will block thread.
    // Using dedicated thread were blocking is acceptable.
    let article = tokio::task::spawn_blocking(|| -> Result<ParsedArticle, ReadabilityError> {
        let mut readability = Readability::new(text, None, Some(dom_smoothie::Config::default()))?;
        let article = readability.parse()?;
        // NOTE: `Article` cannot be safely shared between threads so we use `ParsedArticle` instead.
        Ok(article.into())
    })
    .await??;
    tracing::trace!(?article);

    // TODO: Do smth with article

    Ok(())
}

#[tracing::instrument(level = "trace", skip(redis), ret, err)]
async fn cache_rss_item(redis: &MultiplexedConnection, link: &str) -> RedisResult<bool> {
    let mut conn = redis.clone();

    conn.set_options(
        link,
        "1", // Dummy value
        SetOptions::default()
            .conditional_set(ExistenceCheck::NX)
            .with_expiration(redis::SetExpiry::EX(24 * 7 * 3600)),
    )
    .await
    .map(|o| o.is_some())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt::layer().with_target(false).compact())
        .init();

    tracing::info!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn run_rss_reader(args: Args) -> anyhow::Result<()> {
    let config_file_data = fs::read(args.config)?;
    let config = toml::from_slice::<Config>(&config_file_data)?;
    tracing::trace!(?config);

    let cancel_token = CancellationToken::new();

    let token_clone = cancel_token.clone();
    tokio::spawn(async move {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .inspect_err(|error| tracing::error!(%error))
                .unwrap();
        };

        #[cfg(unix)]
        let terminate = async {
            use tokio::signal::unix::SignalKind;
            tokio::signal::unix::signal(SignalKind::terminate())
                .inspect_err(|error| tracing::error!(%error))
                .unwrap()
                .recv()
                .await;
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => tracing::trace!("ctrl + c"),
            _ = terminate => tracing::trace!("SIGTERM")
        }

        token_clone.cancel();
    });

    let client = reqwest::Client::builder().build()?;

    let redis = redis::Client::open(config.redis)?;
    let redis_conn = Arc::new(redis.get_multiplexed_async_connection().await?);

    let rss_tasks = JoinSet::from_iter(config.feeds.into_iter().map(|config| {
        rss_worker(
            client.clone(),
            config,
            redis_conn.clone(),
            cancel_token.clone(),
        )
    }));

    rss_tasks.join_all().await;

    tracing::info!("bye!");
    Ok(())
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn init_rss_reader(args: Args) -> anyhow::Result<()> {
    let mut config_file = File::create_new(&args.config)?;

    let rss_config = Config {
        feeds: vec![FeedConfig {
            url: "https://news.ycombinator.com/rss".to_owned(),
            interval: Duration::from_secs(60),
            max_concurrent_requests: 5,
        }],
        redis: "redis://127.0.0.1:6379".to_owned(),
    };
    let config_toml = toml::to_string_pretty(&rss_config)?;

    config_file.write_all(config_toml.as_bytes())?;

    tracing::info!("created config file in {}", args.config.display());
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();
    let args = Args::parse();
    tracing::debug!(?args);

    match args.command.unwrap_or_default() {
        Command::Run => run_rss_reader(args).await,
        Command::Init => init_rss_reader(args),
    }
}
