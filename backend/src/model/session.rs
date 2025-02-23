use serde::{Deserialize, Serialize};

use super::{chat::Chat, plan::Plan};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSession {
    workspace_id: String,
    user_id: String,
    generated_title: String,
}

impl NewSession {
    pub fn new(workspace_id: String, user_id: String, generated_title: String) -> Self {
        Self {
            workspace_id,
            user_id,
            generated_title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: String,
    workspace_id: String,
    user_id: String,
    generated_title: String,
    user_title: Option<String>,
}

impl Session {
    pub fn new(
        id: String,
        workspace_id: String,
        user_id: String,
        generated_title: String,
        user_title: Option<String>,
    ) -> Self {
        Self {
            id,
            workspace_id,
            user_id,
            generated_title,
            user_title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedSession {
    id: String,
    generated_title: Option<String>,
    user_title: Option<String>,
}
