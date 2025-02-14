use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkspace {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedWorkspace {
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    id: String,
    name: String,
}
