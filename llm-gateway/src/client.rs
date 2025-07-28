use std::sync::Arc;

use crate::{
    error::Result,
    traits::LlmProvider,
    types::{GenerateRequest, GenerateResponse},
};

/// Main client for interacting with LLM providers
pub struct LlmClient {
    provider: Arc<dyn LlmProvider>,
}

impl LlmClient {
    /// Create a new client with the given provider
    pub fn new(provider: impl LlmProvider + 'static) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Create a new client with an Arc'd provider
    pub fn with_provider(provider: Arc<dyn LlmProvider>) -> Self {
        Self { provider }
    }

    /// Generate text using the configured provider
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        self.provider.generate(&request).await
    }

    /// Get the provider name
    pub fn provider_name(&self) -> &'static str {
        self.provider.provider_name()
    }

    /// Check if a model is available
    pub async fn is_model_available(&self, model: &str) -> Result<bool> {
        self.provider.is_model_available(model).await
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        self.provider.list_models().await
    }

    /// Get cost capabilities of the provider
    pub fn cost_capabilities(&self) -> crate::cost::CostCapabilities {
        self.provider.cost_capabilities()
    }

    /// Get a reference to the underlying provider
    pub fn provider(&self) -> &Arc<dyn LlmProvider> {
        &self.provider
    }
}

impl Clone for LlmClient {
    fn clone(&self) -> Self {
        Self {
            provider: Arc::clone(&self.provider),
        }
    }
}

/// Builder for creating LlmClient instances
pub struct LlmClientBuilder {
    provider: Option<Arc<dyn LlmProvider>>,
}

impl LlmClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self { provider: None }
    }

    /// Set the provider
    pub fn provider(mut self, provider: impl LlmProvider + 'static) -> Self {
        self.provider = Some(Arc::new(provider));
        self
    }

    /// Set the provider with an Arc
    pub fn provider_arc(mut self, provider: Arc<dyn LlmProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<LlmClient> {
        let provider = self
            .provider
            .ok_or_else(|| crate::error::LlmError::configuration("No provider configured"))?;

        Ok(LlmClient::with_provider(provider))
    }
}

impl Default for LlmClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "ollama")]
impl LlmClient {
    /// Create a client configured for Ollama
    ///
    /// # Arguments
    /// * `base_url` - Optional base URL for Ollama server (defaults to http://localhost:11434)
    pub fn ollama(base_url: Option<String>) -> Self {
        let provider = crate::providers::ollama::OllamaProvider::new(base_url);
        Self::new(provider)
    }
}
