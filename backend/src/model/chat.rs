use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    id: String,
    participants: Vec<ChatParticipant>,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    id: String,
    timestamp: DateTime<Utc>,
    #[serde(rename = "authorId")]
    author_id: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParticipant {
    id: String,
    name: String,
    user: bool,
}
