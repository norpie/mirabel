use crate::{
    dto::page::PageRequest,
    model::{
        session::{NewSession, Session, UpdatedSession},
        workspace::NewWorkspace,
    },
    prelude::*,
};

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

pub async fn create_workspace_session(
    db: Data<Box<dyn Repository>>,
    new: NewSession,
) -> Result<Option<Session>> {
    db.create_session(new).await
}

pub async fn get_user_session_by_id(
    db: Data<Box<dyn Repository>>,
    _user_id: String,
    _workspace_id: String,
    id: String,
) -> Result<Option<Session>> {
    db.get_session_by_id(id).await
}

pub async fn update_user_session(
    db: Data<Box<dyn Repository>>,
    id: String,
    session: UpdatedSession,
) -> Result<Option<Session>> {
    db.update_session(id, session).await
}

pub async fn delete_user_session(
    db: Data<Box<dyn Repository>>,
    _user_id: String,
    _workspace_id: String,
    id: String,
) -> Result<Option<Session>> {
    db.delete_session(id).await
}

pub async fn get_user_workspace_sessions(
    db: Data<Box<dyn Repository>>,
    workspace_id: String,
    user_id: String,
    page: PageRequest,
) -> Result<Vec<Session>> {
    db.get_workspaces_users_sessions(workspace_id, user_id, page)
        .await
}
