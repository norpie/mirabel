use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;

use crate::prelude::*;

pub(crate) mod ollama;

#[async_trait]
pub trait Llm {
    async fn generate(&self, parameters: Option<Parameters>, prompt: &str) -> Result<LlmResponse>;
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Parameters;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LlmResponse {
    pub generation: String,
    pub metadata: LlmResponseMetadata,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LlmResponseMetadata {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub prompt_token_count: u32,
    pub response_token_count: u32,
}

pub enum LlmApi {
    Ollama,
    OpenAi,
}

pub enum ModelHub {
    Ollama,
    HuggingFace,
}

pub struct ModelRepository {
    pub hub: ModelHub,
    pub repository: String,
    pub branch: String,
}

pub enum ModelType {
    Embedding,
    Coding,
    General,
    Small,
    Tool,
    Multilingual,
    Multimodal,
    Reasoning,
}

pub struct Model {
    pub name: String,
    pub model_type: ModelType,
    pub api: LlmApi,
    pub provider: ModelProvider,
}

pub enum ModelProvider {
    Remote(RemoteModel),
    Local(LocalModel),
}

pub struct RemoteModel {
    pub model: String,
}

pub struct LocalModel {
    pub repository: ModelRepository,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let gemma = Model {
            name: "Gemma 3 27B".to_string(),
            model_type: ModelType::Multimodal,
            api: LlmApi::Ollama,
            provider: ModelProvider::Local(LocalModel {
                repository: ModelRepository {
                    hub: ModelHub::Ollama,
                    repository: "gemma3".to_string(),
                    branch: "latest".to_string(),
                },
            }),
        };

        let qwen2 = Model {
            name: "Qwen 2.5 Coder 32B".to_string(),
            api: LlmApi::Ollama,
            model_type: ModelType::Coding,
            provider: ModelProvider::Local(LocalModel {
                repository: ModelRepository {
                    hub: ModelHub::Ollama,
                    repository: "qwen2.5-coder".to_string(),
                    branch: "32b".to_string(),
                },
            }),
        };

        let qwq = Model {
            name: "QwQ".to_string(),
            api: LlmApi::Ollama,
            model_type: ModelType::Reasoning,
            provider: ModelProvider::Local(LocalModel {
                repository: ModelRepository {
                    hub: ModelHub::Ollama,
                    repository: "qwq".to_string(),
                    branch: "latest".to_string(),
                },
            }),
        };
    }
}
