use crate::{
    dto::{api_response::ApiResponse, page::PageRequest, workspace::NewWorkspace},
    model::user::User,
    prelude::*,
    service::workspaces,
};

use actix_web::{
    Responder, Scope, get, post,
    web::{self, Data, Json, Query},
};

use self::workspaces::WorkspaceService;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspace")
            .service(get_user_workspaces)
            .service(new_user_workspace),
    );
}

#[get("")]
pub async fn get_user_workspaces(
    workspace_service: Data<WorkspaceService>,
    user: User,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_user_workspaces(user, page.into_inner())
            .await?,
    ))
}

#[post("")]
pub async fn new_user_workspace(
    workspace_service: Data<WorkspaceService>,
    user: User,
    new_workspace: Json<NewWorkspace>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .create_user_workspace(user, new_workspace.into_inner())
            .await?,
    ))
}
