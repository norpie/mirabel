use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::repository::traits::Entity;

use super::{
    chat::{Chat, ChatMessage, ChatParticipant},
    plan::Plan,
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[named_struct]
pub struct Session {
    pub id: Option<Thing>,
    pub title: String,
    pub participants: Vec<ChatParticipant>,
    pub plan: Option<Plan>,
    pub chat: Chat,
    pub terminal: Option<Vec<String>>,
    pub archived: bool,
}

impl Entity for Session {
    type ID = String;

    fn id(&self) -> Option<Self::ID> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }
}

impl Session {
    pub fn new(title: String) -> Self {
        Self {
            id: None,
            title,
            plan: None,
            chat: Chat::default(),
            participants: Vec::new(),
            terminal: None,
            archived: false,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn add_participant(&mut self, user: User) {
        self.participants.push(user.into());
    }

    pub fn add_user_message(&mut self, author_id: String, message: String) {
        self.chat.add_message(ChatMessage::new(author_id, message));
    }
}
