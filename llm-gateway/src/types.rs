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
    /// Generation parameters
    pub parameters: GenerationParameters,
}

/// Comprehensive generation parameters for LLM requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParameters {
    /// Temperature for generation (0.0 to 2.0, None uses provider default)
    /// Higher values make output more random, lower values more deterministic
    pub temperature: Option<f32>,
    
    /// Maximum tokens to generate (None uses provider default)
    pub max_tokens: Option<u32>,
    
    /// Context length/window size (None uses provider default)
    /// Maximum number of tokens the model can consider
    pub context_length: Option<u32>,
    
    /// Top-k sampling: only consider the k most likely tokens (None uses provider default)
    /// Higher values increase diversity, lower values increase focus
    pub top_k: Option<u32>,
    
    /// Top-p (nucleus) sampling: only consider tokens with cumulative probability <= top_p (None uses provider default)
    /// Range: 0.0 to 1.0. Lower values increase focus, higher values increase diversity
    pub top_p: Option<f32>,
    
    /// Minimum probability threshold for tokens (None uses provider default)
    /// Tokens below this probability are filtered out
    pub min_p: Option<f32>,
    
    /// Typical probability mass for typical sampling (None uses provider default)
    /// Range: 0.0 to 1.0. Alternative to top-p sampling
    pub typical_p: Option<f32>,
    
    /// Stop sequences: generation stops when any of these sequences is encountered
    pub stop_sequences: Option<Vec<String>>,
    
    /// Repetition penalty to discourage repetitive text (None uses provider default)
    /// Values > 1.0 discourage repetition, < 1.0 encourage it
    pub repetition_penalty: Option<f32>,
    
    /// Frequency penalty to reduce likelihood of repeated tokens (None uses provider default)
    /// Range typically -2.0 to 2.0
    pub frequency_penalty: Option<f32>,
    
    /// Presence penalty to reduce likelihood of new topics (None uses provider default)
    /// Range typically -2.0 to 2.0
    pub presence_penalty: Option<f32>,
    
    /// Number of tokens to keep from beginning when truncating context
    pub num_keep: Option<u32>,
    
    /// Random seed for reproducible outputs
    pub seed: Option<u64>,
    
    /// Number of predictions to generate (alternative to max_tokens)
    pub num_predict: Option<u32>,
    
    /// Number of previous tokens to consider for repetition penalty
    pub repeat_last_n: Option<u32>,
    
    /// Whether to penalize newline characters in repetition penalty
    pub penalize_newline: Option<bool>,
    
    /// Whether to use NUMA (Non-Uniform Memory Access) optimizations
    pub numa: Option<bool>,
    
    /// Number of tokens to process in parallel (batch size)
    pub num_batch: Option<u32>,
    
    /// Number of GPU layers to use (0 = CPU only)
    pub num_gpu: Option<u32>,
    
    /// Main GPU to use for processing
    pub main_gpu: Option<u32>,
    
    /// Whether to use memory mapping for model loading
    pub use_mmap: Option<bool>,
    
    /// Number of threads to use for processing
    pub num_thread: Option<u32>,
    
    /// Provider-specific custom parameters
    /// Use this for parameters not covered by the standard fields above
    pub custom_parameters: Option<HashMap<String, serde_json::Value>>,
}

impl Default for GenerationParameters {
    fn default() -> Self {
        Self {
            temperature: None,
            max_tokens: None,
            context_length: None,
            top_k: None,
            top_p: None,
            min_p: None,
            typical_p: None,
            stop_sequences: None,
            repetition_penalty: None,
            frequency_penalty: None,
            presence_penalty: None,
            num_keep: None,
            seed: None,
            num_predict: None,
            repeat_last_n: None,
            penalize_newline: None,
            numa: None,
            num_batch: None,
            num_gpu: None,
            main_gpu: None,
            use_mmap: None,
            num_thread: None,
            custom_parameters: None,
        }
    }
}

impl GenerationParameters {
    /// Create new default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set temperature (0.0 to 1.0)
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set maximum tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set context length
    pub fn with_context_length(mut self, context_length: u32) -> Self {
        self.context_length = Some(context_length);
        self
    }

    /// Set top-k sampling
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set top-p (nucleus) sampling (0.0 to 1.0)
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set typical-p sampling (0.0 to 1.0)
    pub fn with_typical_p(mut self, typical_p: f32) -> Self {
        self.typical_p = Some(typical_p);
        self
    }

    /// Set number of tokens to keep from beginning
    pub fn with_num_keep(mut self, num_keep: u32) -> Self {
        self.num_keep = Some(num_keep);
        self
    }

