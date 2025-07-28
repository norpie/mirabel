use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a provider's cost tracking capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCapabilities {
    /// Whether the provider tracks token usage
    pub tracks_token_usage: bool,
    /// Whether the provider supports token caching with separate billing
    pub supports_token_caching: bool,
    /// Whether the provider can calculate cost estimates
    pub provides_cost_estimates: bool,
    /// Whether the provider charges per request
    pub has_request_fees: bool,
    /// The billing model used by this provider
    pub billing_model: BillingModel,
}

/// Different billing models supported by providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingModel {
    /// Free service (e.g., Ollama, local models)
    Free,
    /// Pay per token consumed (e.g., OpenAI, Anthropic)
    PayPerToken,
    /// Pay per request made (some API services)
    PayPerRequest,
    /// Subscription-based (monthly/yearly plans)
    Subscription,
    /// Combination of multiple billing models
    Hybrid(Vec<BillingModel>),
}

/// Complete cost information for a request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCost {
    /// Total cost in the specified currency (None for free services)
    pub total_cost: Option<Decimal>,
    /// Detailed breakdown of costs
    pub breakdown: CostBreakdown,
    /// Currency code (USD, EUR, tokens, FREE, etc.)
    pub currency: String,
}

impl UsageCost {
    /// Create a free usage cost (for free providers like Ollama)
    pub fn free() -> Self {
        Self {
            total_cost: None,
            breakdown: CostBreakdown::free(),
            currency: "FREE".to_string(),
        }
    }

    /// Create a usage cost with total and breakdown
    pub fn new(
        total_cost: Decimal,
        breakdown: CostBreakdown,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            total_cost: Some(total_cost),
            breakdown,
            currency: currency.into(),
        }
    }
}

/// Detailed breakdown of costs for a request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    /// Cost for input/prompt tokens
    pub input_tokens: TokenCost,
    /// Cost for output/completion tokens
    pub output_tokens: TokenCost,
    /// Cost for cached tokens (if supported)
    pub cached_tokens: Option<CachedTokenCost>,
    /// Per-request cost (if applicable)
    pub request_cost: Option<Decimal>,
    /// Provider-specific additional costs
    pub additional_costs: HashMap<String, Decimal>,
}

impl CostBreakdown {
    /// Create a free cost breakdown
    pub fn free() -> Self {
        Self {
            input_tokens: TokenCost::free(0),
            output_tokens: TokenCost::free(0),
            cached_tokens: None,
            request_cost: None,
            additional_costs: HashMap::new(),
        }
    }

    /// Create a new cost breakdown
    pub fn new(input_tokens: TokenCost, output_tokens: TokenCost) -> Self {
        Self {
            input_tokens,
            output_tokens,
            cached_tokens: None,
            request_cost: None,
            additional_costs: HashMap::new(),
        }
    }

    /// Add cached token costs
    pub fn with_cached_tokens(mut self, cached_tokens: CachedTokenCost) -> Self {
        self.cached_tokens = Some(cached_tokens);
        self
    }

    /// Add a per-request cost
    pub fn with_request_cost(mut self, cost: Decimal) -> Self {
        self.request_cost = Some(cost);
        self
    }

    /// Add an additional cost item
    pub fn with_additional_cost(mut self, name: String, cost: Decimal) -> Self {
        self.additional_costs.insert(name, cost);
        self
    }
}

/// Cost information for a set of tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCost {
    /// Number of tokens
    pub count: u32,
    /// Cost per token (None for free services)
    pub cost_per_token: Option<Decimal>,
    /// Total cost for these tokens (None for free services)
    pub total_cost: Option<Decimal>,
}

impl TokenCost {
    /// Create a free token cost
    pub fn free(count: u32) -> Self {
        Self {
            count,
            cost_per_token: None,
            total_cost: None,
        }
    }

    /// Create a paid token cost
    pub fn paid(count: u32, cost_per_token: Decimal) -> Self {
        let total_cost = Decimal::from(count) * cost_per_token;
        Self {
            count,
            cost_per_token: Some(cost_per_token),
            total_cost: Some(total_cost),
        }
    }
}

/// Cost information for cached tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTokenCost {
    /// Cost for reading tokens from cache
    pub cache_read_tokens: TokenCost,
    /// Cost for writing tokens to cache
    pub cache_write_tokens: TokenCost,
    /// One-time cache creation cost (if applicable)
    pub cache_creation_cost: Option<Decimal>,
}

impl CachedTokenCost {
    /// Create new cached token cost
    pub fn new(cache_read_tokens: TokenCost, cache_write_tokens: TokenCost) -> Self {
        Self {
            cache_read_tokens,
            cache_write_tokens,
            cache_creation_cost: None,
        }
    }

    /// Add cache creation cost
    pub fn with_creation_cost(mut self, cost: Decimal) -> Self {
        self.cache_creation_cost = Some(cost);
        self
    }
}

/// Pricing information for a specific model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    /// Model name or identifier
    pub model: String,
    /// Cost per input token
    pub input_cost_per_token: Option<Decimal>,
    /// Cost per output token
    pub output_cost_per_token: Option<Decimal>,
    /// Cost per cached input token (if supported)
    pub cached_input_cost_per_token: Option<Decimal>,
    /// Cost per request (if applicable)
    pub request_cost: Option<Decimal>,
    /// Currency for these prices
    pub currency: String,
}

impl ModelPricing {
    /// Create pricing for a free model
    pub fn free(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            input_cost_per_token: None,
            output_cost_per_token: None,
            cached_input_cost_per_token: None,
            request_cost: None,
            currency: "FREE".to_string(),
        }
    }

    /// Create pricing for a paid model
    pub fn paid(
        model: impl Into<String>,
        input_cost: Decimal,
        output_cost: Decimal,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            input_cost_per_token: Some(input_cost),
            output_cost_per_token: Some(output_cost),
            cached_input_cost_per_token: None,
            request_cost: None,
            currency: currency.into(),
        }
    }

    /// Add cached input pricing
    pub fn with_cached_input_cost(mut self, cost: Decimal) -> Self {
        self.cached_input_cost_per_token = Some(cost);
        self
    }

    /// Add request cost
    pub fn with_request_cost(mut self, cost: Decimal) -> Self {
        self.request_cost = Some(cost);
        self
    }
}