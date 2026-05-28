use genai::{
    adapter::AdapterKind,
    chat::{ChatRequest, ChatResponse},
    resolver::{AuthData, AuthResolver},
};
use types::{NonEmpty, TrimmedString};

use crate::app::config::Config;

#[derive(Debug, Clone)]
pub struct Client {
    client: genai::Client,
    model: NonEmpty<TrimmedString>,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientInitError {
    #[error(transparent)]
    Genai(#[from] genai::Error),
    #[error("api key not found for a model that requires it")]
    ApiKeyNotFound,
}

impl Client {
    pub async fn try_new(config: &Config) -> Result<Self, ClientInitError> {
        let api_key = config.ai_api_key().as_str().to_owned();
        let auth_resolver = AuthResolver::from_resolver_fn(
            |_| -> Result<Option<AuthData>, genai::resolver::Error> {
                Ok(Some(AuthData::Key(api_key)))
            },
        );

        let client = genai::Client::builder()
            .with_auth_resolver(auth_resolver)
            .build();

        let target = client.resolve_service_target(config.ai_model()).await?;
        tracing::info!(model = ?target.model);

        if target.model.adapter_kind != AdapterKind::Ollama && config.ai_api_key().is_empty() {
            Err(ClientInitError::ApiKeyNotFound)
        } else {
            Ok(Self {
                client,
                model: config.ai_model().clone(),
            })
        }
    }

    #[tracing::instrument(level = "trace", skip_all, err(Debug))]
    pub async fn exec_chat(&self, request: ChatRequest) -> genai::Result<ChatResponse> {
        self.client
            .exec_chat(self.model.as_str(), request, None)
            .await
            .inspect(|resp| tracing::trace!(usage = ?resp.usage))
    }

    pub fn model(&self) -> &NonEmpty<TrimmedString> {
        &self.model
    }
}
