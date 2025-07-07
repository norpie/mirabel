use serde::{Deserialize, Serialize};

use crate::{model::{chat::{Chat, ChatParticipant}, plan::Plan, session::Session}};

pub mod event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSession {
    pub id: String,
    pub title: String,
    pub participants: Vec<ChatParticipant>,
    pub chat: Option<Chat>,
    pub plan: Option<Plan>,
    pub terminal: Option<Vec<String>>,
    pub archived: bool,
    // pub actions: Vec<Action>,
}

impl From<Session> for FullSession {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            title: session.title,
            participants: vec![],
            chat: None,
            plan: None,
            terminal: None,
            archived: session.archived,
        }
    }
}
