use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Queryable},
};

use crate::{model::workspace::Workspace, schema::sessions};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone)]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: String,
    pub workspace_id: String,
    pub user_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub archived: bool,
}

impl Session {
    pub fn new(workspace_id: String, user_id: String, title: String) -> Self {
        Self {
            id: id!(),
            workspace_id,
            user_id,
            title,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            archived: false,
        }
    }
}

// participants: Vec<ChatParticipant>,
// plan: Option<Plan>,
// chat: Chat,
// terminal: Option<Vec<String>>,
