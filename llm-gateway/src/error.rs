use thiserror::Error;

/// Main error type for LLM Gateway operations
#[derive(Error, Debug)]
pub enum LlmError {
    /// Network or connection related errors
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Provider-specific errors
    #[error("Provider error ({provider}): {message}")]
    Provider {
        provider: String,
        message: String,
    },

    /// Model not found or not available
    #[error("Model '{model}' not found or not available")]
    ModelNotFound { model: String },

    /// Invalid request parameters
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },

    /// Rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    /// Authentication/authorization errors
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// Cost calculation errors
    #[error("Cost calculation error: {message}")]
    CostCalculation { message: String },

    /// Provider configuration errors
    #[error("Provider configuration error: {message}")]
    Configuration { message: String },

    /// Timeout errors
    #[error("Request timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    /// Generic errors for edge cases
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl LlmError {
    /// Create a provider-specific error
    pub fn provider(provider: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Provider {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create a model not found error
    pub fn model_not_found(model: impl Into<String>) -> Self {
        Self::ModelNotFound {
            model: model.into(),
        }
    }

    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::RateLimit {
            message: message.into(),
        }
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a cost calculation error
    pub fn cost_calculation(message: impl Into<String>) -> Self {
        Self::CostCalculation {
            message: message.into(),
        }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a timeout error
    pub fn timeout(timeout_ms: u64) -> Self {
        Self::Timeout { timeout_ms }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            LlmError::Network(_) | LlmError::RateLimit { .. } | LlmError::Timeout { .. }
        )
    }

    /// Check if this is a client error (4xx type)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            LlmError::InvalidRequest { .. }
                | LlmError::Authentication { .. }
                | LlmError::ModelNotFound { .. }
                | LlmError::Configuration { .. }
        )
    }

    /// Check if this is a server error (5xx type)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            LlmError::Provider { .. } | LlmError::Internal { .. }
        )
    }
}

/// Result type for LLM Gateway operations
pub type Result<T> = std::result::Result<T, LlmError>;

/// Convert from ollama-rs errors when the feature is enabled
#[cfg(feature = "ollama")]
impl From<ollama_rs::error::OllamaError> for LlmError {
    fn from(err: ollama_rs::error::OllamaError) -> Self {
        LlmError::provider("ollama", err.to_string())
    }
}