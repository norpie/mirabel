use async_trait::async_trait;
use chrono::Utc;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationResponse},
    Ollama,
};

use crate::{
    cost::{BillingModel, CostCapabilities, UsageCost},
    error::{LlmError, Result},
    traits::LlmProvider,
    types::{GenerateRequest, GenerateResponse, ResponseMetadata, UsageMetrics},
};

/// Ollama provider implementation
pub struct OllamaProvider {
    client: Ollama,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    /// 
    /// # Arguments
    /// * `base_url` - Optional base URL for Ollama server (defaults to http://localhost:11434)
    pub fn new(base_url: Option<String>) -> Self {
        let client = match base_url {
            Some(url) => {
                // Parse URL to extract host and port
                let url = url.trim_end_matches('/');
                if let Some(host_port) = url.strip_prefix("http://") {
                    if let Some(colon_pos) = host_port.find(':') {
                        let host = &host_port[..colon_pos];
                        let port = host_port[colon_pos + 1..]
                            .parse::<u16>()
                            .unwrap_or(11434);
                        Ollama::new(format!("http://{}", host), port)
                    } else {
                        Ollama::new(format!("http://{}", host_port), 11434)
                    }
                } else {
                    Ollama::new(url.to_string(), 11434)
                }
            }
            None => Ollama::default(),
        };

        Self { client }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse> {
        let start_time = Utc::now();

        // Convert our request to Ollama format
        let mut ollama_request = GenerationRequest::new(request.model.clone(), request.prompt.clone());

        // Add system message if provided
        if let Some(ref system) = request.system {
            ollama_request = ollama_request.system(system.clone());
        }

        // Set temperature if provided
        if let Some(temperature) = request.temperature {
            let mut options = std::collections::HashMap::new();
            options.insert("temperature".to_string(), serde_json::Value::from(temperature));
            ollama_request = ollama_request.options(options);
        }

        // Make the request
        let response: GenerationResponse = self.client.generate(ollama_request).await?;
        let end_time = Utc::now();

        // Create usage metrics (Ollama doesn't provide token counts)
        let usage = UsageMetrics::empty();

        // Calculate cost (free for Ollama)
        let cost = self.calculate_cost(&usage, &request.model);

        // Create metadata
        let metadata = ResponseMetadata {
            model: request.model.clone(),
            provider: self.provider_name().to_string(),
            request_id: None, // Ollama doesn't provide request IDs
            start_time,
            end_time,
            latency_ms: (end_time - start_time).num_milliseconds() as u64,
        };

        Ok(GenerateResponse {
            content: response.response,
            metadata,
            usage,
            cost,
        })
    }

    fn cost_capabilities(&self) -> CostCapabilities {
        CostCapabilities {
            tracks_token_usage: false,
            supports_token_caching: false,
            provides_cost_estimates: false,
            has_request_fees: false,
            billing_model: BillingModel::Free,
        }
    }

    fn calculate_cost(&self, _usage: &UsageMetrics, _model: &str) -> UsageCost {
        // Ollama is free
        UsageCost::free()
    }

    fn provider_name(&self) -> &'static str {
        "ollama"
    }

    async fn is_model_available(&self, model: &str) -> Result<bool> {
        // Try to list local models and check if the requested model exists
        match self.client.list_local_models().await {
            Ok(models) => {
                let model_name = model.to_lowercase();
                Ok(models.iter().any(|m| {
                    m.name.to_lowercase() == model_name || 
                    m.name.to_lowercase().starts_with(&format!("{}:", model_name))
                }))
            }
            Err(_) => {
                // If we can't list models, assume the model might be available
                // This could happen if Ollama is not running or accessible
                Ok(false)
            }
        }
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let models = self.client.list_local_models().await?;
        Ok(models.into_iter().map(|m| m.name).collect())
    }
}