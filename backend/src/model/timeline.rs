use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::{driver::id::id, error::Error};

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Session))]
#[diesel(table_name = crate::schema::timeline_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseTimelineEntry {
    pub id: String,
    pub session_id: String,
    pub content: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub content_type: String,
}

impl TryFrom<TimelineEntry> for DatabaseTimelineEntry {
    type Error = Error;

    fn try_from(value: TimelineEntry) -> Result<Self, Self::Error> {
        Ok(DatabaseTimelineEntry {
            id: value.id,
            session_id: value.session_id,
            content_type: value.content_type,
            content: serde_json::to_value(value.content)?,
            created_at: value.created_at,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEntry {
    pub id: String,
    pub session_id: String,
    pub content: TimelineEntryContent,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

impl TimelineEntry {
    pub fn user_message(session_id: String, content: String) -> Self {
        TimelineEntry {
            id: id!(),
            session_id,
            content: TimelineEntryContent::Message {
                sender: MessageSender::User,
                message: content,
            },
            content_type: "message".to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn agent_message(session_id: String, content: String) -> Self {
        TimelineEntry {
            id: id!(),
            session_id,
            content: TimelineEntryContent::Message {
                sender: MessageSender::Agent,
                message: content,
            },
            content_type: "message".to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn acknowledgment(session_id: String, ack_type: AcknowledgmentType) -> Self {
        TimelineEntry {
            id: id!(),
            session_id,
            content: TimelineEntryContent::Acknowledgment { ack_type },
            content_type: "acknowledgment".to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn status(session_id: String, status: AgentStatus) -> Self {
        TimelineEntry {
            id: id!(),
            session_id,
            content: TimelineEntryContent::AgentStatus { status },
            content_type: "agentStatus".to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn prompt_response(session_id: String, prompt_id: String, response: String) -> Self {
        TimelineEntry {
            id: id!(),
            session_id,
            content: TimelineEntryContent::PromptResponse {
                prompt_id,
                response,
            },
            content_type: "prompt_response".to_string(),
            created_at: Utc::now(),
        }
    }
}

impl TryFrom<DatabaseTimelineEntry> for TimelineEntry {
    type Error = Error;

    fn try_from(value: DatabaseTimelineEntry) -> Result<Self, Self::Error> {
        Ok(TimelineEntry {
            id: value.id,
            session_id: value.session_id,
            content: serde_json::from_value(value.content)?,
            content_type: value.content_type,
            created_at: value.created_at,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageSender {
    User,
    Agent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AcknowledgmentType {
    Sent,
    Delivered,
    Seen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AgentStatus {
    Thinking,
    Typing,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionType {
    Command,
    NewFile,
    EditFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum TimelineEntryContent {
    #[serde(rename_all = "camelCase")]
    Message {
        sender: MessageSender,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Acknowledgment {
        ack_type: AcknowledgmentType,
    },
    #[serde(rename_all = "camelCase")]
    AgentStatus {
        status: AgentStatus,
    },
    #[serde(rename_all = "camelCase")]
    Prompt {
        prompt_id: String,
        options: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    PromptResponse {
        prompt_id: String,
        response: String,
    },
    #[serde(rename_all = "camelCase")]
    Action {
        action_type: ActionType,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Spec {
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Plan {
        placeholder: bool,
    },
    #[serde(rename_all = "camelCase")]
    Shell {
        lines: Vec<String>,
    },
}

impl TimelineEntry {

    // IMPORTANT: Keep these in camelCase to match the JSON serialization
    pub fn type_name(&self) -> String {
        match &self.content {
            TimelineEntryContent::Message { .. } => "message".to_string(),
            TimelineEntryContent::Acknowledgment { .. } => "acknowledgment".to_string(),
            TimelineEntryContent::AgentStatus { .. } => "agentStatus".to_string(),
            TimelineEntryContent::Prompt { .. } => "prompt".to_string(),
            TimelineEntryContent::PromptResponse { .. } => "promptResponse".to_string(),
            TimelineEntryContent::Action { .. } => "action".to_string(),
            TimelineEntryContent::Spec { .. } => "spec".to_string(),
            TimelineEntryContent::Plan { .. } => "plan".to_string(),
            TimelineEntryContent::Shell { .. } => "shell".to_string(),
        }
    }
}
