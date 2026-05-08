use lapin::{
    Connection, ConnectionProperties,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use rss_reader::ParsedArticle;
use tokio_stream::StreamExt;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

mod internal;

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            fmt::layer()
                .with_target(false)
                .with_span_events(FmtSpan::CLOSE),
        )
        .init();

    tracing::info!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    let amqp_connection = Connection::connect(
        "amqp://user:password@127.0.0.1:5672",
        ConnectionProperties::default(),
    )
    .await
    .unwrap();
    let channel = amqp_connection.create_channel().await.unwrap();

    let mut conusmer = channel
        .basic_consume(
            "rss_articles".into(),
            "rss_feed_consumer".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    while let Some(delivery) = conusmer.next().await {
        let delivery = delivery.unwrap();
        delivery.ack(BasicAckOptions::default()).await.unwrap();
        let article = serde_json::from_slice::<ParsedArticle>(&delivery.data).unwrap();
        tracing::trace!(?article);
    }
}
