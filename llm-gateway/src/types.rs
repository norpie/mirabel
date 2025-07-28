use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cost::UsageCost;

/// Request for generating text from an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// The model to use for generation
    pub model: String,
    /// The input prompt
    pub prompt: String,
    /// Optional system message/context
    pub system: Option<String>,
    /// Temperature for generation (0.0 to 1.0, None uses provider default)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate (None uses provider default)
    pub max_tokens: Option<u32>,
}

impl GenerateRequest {
    /// Create a new generate request with model and prompt
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            system: None,
            temperature: None,
            max_tokens: None,
        }
    }

    /// Set system message
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
}

/// Response from an LLM generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    /// The generated text content
    pub content: String,
    /// Metadata about the response
    pub metadata: ResponseMetadata,
    /// Usage metrics for this request
    pub usage: UsageMetrics,
    /// Cost information for this request
    pub cost: UsageCost,
}

/// Metadata about a generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// The model that generated the response
    pub model: String,
    /// The provider that handled the request
    pub provider: String,
    /// Provider-specific request ID (if available)
    pub request_id: Option<String>,
    /// When the request started
    pub start_time: DateTime<Utc>,
    /// When the request completed
    pub end_time: DateTime<Utc>,
    /// Total latency in milliseconds
    pub latency_ms: u64,
}

/// Usage metrics for a generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetrics {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total tokens (prompt + completion)
    pub total_tokens: u32,
    /// Cached token metrics (if supported by provider)
    pub cached_tokens: Option<CachedTokenMetrics>,
    /// Provider-specific usage data
    pub provider_specific: HashMap<String, serde_json::Value>,
}

impl UsageMetrics {
    /// Create new usage metrics
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
            cached_tokens: None,
            provider_specific: HashMap::new(),
        }
    }

    /// Create empty usage metrics (for providers that don't track tokens)
    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    /// Add cached token metrics
    pub fn with_cached_tokens(mut self, cached: CachedTokenMetrics) -> Self {
        self.cached_tokens = Some(cached);
        self
    }

    /// Add provider-specific data
    pub fn with_provider_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.provider_specific.insert(key, value);
        self
    }
}

/// Metrics for cached tokens (when supported by provider)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTokenMetrics {
    /// Tokens read from cache
    pub cache_read_tokens: u32,
    /// Tokens written to cache
    pub cache_write_tokens: u32,
    /// Cache hit rate (0.0 to 1.0, if available)
    pub cache_hit_rate: Option<f32>,
}

impl CachedTokenMetrics {
    /// Create new cached token metrics
    pub fn new(cache_read_tokens: u32, cache_write_tokens: u32) -> Self {
        Self {
            cache_read_tokens,
            cache_write_tokens,
            cache_hit_rate: None,
        }
    }

    /// Set cache hit rate
    pub fn with_hit_rate(mut self, hit_rate: f32) -> Self {
        self.cache_hit_rate = Some(hit_rate);
        self
    }
}