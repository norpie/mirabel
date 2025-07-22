use crate::driver::llm::LlmResponseMetadata;

pub mod router;
pub mod title_generation;

pub struct AgentResponse<T> {
    pub response: T,
    pub metadata: LlmResponseMetadata,
}

impl<T> AgentResponse<T> {
    pub fn new(response: T, metadata: LlmResponseMetadata) -> Self {
        AgentResponse { response, metadata }
    }
}
