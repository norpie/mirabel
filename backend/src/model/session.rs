use chrono::DateTime;
use chrono::Utc;
use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};

use serde::Deserialize;
use serde::Serialize;

use crate::driver::id::id;
use crate::model::workspace::Workspace;
use crate::schema::sessions;

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.modified_at = Utc::now();
    }
}

// participants: Vec<ChatParticipant>,
// plan: Option<Plan>,
// chat: Chat,
// terminal: Option<Vec<String>>,
