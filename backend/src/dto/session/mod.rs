use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub id: String,
    pub source: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub content: SessionEventContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEventContent {
    MessageContent {
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
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
