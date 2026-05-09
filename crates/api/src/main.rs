use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod internal;

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt::layer().with_target(false))
        .init();

    tracing::info!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    internal::app::run().await.unwrap();
}
