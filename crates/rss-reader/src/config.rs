use std::time::Duration;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub feeds: Vec<FeedConfig>,
    pub redis: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FeedConfig {
    pub url: String,
    pub interval: Duration,
    pub max_concurrent_requests: usize,
}
