use crate::prelude::*;
use mirabel_core::dto::api_response::ApiResponse;
use mirabel_core::dto::page::PageRequest;
use mirabel_core::dto::workspace::NewWorkspace;

use actix_web::Responder;
use actix_web::Scope;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Query;

use crate::handler::extractors::W;
use crate::service::workspaces::WorkspaceService;

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
    user: W,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_user_workspaces(user.into_inner(), page.into_inner())
            .await?,
    ))
}

#[post("")]
pub async fn new_user_workspace(
    workspace_service: Data<WorkspaceService>,
    user: W,
    new_workspace: Json<NewWorkspace>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .create_user_workspace(user.into_inner(), new_workspace.into_inner())
            .await?,
    ))
}
