use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::repository::traits::Entity;

use super::user::User;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    messages: Vec<ChatMessage>,
}

impl Chat {
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    timestamp: DateTime<Utc>,
    #[serde(rename = "authorId")]
    author_id: String,
    message: String,
}

impl ChatMessage {
    pub fn new(author_id: String, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            author_id,
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParticipant {
    id: String,
    name: String,
    avatar: Option<String>,
}

impl From<User> for ChatParticipant {
    fn from(user: User) -> Self {
        Self {
            id: user.id().unwrap(),
            name: user.username,
            avatar: user.avatar,
        }
    }
}
