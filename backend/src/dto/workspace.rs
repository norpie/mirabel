use serde::{Deserialize, Serialize};

use crate::{model::workspace::Workspace, repository::traits::Entity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendWorkspace {
    id: String,
    name: String,
    avatar: Option<String>,
}

impl From<Workspace> for FrontendWorkspace {
    fn from(value: Workspace) -> Self {
        Self {
            id: value.id().unwrap(),
            name: value.name().to_owned(),
            avatar: value.avatar().cloned(),
        }
    }
}
