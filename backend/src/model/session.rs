use serde::{Deserialize, Serialize};

use super::{chat::Chat, plan::Plan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: String,
    title: String,
    chat: Chat,
    plan: Plan,
}
