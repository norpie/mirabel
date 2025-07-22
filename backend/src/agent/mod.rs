use crate::driver::llm::LlmResponseMetadata;

pub mod title_generation;
pub mod router;

pub struct AgentResponse<T> {
    pub response: T,
    pub metadata: LlmResponseMetadata,
}

impl<T> AgentResponse<T> {
    pub fn new(response: T, metadata: LlmResponseMetadata) -> Self {
        AgentResponse { response, metadata }
    }
}
