# LLM Gateway

A unified Rust library for interacting with Large Language Model (LLM) providers with comprehensive cost tracking and usage analytics.

## Overview

LLM Gateway provides a consistent interface for accessing different LLM providers while handling the complexity of varying billing models, cost structures, and usage tracking capabilities. Whether you're using free local models (Ollama) or paid cloud services (OpenAI, Anthropic), this library provides unified cost reporting and usage analytics.

## Features

- **Unified API**: Single interface for multiple LLM providers
- **Cost Tracking**: Support for diverse billing models including token-based, request-based, and caching costs  
- **Usage Analytics**: Detailed metrics and metadata for all LLM interactions
- **Provider Abstraction**: Easy to add new LLM providers via trait system
- **Parameter Validation**: Range checking and validation for all LLM parameters
- **Builder Pattern**: Flexible, discoverable parameter setting
- **Error Classification**: Retryable vs non-retryable error handling

## Supported Providers

### Current
- **Ollama** - Local LLM inference with comprehensive parameter support

### Planned
- **OpenAI** - GPT models with token-based billing
- **Anthropic** - Claude models with prompt caching support
- **Google** - Gemini models
- **Groq** - High-speed inference
- **Azure OpenAI** - Enterprise OpenAI deployment
- **vLLM** - High-performance inference server

## Project Structure

```
llm-gateway/
├── src/
│   ├── lib.rs                  # Main exports and public API
│   ├── client.rs               # LlmClient with builder pattern
│   ├── traits.rs               # Core traits (LlmProvider)
│   ├── types.rs                # Request/Response types with comprehensive parameters
│   ├── error.rs                # Error handling with provider mapping
│   ├── cost/                   # Cost tracking system
│   │   ├── mod.rs              
│   │   └── types.rs            # Cost structure types for different billing models
│   └── providers/              
│       ├── mod.rs              
│       └── ollama.rs           # Complete Ollama implementation
```

## Core API Design

### Main Client

```rust
use llm_gateway::{LlmClient, LlmParameters};

// Create client with builder pattern
let client = LlmClient::builder()
    .with_provider("ollama")
    .build()
    .await?;

// Configure parameters with builder
let params = LlmParameters::builder()
    .temperature(0.7)
    .max_tokens(100)
    .top_k(40)
    .stop_sequences(vec!["END".to_string()])
    .build();

// Generate text  
let response = client.generate("Why is the sky blue?", params).await?;
```

### Core Types

```rust
// Comprehensive parameter support
pub struct LlmParameters {
    // Core parameters
    pub temperature: Option<f32>,
    pub top_k: Option<u32>, 
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
    pub typical_p: Option<f32>,
    pub max_tokens: Option<u32>,
    pub context_length: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    
    // Advanced parameters
    pub seed: Option<i32>,
    pub num_predict: Option<i32>,
    pub repeat_last_n: Option<i32>,
    pub repetition_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    
    // Performance parameters
    pub num_gpu: Option<u32>,
    pub num_thread: Option<u32>,
    pub numa: Option<bool>,
    pub num_batch: Option<u32>,
    pub main_gpu: Option<u32>,
    pub use_mmap: Option<bool>,
    
    // Provider-specific parameters
    pub custom_parameters: Option<HashMap<String, serde_json::Value>>,
}

// Response with cost tracking
pub struct LlmResponse {
    pub content: String,
    pub metadata: ResponseMetadata,
    pub usage: UsageMetrics,
    pub cost: UsageCost,
}
```

## Cost Structure System

The library supports various billing models:

### Billing Models

- **Free** - Ollama, local models (implemented with "FREE" currency)
- **Pay Per Token** - OpenAI, Anthropic (input/output token pricing)  
- **Pay Per Request** - Some API services (fixed cost per call)
- **Subscription** - Monthly/yearly plans
- **Hybrid** - Combinations of the above

### Example Cost Breakdowns

**Ollama (Free Provider - Currently Implemented):**
```rust
UsageCost {
    total_cost: None,
    breakdown: CostBreakdown { /* no costs */ },
    currency: "FREE",
}
```

**Future Anthropic (Paid Provider - Planned):**
```rust
UsageCost {
    total_cost: Some(Decimal::from_str("0.00045")?),
    breakdown: CostBreakdown {
        input_tokens: TokenCost { count: 100, cost_per_token: Some(...), ... },
        output_tokens: TokenCost { count: 50, cost_per_token: Some(...), ... },
        cached_tokens: Some(CachedTokenCost { ... }),
        // ...
    },
    currency: "USD",
}
```

## Provider Trait

New providers implement the `LlmProvider` trait:

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str, params: &LlmParameters) -> Result<LlmResponse>;
    fn cost_capabilities(&self) -> CostCapabilities;
    fn calculate_cost(&self, usage: &UsageMetrics, model: &str) -> UsageCost;
}
```

## Related Projects

- [ollama-rs](https://crates.io/crates/ollama-rs) - Ollama Rust SDK (used internally)

---

**Note**: This library is designed to be provider-agnostic and cost-aware. The goal is to provide a single, consistent interface for LLM interactions while maintaining detailed cost and usage tracking across different billing models.