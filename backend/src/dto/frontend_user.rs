use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{model::user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    id: String,
    username: String,
    email: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

impl From<User> for FrontendUser {
    fn from(user: User) -> Self {
        FrontendUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
