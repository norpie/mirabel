use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/lib/generated/")]
pub struct FrontendUser {
    id: String,
    username: String,
    email: String,
    #[serde(rename = "createdAt")]
    #[ts(rename = "createdAt")]
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
