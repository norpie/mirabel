use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::driver::id::id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub id: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub content: SessionEventContent,
}

impl SessionEvent {
    pub fn new(content: SessionEventContent) -> Self {
        SessionEvent {
            id: id!(),
            source: "system".to_string(),
            timestamp: Utc::now(),
            content,
        }
    }

    pub fn acknowledgment(ack_type: AcknowledgmentType) -> Self {
        SessionEvent {
            id: id!(),
            source: "system".to_string(),
            timestamp: Utc::now(),
            content: SessionEventContent::AcknowledgmentContent { ack_type },
        }
    }

    pub fn error() -> Self {
        SessionEvent {
            id: id!(),
            source: "system".to_string(),
            timestamp: Utc::now(),
            content: SessionEventContent::AcknowledgmentContent {
                ack_type: AcknowledgmentType::Error,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcknowledgmentType {
    Delivered,
    Seen,
    Thinking,
    Typing,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SessionEventContent {
    AcknowledgmentContent {
        #[serde(rename = "ackType")]
        ack_type: AcknowledgmentType,
    },
    MessageContent {
        #[serde(rename = "authorId")]
        author_id: String,
        message: String,
    },
    AgentActionContent {
        action: String,
        description: Option<String>,
    },
    AgentPromptContent {
        #[serde(rename = "promptId")]
        prompt_id: String,
        prompt: String,
        options: Vec<String>,
        #[serde(default)]
        allow_other: Option<bool>,
    },
    UserPromptResponseContent {
        #[serde(rename = "promptId")]
        prompt_id: String,
        response: String,
    },
    AgentNewTaskEvent {
        #[serde(rename = "taskId")]
        task_id: String,
        #[serde(rename = "parentId")]
        parent_id: String,
        description: String,
    },
    AgentTaskEvent {
        #[serde(rename = "taskId")]
        task_id: String,
        status: TaskStatus,
    },
    AgentSpecUpdateEvent {
        spec: String,
    },
    AgentTerminalContentEvent {
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
