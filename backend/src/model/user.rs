use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(email: String, password: String) -> Self {
        NewUser {
            email,
            password,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedUser {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct User {
    id: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}

impl From<User> for FrontendUser {
    fn from(user: User) -> Self {
        FrontendUser {
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    email: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: String, email: String, password: String, created_at: DateTime<Utc>) -> Self {
        User {
            id,
            email,
            password,
            created_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
