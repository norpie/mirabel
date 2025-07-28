use async_trait::async_trait;
use chrono::Utc;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationResponse},
    models::ModelOptions,
    Ollama,
};

use crate::{
    cost::{BillingModel, CostCapabilities, UsageCost},
    error::Result,
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
                        let port = host_port[colon_pos + 1..].parse::<u16>().unwrap_or(11434);
                        Ollama::new(format!("http://{host}"), port)
                    } else {
                        Ollama::new(format!("http://{host_port}"), 11434)
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
        // Validate parameters first
        if let Err(validation_error) = request.validate() {
            return Err(crate::error::LlmError::invalid_request(validation_error));
        }

        let start_time = Utc::now();

        // Convert our request to Ollama format
        let mut ollama_request =
            GenerationRequest::new(request.model.clone(), request.prompt.clone());

        // Add system message if provided
        if let Some(ref system) = request.system {
            ollama_request = ollama_request.system(system.clone());
        }

        // Build ModelOptions from our GenerationParameters
        let mut options = ModelOptions::default();
        let params = &request.parameters;

        // Available in ollama-rs ModelOptions:
        if let Some(temperature) = params.temperature {
            options = options.temperature(temperature);
        }

        if let Some(top_k) = params.top_k {
            options = options.top_k(top_k);
        }

        if let Some(top_p) = params.top_p {
            options = options.top_p(top_p);
        }

        if let Some(repetition_penalty) = params.repetition_penalty {
            options = options.repeat_penalty(repetition_penalty);
        }

        if let Some(context_length) = params.context_length {
            options = options.num_ctx(context_length as u64);
        }

        if let Some(seed) = params.seed {
            options = options.seed(seed as i32);
        }

        if let Some(num_predict) = params.num_predict {
            options = options.num_predict(num_predict as i32);
        }

        if let Some(repeat_last_n) = params.repeat_last_n {
            options = options.repeat_last_n(repeat_last_n as i32);
        }

        if let Some(num_gpu) = params.num_gpu {
            options = options.num_gpu(num_gpu);
        }

        if let Some(num_thread) = params.num_thread {
            options = options.num_thread(num_thread);
        }

        if let Some(stop_sequences) = &params.stop_sequences {
            if !stop_sequences.is_empty() {
                options = options.stop(stop_sequences.clone());
            }
        }

        // Additional ollama-rs specific parameters
        
        // Mirostat parameters (not in our standard params but useful for Ollama)
        if let Some(custom_params) = &params.custom_parameters {
            if let Some(mirostat) = custom_params.get("mirostat") {
                if let Some(val) = mirostat.as_u64() {
                    options = options.mirostat(val as u8);
                }
            }
            
            if let Some(mirostat_eta) = custom_params.get("mirostat_eta") {
                if let Some(val) = mirostat_eta.as_f64() {
                    options = options.mirostat_eta(val as f32);
                }
            }
            
            if let Some(mirostat_tau) = custom_params.get("mirostat_tau") {
                if let Some(val) = mirostat_tau.as_f64() {
                    options = options.mirostat_tau(val as f32);
                }
            }
            
            if let Some(tfs_z) = custom_params.get("tfs_z") {
                if let Some(val) = tfs_z.as_f64() {
                    options = options.tfs_z(val as f32);
                }
            }
            
            if let Some(num_gqa) = custom_params.get("num_gqa") {
                if let Some(val) = num_gqa.as_u64() {
                    options = options.num_gqa(val as u32);
                }
            }
        }

        // Parameters not supported by ollama-rs ModelOptions (but part of Ollama API):
        // - min_p, typical_p, frequency_penalty, presence_penalty
        // - num_keep, penalize_newline, numa, num_batch, main_gpu, use_mmap
        // These could be added via custom parameters if ollama-rs supports raw parameter passing

        // Apply the options to the request
        ollama_request = ollama_request.options(options);

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
                    m.name.to_lowercase() == model_name
                        || m.name.to_lowercase().starts_with(&format!("{model_name}:"))
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
