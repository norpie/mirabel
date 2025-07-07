use serde::{Deserialize, Serialize};

use crate::model::{
    session::Session,
};

pub mod event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSession {
    pub id: String,
    pub title: String,
    pub terminal: Option<Vec<String>>,
    pub archived: bool,
}

impl From<Session> for FullSession {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            title: session.title,
            terminal: None,
            archived: session.archived,
        }
    }
}
