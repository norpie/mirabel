use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::{Insertable, Queryable},
    serialize::ToSql,
    sql_types::Jsonb,
};
use serde::{Deserialize, Serialize};

use crate::driver::id::id;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Insertable,
)]
#[diesel(belongs_to(Session))]
#[diesel(table_name = crate::schema::timeline_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct TimelineEntry {
    pub id: String,
    pub session_id: String,
    pub content: TimelineEntryContent,
    pub created_at: DateTime<Utc>,
    pub content_type: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum TimelineEntryContent {
    #[serde(rename_all = "camelCase")]
    Message {
        sender: MessageSender,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Acknowledgment { ack_type: AcknowledgmentType },
    #[serde(rename_all = "camelCase")]
    AgentStatus { status: AgentStatus },
    #[serde(rename_all = "camelCase")]
    Prompt {
        prompt_id: String,
        options: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    PromptResponse { prompt_id: String, response: String },
    #[serde(rename_all = "camelCase")]
    Action {
        action_type: ActionType,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Spec { content: String },
    #[serde(rename_all = "camelCase")]
    Plan { placeholder: bool },
    #[serde(rename_all = "camelCase")]
    Shell { lines: Vec<String> },
}

impl FromSql<Jsonb, Pg> for TimelineEntryContent {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        serde_json::from_value(value).map_err(|e| e.into())
    }
}

impl ToSql<Jsonb, Pg> for TimelineEntryContent {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}
