use std::sync::Arc;

use clap::Parser;
use lapin::{options::QueueDeclareOptions, types::FieldTable};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::internal::{
    cli::{Args, Command},
    config::{WriteConfigError, load_config_from_file, new_default_config, write_config_into_file},
    worker::{FeedWorkerContext, rss_worker},
};

mod internal;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
enum RunError {
    #[error(transparent)]
    LoadConfigError(#[from] internal::config::LoadConfigError),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    AmqpError(#[from] lapin::Error),
}

#[tracing::instrument(level = "trace", skip_all, err)]
async fn run_rss_reader(args: Args) -> Result<(), RunError> {
    let config = load_config_from_file(args.config)?;

    let cancel_token = CancellationToken::new();
    tokio::spawn(shutdown_signal_listener(cancel_token.clone()));

    let http_client = reqwest::Client::new();

    let redis = redis::Client::open(config.redis().url())?;
    let redis_connection = redis.get_multiplexed_async_connection().await?;
    tracing::info!("redis connected");

    let amqp_connection =
        lapin::Connection::connect(config.amqp().url(), lapin::ConnectionProperties::default())
            .await?;
    tracing::info!("amqp server connected");

    let setup_channel = amqp_connection.create_channel().await?;
    tracing::info!("amqp setup channel created");
    let queue = setup_channel
        .queue_declare(
            config.amqp().queue().into(),
            QueueDeclareOptions::durable(),
            FieldTable::default(),
        )
        .await?;
    tracing::info!(?queue, "amqp queue declared");
    drop(setup_channel);

    let amqp_queue = Arc::new(config.amqp().queue().to_owned());

    let mut worker_contexts = vec![];
    for config in config.into_feeds() {
        worker_contexts.push(FeedWorkerContext::new(
            config,
            http_client.clone(),
            redis_connection.clone(),
            amqp_connection.create_channel().await?,
            amqp_queue.clone(),
            cancel_token.clone(),
        ));
    }
    tracing::info!("rss worker contexts prepared");

    let workers = worker_contexts
        .into_iter()
        .map(rss_worker)
        .collect::<JoinSet<_>>();

    tracing::info!("starting {} workers", workers.len());
    workers.join_all().await;

    tracing::info!("bye!");
    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn shutdown_signal_listener(cancel_token: CancellationToken) {
    let ctrl_c = async {
        tracing::info!("listening to ctrl + c notification");
        tokio::signal::ctrl_c()
            .await
            .inspect_err(|error| tracing::error!(%error))
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;

        tracing::info!("listening to SIGTERM");
        tokio::signal::unix::signal(SignalKind::terminate())
            .inspect_err(|error| tracing::error!(%error))
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => tracing::trace!("ctrl + c"),
        () = terminate => tracing::trace!("SIGTERM")
    }

    cancel_token.cancel();
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn init_rss_reader(args: Args) -> Result<(), WriteConfigError> {
    let default_config = new_default_config();
    write_config_into_file(&args.config, &default_config)?;

    tracing::info!("created config file in {}", args.config.display());
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt::layer().with_target(false))
        .init();

    tracing::info!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"));
}

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    let args = Args::parse();
    tracing::debug!(?args);

    match args.command.unwrap_or_default() {
        Command::Run => run_rss_reader(args)
            .await
            .inspect_err(|error| tracing::error!(%error))
            .unwrap(),
        Command::Init => init_rss_reader(args)
            .inspect_err(|error| tracing::error!(%error))
            .unwrap(),
    }
}
