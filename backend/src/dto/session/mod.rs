use serde::{Deserialize, Serialize};

use crate::model::{chat::Chat, plan::Plan};

pub mod event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSession {
    pub id: String,
    pub title: String,
    pub chat: Chat,
    pub plan: Option<Plan>,
    pub terminal: Option<Vec<String>>,
    // pub actions: Vec<Action>,
    pub spec: Option<String>,
}
