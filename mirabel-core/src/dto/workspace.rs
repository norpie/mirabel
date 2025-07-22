use serde::Deserialize;
use serde::Serialize;

use crate::models::workspace::Workspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkspace {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendWorkspace {
    id: String,
    name: String,
    avatar: Option<String>,
}

impl From<Workspace> for FrontendWorkspace {
    fn from(value: Workspace) -> Self {
        Self {
            id: value.id,
            name: value.name,
            avatar: value.avatar,
        }
    }
}
