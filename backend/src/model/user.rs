use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        NewUser {
            username,
            email,
            password,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}

impl From<User> for FrontendUser {
    fn from(user: User) -> Self {
        FrontendUser {
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    username: String,
    email: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: String,
        username: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        User {
            id,
            username,
            email,
            password,
            created_at
        }
    }
}
