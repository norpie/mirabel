use backend_derive::named_struct;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::repository::traits::{Entity, FieldFindableStruct};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[named_struct]
pub struct User {
    id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub avatar: Option<String>,
}

impl Entity for User {
    type ID = String;

    fn id(&self) -> Option<Self::ID> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }
}

impl FieldFindableStruct for User {
    fn filterable_fields() -> &'static [&'static str] {
        &["email", "username"]
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        User {
            id: None,
            username,
            email,
            password,
            created_at: Utc::now(),
            avatar: None,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn set_avatar(&mut self, avatar: String) {
        self.avatar = Some(avatar);
    }
}
