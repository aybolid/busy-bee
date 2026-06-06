use tokio::sync::broadcast;
use types::{NonEmpty, TrimmedString};

/// Represents all possible events that can be broadcast across the application.
///
/// This enum allows different components of the application to communicate
/// decoupled events to interested subscribers.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// An event signaling that a notification should be presented to the user.
    Notification(NotificationData),
    /// An event signaling that there is some data that can be refetched as it was updated.
    RefetchTrigger(RefetchTriggerType),
}

/// The type of data that can be refetched as it was updated.
#[derive(Debug, Clone, Copy)]
pub enum RefetchTriggerType {
    /// Articles related data.
    Articles,
    /// Rss feeds related data.
    RssFeeds,
    /// Article processing outputs related data.
    Outputs,
}

impl AsRef<str> for RefetchTriggerType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Articles => "articles",
            Self::RssFeeds => "rss_feeds",
            Self::Outputs => "outputs",
        }
    }
}

/// The visual or semantic category of a notification.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationVariant {
    /// A general informational message.
    Info,
    /// An error or failure state.
    Error,
}

/// A validated, non-empty string used for notification titles and descriptions.
///
/// This newtype wrapper ensures that empty or purely whitespace notifications
/// cannot be constructed or sent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct NotificationString(pub NonEmpty<TrimmedString>);

impl NotificationString {
    /// Attempts to create a new [`NotificationString`] from any type implementing [`ToString`].
    ///
    /// Returns [`None`] if the resulting string is empty or contains only whitespace.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(s: impl ToString) -> Option<Self> {
        let s = TrimmedString::from(s.to_string());
        NonEmpty::new(s).map(Self)
    }
}

/// The payload for a notification event, containing its metadata and content.
#[derive(Debug, Clone, serde::Serialize)]
pub struct NotificationData {
    /// The category/type of the notification.
    pub variant: NotificationVariant,
    /// The primary, non-empty text of the notification.
    pub title: NotificationString,
    /// Optional supporting text providing more detail.
    pub description: Option<NotificationString>,
}

impl NotificationData {
    /// Creates a fully constructed [`NotificationData`].
    pub fn new(
        variant: NotificationVariant,
        title: NotificationString,
        description: Option<NotificationString>,
    ) -> Self {
        Self {
            variant,
            title,
            description,
        }
    }

    /// Convenience method to create a new Error notification with just a title.
    pub fn error(title: NotificationString) -> Self {
        Self::new(NotificationVariant::Error, title, None)
    }

    /// Convenience method to create a new Info notification with just a title.
    pub fn info(title: NotificationString) -> Self {
        Self::new(NotificationVariant::Info, title, None)
    }

    /// Builder method to attach or replace the optional description of the notification.
    pub fn with_description(mut self, description: Option<NotificationString>) -> Self {
        self.description = description;
        self
    }
}

/// A centralized hub for broadcasting application-wide events.
///
/// This struct wraps a [`tokio::sync::broadcast::Sender`] to provide a multi-producer,
/// multi-consumer channel specifically typed for [`AppEvent`]s.
#[derive(Debug, Clone)]
pub struct AppEventsBroadcaster {
    sender: broadcast::Sender<AppEvent>,
}

impl AppEventsBroadcaster {
    /// The maximum number of unread events the channel will hold before lagging
    /// receivers are dropped or messages are missed.
    const CAPACITY: usize = 100;

    /// Creates a new [`AppEventsBroadcaster`] with the predefined capacity.
    pub fn new() -> Self {
        Self {
            sender: broadcast::Sender::new(Self::CAPACITY),
        }
    }

    /// Dispatches a notification event to all active subscribers.
    ///
    /// If there are no active receivers, the error is ignored (caught and logged as a warning),
    /// preventing the sender from crashing just because no one is listening.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn send_notification(&self, data: NotificationData) -> &Self {
        _ = self
            .sender
            .send(AppEvent::Notification(data))
            .inspect_err(|error| tracing::warn!(?error));

        self
    }

    /// Dispatches a refetch trigger event to all active subscribers.
    ///
    /// This should be used to tell clients that data needs to be refetched only when client
    /// has no other way to know that data needs to be refetched.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn send_refetch_trigger(&self, trigger_type: RefetchTriggerType) -> &Self {
        _ = self
            .sender
            .send(AppEvent::RefetchTrigger(trigger_type))
            .inspect_err(|error| tracing::warn!(?error));

        self
    }

    /// Dispatches a refetch trigger events to all active subscribers.
    ///
    /// See [`AppEventsBroadcaster::send_refetch_trigger`].
    pub fn send_refetch_triggers(
        &self,
        trigger_types: impl IntoIterator<Item = RefetchTriggerType>,
    ) -> &Self {
        for trigger_type in trigger_types {
            self.send_refetch_trigger(trigger_type);
        }

        self
    }

    /// Creates a new receiver subscribed to the event stream.
    ///
    /// Any component calling this method will receive a [`broadcast::Receiver`]
    /// that can asynchronously await new [`AppEvent`]s.
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
}
