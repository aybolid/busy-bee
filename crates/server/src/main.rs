use tracing_subscriber::{
    EnvFilter, Layer,
    filter::filter_fn,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::app::RunError;

mod ai;
mod app;
mod infra;
mod repos;
mod workers;

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            fmt::layer()
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(filter_fn(|meta| meta.target().contains("repos"))),
        )
        .with(fmt::layer().with_filter(filter_fn(|meta| !meta.target().contains("repos"))))
        .init();

    tracing::info!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"));
}

#[tokio::main]
async fn main() -> Result<(), RunError> {
    init_tracing_subscriber();
    app::run().await
}
