# LLM Gateway

A unified Rust library for interacting with Large Language Model (LLM) providers with comprehensive cost tracking and usage analytics.

## Overview

LLM Gateway provides a consistent interface for accessing different LLM providers while handling the complexity of varying billing models, cost structures, and usage tracking capabilities. Whether you're using free local models (Ollama) or paid cloud services (OpenAI, Anthropic), this library provides unified cost reporting and usage analytics.

## Features

- **Unified API**: Single interface for multiple LLM providers
- **Comprehensive Cost Tracking**: Support for diverse billing models including token-based, request-based, and caching costs
- **Usage Analytics**: Detailed metrics and metadata for all LLM interactions
- **Provider Abstraction**: Easy to add new LLM providers
- **Async/Await Support**: Built for modern Rust async applications
- **Type Safety**: Strongly typed requests and responses

## Supported Providers

### Current
- **Ollama** - Local LLM inference (free)

### Planned
- **OpenAI** - GPT models with token-based billing
- **Anthropic** - Claude models with prompt caching support
- **Google** - Gemini models
- **Groq** - High-speed inference
- **Azure OpenAI** - Enterprise OpenAI deployment

## Project Structure

```
llm-gateway/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Main exports and public API
│   ├── client.rs           # LlmClient - main interface
│   ├── traits.rs           # Core traits (LlmProvider)
│   ├── types.rs            # Request/Response types
│   ├── error.rs            # Error handling
│   ├── cost/               # Cost tracking system
│   │   ├── mod.rs
│   │   ├── types.rs        # Cost structure types
│   │   └── calculator.rs   # Cost calculation logic
│   └── providers/
│       ├── mod.rs
│       └── ollama.rs       # Ollama adapter using ollama-rs
```

## Core API Design

### Main Client

```rust
use llm_gateway::{LlmClient, GenerateRequest};

// Create client for Ollama
let client = LlmClient::ollama(None); // Uses default localhost:11434

// Generate text
let request = GenerateRequest::new("llama2:latest", "Why is the sky blue?");
let response = client.generate(request).await?;

println!("Response: {}", response.content);
println!("Cost: {:?}", response.cost);
println!("Usage: {:?}", response.usage);
```

### Core Types

```rust
// Main request type
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub system: Option<String>,
    pub temperature: Option<f32>,
}

// Main response type
pub struct GenerateResponse {
    pub content: String,
    pub metadata: ResponseMetadata,
    pub usage: UsageMetrics,
    pub cost: UsageCost,
}

// Usage tracking
pub struct UsageMetrics {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub cached_tokens: Option<CachedTokenMetrics>,
    pub provider_specific: HashMap<String, serde_json::Value>,
}

// Cost tracking
pub struct UsageCost {
    pub total_cost: Option<Decimal>,
    pub breakdown: CostBreakdown,
    pub currency: String,
}
```

## Cost Structure System

The library supports various billing models across different providers:

### Billing Models

- **Free** - Ollama, local models (no cost tracking needed)
- **Pay Per Token** - OpenAI, Anthropic (input/output token pricing)
- **Pay Per Request** - Some API services (fixed cost per call)
- **Subscription** - Monthly/yearly plans
- **Hybrid** - Combinations of the above

### Cost Capabilities

Each provider exposes its cost tracking capabilities:

```rust
pub struct CostCapabilities {
    pub tracks_token_usage: bool,        // Can count tokens
    pub supports_token_caching: bool,    // Has caching with separate billing
    pub provides_cost_estimates: bool,   // Can calculate costs
    pub has_request_fees: bool,          // Charges per request
    pub billing_model: BillingModel,
}
```

### Example Cost Breakdowns

**Ollama (Free Provider):**
```rust
UsageCost {
    total_cost: None,
    breakdown: CostBreakdown {
        input_tokens: TokenCost { count: 0, cost_per_token: None, total_cost: None },
        output_tokens: TokenCost { count: 0, cost_per_token: None, total_cost: None },
        cached_tokens: None,
        request_cost: None,
        additional_costs: HashMap::new(),
    },
    currency: "FREE",
}
```

**Future Anthropic (Paid Provider):**
```rust
UsageCost {
    total_cost: Some(Decimal::from_str("0.00045")?),
    breakdown: CostBreakdown {
        input_tokens: TokenCost { 
            count: 100, 
            cost_per_token: Some(Decimal::from_str("0.000003")?), 
            total_cost: Some(Decimal::from_str("0.0003")?),
        },
        output_tokens: TokenCost { 
            count: 50, 
            cost_per_token: Some(Decimal::from_str("0.000015")?), 
            total_cost: Some(Decimal::from_str("0.00075")?),
        },
        cached_tokens: Some(CachedTokenCost { 
            cache_read_tokens: TokenCost { count: 20, cost_per_token: Some(...), ... },
            cache_write_tokens: TokenCost { count: 0, cost_per_token: Some(...), ... },
            cache_creation_cost: None,
        }),
        request_cost: None,
        additional_costs: HashMap::new(),
    },
    currency: "USD",
}
```

## Provider Trait

New providers implement the `LlmProvider` trait:

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse>;
    fn cost_capabilities(&self) -> CostCapabilities;
    fn calculate_cost(&self, usage: &UsageMetrics, model: &str) -> UsageCost;
}
```

## Dependencies

### Core Dependencies
- `async-trait` - Async trait support
- `serde` - Serialization
- `chrono` - Date/time handling
- `thiserror` - Error handling
- `rust_decimal` - Precise decimal arithmetic for costs

### Provider Dependencies (Feature-gated)
- `ollama-rs` - Ollama integration (feature: "ollama")
- `async-openai` - OpenAI integration (feature: "openai")

## Development Roadmap

### Phase 1: Core Foundation ✅
- [x] Project structure and planning
- [ ] Basic types and traits
- [ ] Error handling
- [ ] Ollama provider implementation

### Phase 2: Cost System
- [ ] Cost tracking types
- [ ] Usage metrics collection
- [ ] Cost calculation framework
- [ ] Provider capability system

### Phase 3: Additional Providers
- [ ] OpenAI provider
- [ ] Anthropic provider (with caching support)
- [ ] Provider-specific optimizations

### Phase 4: Advanced Features
- [ ] Streaming support
- [ ] Request batching
- [ ] Cost optimization recommendations
- [ ] Usage analytics and reporting

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Related Projects

- [ollama-rs](https://crates.io/crates/ollama-rs) - Ollama Rust SDK (used internally)
- [async-openai](https://crates.io/crates/async-openai) - OpenAI Rust SDK (planned)

---

**Note**: This library is designed to be provider-agnostic and cost-aware. The goal is to provide a single, consistent interface for LLM interactions while maintaining detailed cost and usage tracking across different billing models.