use crate::{
    dto::{api_response::ApiResponse, page::PageRequest},
    model::{
        user::User,
        workspace::{self, NewWorkspace},
    },
    prelude::*,
    repository::traits::Entity,
    service::workspaces,
};

use actix_web::{
    get, post,
    web::{self, Data, Json, Query},
    Responder, Scope,
};

use self::workspaces::WorkspaceService;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspaces")
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
            .get_user_workspaces(user.id().unwrap(), page.into_inner())
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
            .create_user_workspace(user.id().unwrap(), new_workspace.into_inner())
            .await?,
    ))
}
