use tokio::sync::broadcast;
use types::{NonEmpty, TrimmedString};

const BROADCAST_CAPACITY: usize = 100;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Notification(NotificationData),
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationVariant {
    Info,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct NotificationString(pub NonEmpty<TrimmedString>);

impl NotificationString {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        let s = TrimmedString::from(s.to_string());
        NonEmpty::new(s).map(Self)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NotificationData {
    pub variant: NotificationVariant,
    pub title: NotificationString,
    pub description: Option<NotificationString>,
}

pub type AppEventsBroadcaster = broadcast::Sender<AppEvent>;

#[tracing::instrument(level = "trace")]
pub fn create_app_events_broadcaster() -> AppEventsBroadcaster {
    tracing::trace!(BROADCAST_CAPACITY);
    let broadcaster = AppEventsBroadcaster::new(BROADCAST_CAPACITY);
    tracing::trace!("broadcaster created");
    broadcaster
}
