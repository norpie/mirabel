use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: String, username: String, email: String, password: String) -> Self {
        User {
            id,
            username,
            email,
            password,
            created_at: Utc::now(),
        }
    }
}
