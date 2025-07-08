use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

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
pub struct TimelineEntry {
    pub id: String,
    pub session_id: String,
    pub content: TimelineEntryContent,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
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
#[serde(rename_all = "snake_case")]
pub enum MessageSender {
    User,
    Agent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Command,
    NewFile,
    EditFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TimelineEntryContent {
    Message {
        sender: MessageSender,
    },
    Action {
        action_type: ActionType,
        message: String,
    },
}

impl TimelineEntry {
    pub fn type_name(&self) -> String {
        match &self.content {
            TimelineEntryContent::Message { .. } => "message".to_string(),
            TimelineEntryContent::Action { .. } => "action".to_string(),
        }
    }
}
