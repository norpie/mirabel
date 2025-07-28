use async_trait::async_trait;

use crate::{
    cost::{CostCapabilities, UsageCost},
    error::Result,
    types::{GenerateRequest, GenerateResponse, UsageMetrics},
};

/// Trait that all LLM providers must implement
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate text from the given request
    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse>;

    /// Get the cost tracking capabilities of this provider
    fn cost_capabilities(&self) -> CostCapabilities;

    /// Calculate the cost for the given usage and model
    fn calculate_cost(&self, usage: &UsageMetrics, model: &str) -> UsageCost;

    /// Get the provider name
    fn provider_name(&self) -> &'static str;

    /// Check if a model is available
    async fn is_model_available(&self, model: &str) -> Result<bool> {
        // Default implementation - providers can override
        let _ = model;
        Ok(true)
    }

    /// Get available models (if supported)
    async fn list_models(&self) -> Result<Vec<String>> {
        // Default implementation returns empty list
        Ok(vec![])
    }
}

/// Trait for providers that support streaming responses
#[async_trait]
pub trait StreamingProvider: LlmProvider {
    /// Stream type for this provider
    type Stream: futures::Stream<Item = Result<String>> + Send + Unpin;

    /// Generate streaming text from the given request
    async fn generate_stream(&self, request: &GenerateRequest) -> Result<Self::Stream>;
}