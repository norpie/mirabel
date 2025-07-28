pub mod client;
pub mod cost;
pub mod error;
pub mod traits;
pub mod types;

#[cfg(feature = "ollama")]
pub mod providers;

// Re-export main types
pub use client::{LlmClient, LlmClientBuilder};
pub use cost::{BillingModel, CostBreakdown, CostCapabilities, UsageCost};
pub use error::{LlmError, Result};
pub use traits::{LlmProvider, StreamingProvider};
pub use types::{
    CachedTokenMetrics, GenerateRequest, GenerateResponse, GenerationParameters, ResponseMetadata, 
    UsageMetrics,
};

#[cfg(feature = "ollama")]
pub use providers::ollama::OllamaProvider;
