use crate::{dto::page::PageRequest, model::workspace::NewWorkspace, prelude::*};

use actix_web::web::Data;

use crate::{model::workspace::Workspace, repository::Repository};

pub async fn create_user_workspace(
    db: Data<Box<dyn Repository>>,
    user_id: String,
    workspace: NewWorkspace,
) -> Result<Workspace> {
    db.create_workspace(user_id, workspace).await
}

pub async fn get_user_workspaces(
    db: Data<Box<dyn Repository>>,
    user_id: String,
    page: PageRequest,
) -> Result<Vec<Workspace>> {
    db.get_users_workspaces(user_id, page).await
}

pub async fn set_workspace_avatar(
    db: Data<Box<dyn Repository>>,
    path: String,
    user_id: String,
    workspace_id: String,
) -> Result<String> {
    db.set_avatar("workspace".into(), workspace_id, path).await
}

pub async fn get_workspace_avatar(
    db: Data<Box<dyn Repository>>,
    user_id: String,
    workspace_id: String,
) -> Result<Option<String>> {
    db.get_avatar("workspace".into(), workspace_id).await
}
