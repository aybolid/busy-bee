use genai::{
    ModelIden,
    adapter::AdapterKind,
    resolver::{AuthData, AuthResolver},
};
use types::{LengthBoundedParseError, NonEmpty, TrimmedString};

use crate::{
    ai::{ChatResponse, ExecChatError, chat::ChatRequest},
    app::config::AiConfig,
};

/// Represents a validated, non-empty name of an AI model (e.g., "gpt-4o", "llama3").
///
/// This is a newtype wrapper around a `NonEmpty<TrimmedString>` to guarantee
/// the model name is well-formed throughout the application.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ModelName(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ModelName {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for ModelName {
    type Err = LengthBoundedParseError<<TrimmedString as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// Represents an authentication token or API key for an AI service.
///
/// Wraps a [`TrimmedString`] to ensure no trailing/leading whitespace causes
/// authentication failures.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiKey(pub NonEmpty<TrimmedString>);

impl std::ops::Deref for ApiKey {
    type Target = NonEmpty<TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for ApiKey {
    type Err = LengthBoundedParseError<<TrimmedString as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// A high-level client for executing AI chat requests.
///
/// This struct wraps the underlying [`genai::Client`] and binds it to a specific
/// [`ModelName`], streamlining the execution of chat requests for a targeted model.
pub struct Client {
    pub model: ModelName,
    inner: genai::Client,
}

impl Client {
    pub async fn exec_chat(&self, request: ChatRequest) -> Result<ChatResponse, ExecChatError> {
        self.inner
            .exec_chat(self.model.as_str(), request.into(), None)
            .await
            .map(ChatResponse::from)
            .map_err(ExecChatError::from)
    }
}

/// Creates and configures a new [`Client`] based on the provided [`AiConfig`].
///
/// This initialization function performs the following steps:
/// 1. Builds an underlying [`genai::Client`] with a custom authentication resolver.
/// 2. Verifies the service target for the requested model to ensure it is routable.
/// 3. Returns the high-level wrapped [`Client`].
///
/// # Errors
/// Returns an error if the service target cannot be resolved for the configured model.
pub async fn create_ai_client(config: &AiConfig) -> genai::Result<Client> {
    let inner = genai::Client::builder()
        .with_auth_resolver(genai_auth_reslover(config))
        .build();

    inner.resolve_service_target(&config.model).await?;

    Ok(Client {
        model: config.model.clone(),
        inner,
    })
}

/// Constructs an [`AuthResolver`] for the [`genai`] client.
///
/// The resolver dynamically dictates whether an API key should be injected into
/// the request based on the specific model being queried. If a model requires an
/// API key but the configuration lacks one, the resolver will return a custom error.
fn genai_auth_reslover(config: &AiConfig) -> AuthResolver {
    let api_key = config.api_key.clone();

    AuthResolver::from_resolver_fn(
        move |model: ModelIden| -> genai::resolver::Result<Option<AuthData>> {
            if is_model_requires_api_key(&model) {
                api_key
                    .as_ref()
                    .map(|key| Some(AuthData::Key(key.to_string())))
                    .ok_or_else(|| {
                        genai::resolver::Error::Custom(format!(
                            "{} requires api key to be set",
                            model.model_name
                        ))
                    })
            } else {
                Ok(None)
            }
        },
    )
}

/// Determines whether a given AI model requires an API key for authentication.
///
/// Currently, local/self-hosted adapters (like Ollama) bypass the need for an API key,
/// whereas cloud-based models generally require one.
fn is_model_requires_api_key(model: &ModelIden) -> bool {
    !matches!(model.adapter_kind, AdapterKind::Ollama)
}
