use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::repository::traits::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[named_struct]
pub struct Session {
    id: Option<Thing>,
    generated_title: String,
    user_title: Option<String>,
}

impl Entity for Session {
    type ID = String;

    fn id(&self) -> Option<Self::ID> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }
}

impl Session {
    pub fn new(generated_title: String, user_title: Option<String>) -> Self {
        Self {
            id: None,
            generated_title,
            user_title,
        }
    }

    pub fn set_generated_title(&mut self, generated_title: String) {
        self.generated_title = generated_title;
    }

    pub fn set_user_title(&mut self, user_title: Option<String>) {
        self.user_title = user_title;
    }
}
