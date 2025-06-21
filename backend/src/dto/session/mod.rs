use serde::{Deserialize, Serialize};

use crate::{model::{chat::{Chat, ChatParticipant}, plan::Plan, session::Session}, repository::traits::Entity};

pub mod event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSession {
    pub id: String,
    pub title: String,
    pub participants: Vec<ChatParticipant>,
    pub chat: Chat,
    pub plan: Option<Plan>,
    pub terminal: Option<Vec<String>>,
    pub archived: bool,
    // pub actions: Vec<Action>,
}

impl From<Session> for FullSession {
    fn from(session: Session) -> Self {
        Self {
            id: session.id().unwrap_or_else(|| "unknown".to_string()),
            title: session.title,
            participants: session.participants,
            chat: session.chat,
            plan: session.plan,
            terminal: session.terminal,
            archived: session.archived,
        }
    }
}
