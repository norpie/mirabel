use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::repository::traits::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkspace {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedWorkspace {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[named_struct]
pub struct Workspace {
    id: Option<Thing>,
    name: String,
    avatar: Option<String>,
}

impl Entity for Workspace {
    type ID = String;

    fn id(&self) -> Option<Self::ID> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }
}

impl Workspace {
    pub fn new(name: String) -> Self {
        Self { id: None, name, avatar: None }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_avatar(&mut self, path: String) {
        self.avatar = Some(path);
    }

    pub fn avatar(&self) -> Option<&String> {
        self.avatar.as_ref()
    }
}