    /// Set random seed for reproducible outputs
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set number of predictions to generate
    pub fn with_num_predict(mut self, num_predict: u32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    /// Set number of previous tokens for repetition penalty
    pub fn with_repeat_last_n(mut self, repeat_last_n: u32) -> Self {
        self.repeat_last_n = Some(repeat_last_n);
        self
    }

    /// Set whether to penalize newlines
    pub fn with_penalize_newline(mut self, penalize_newline: bool) -> Self {
        self.penalize_newline = Some(penalize_newline);
        self
    }

    /// Set NUMA optimization
    pub fn with_numa(mut self, numa: bool) -> Self {
        self.numa = Some(numa);
        self
    }

    /// Set batch size for parallel processing
    pub fn with_num_batch(mut self, num_batch: u32) -> Self {
        self.num_batch = Some(num_batch);
        self
    }

    /// Set number of GPU layers
    pub fn with_num_gpu(mut self, num_gpu: u32) -> Self {
        self.num_gpu = Some(num_gpu);
        self
    }

    /// Set main GPU to use
    pub fn with_main_gpu(mut self, main_gpu: u32) -> Self {
        self.main_gpu = Some(main_gpu);
        self
    }

    /// Set memory mapping usage
    pub fn with_use_mmap(mut self, use_mmap: bool) -> Self {
        self.use_mmap = Some(use_mmap);
        self
    }

    /// Set number of threads
    pub fn with_num_thread(mut self, num_thread: u32) -> Self {
        self.num_thread = Some(num_thread);
        self
    }

    /// Add a custom parameter
    pub fn with_custom_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.custom_parameters
            .get_or_insert_with(HashMap::new)
            .insert(key, value);
        self
    }

    /// Set custom parameters
    pub fn with_custom_parameters(mut self, custom_parameters: HashMap<String, serde_json::Value>) -> Self {
        self.custom_parameters = Some(custom_parameters);
        self
    }

    /// Set stop sequences
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Add a single stop sequence
    pub fn with_stop_sequence(mut self, stop_sequence: impl Into<String>) -> Self {
        self.stop_sequences
            .get_or_insert_with(Vec::new)
            .push(stop_sequence.into());
        self
    }

    /// Set repetition penalty
    pub fn with_repetition_penalty(mut self, repetition_penalty: f32) -> Self {
        self.repetition_penalty = Some(repetition_penalty);
        self
    }

    /// Set frequency penalty (-2.0 to 2.0)
    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Set presence penalty (-2.0 to 2.0)
    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    /// Validate parameter ranges and return validation errors
    pub fn validate(&self) -> Result<(), String> {
        if let Some(temp) = self.temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err("Temperature must be between 0.0 and 2.0".to_string());
            }
        }

        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err("Top-p must be between 0.0 and 1.0".to_string());
            }
        }

        if let Some(min_p) = self.min_p {
            if !(0.0..=1.0).contains(&min_p) {
                return Err("Min-p must be between 0.0 and 1.0".to_string());
            }
        }

        if let Some(freq_penalty) = self.frequency_penalty {
            if !(-2.0..=2.0).contains(&freq_penalty) {
                return Err("Frequency penalty must be between -2.0 and 2.0".to_string());
            }
        }

        if let Some(pres_penalty) = self.presence_penalty {
            if !(-2.0..=2.0).contains(&pres_penalty) {
                return Err("Presence penalty must be between -2.0 and 2.0".to_string());
            }
        }

        if let Some(typical_p) = self.typical_p {
            if !(0.0..=1.0).contains(&typical_p) {
                return Err("Typical-p must be between 0.0 and 1.0".to_string());
            }
        }

        if let Some(rep_penalty) = self.repetition_penalty {
            if rep_penalty < 0.0 {
                return Err("Repetition penalty must be positive".to_string());
            }
        }

        Ok(())
    }
}
impl GenerateRequest {
    /// Create a new generate request with model and prompt
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            system: None,
            parameters: GenerationParameters::default(),
        }
    }

    /// Set system message
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set generation parameters
    pub fn with_parameters(mut self, parameters: GenerationParameters) -> Self {
        self.parameters = parameters;
        self
    }

    /// Set temperature (shorthand)
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.parameters.temperature = Some(temperature);
        self
    }

    /// Set max tokens (shorthand)
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.parameters.max_tokens = Some(max_tokens);
        self
    }

    /// Set context length (shorthand)
    pub fn with_context_length(mut self, context_length: u32) -> Self {
        self.parameters.context_length = Some(context_length);
        self
    }

    /// Set top-k (shorthand)
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.parameters.top_k = Some(top_k);
        self
    }

    /// Set top-p (shorthand)
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.parameters.top_p = Some(top_p);
        self
    }

    /// Set min-p (shorthand)
    pub fn with_min_p(mut self, min_p: f32) -> Self {
        self.parameters.min_p = Some(min_p);
        self
    }

    /// Add stop sequence (shorthand)
    pub fn with_stop_sequence(mut self, stop_sequence: impl Into<String>) -> Self {
        self.parameters.stop_sequences
            .get_or_insert_with(Vec::new)
            .push(stop_sequence.into());
        self
    }

    /// Validate the request parameters
    pub fn validate(&self) -> Result<(), String> {
        self.parameters.validate()
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
