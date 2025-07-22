use crate::prelude::*;
use mirabel_core::dto::api_response::ApiResponse;
use mirabel_core::dto::page::PageRequest;

use actix_web::Responder;
use actix_web::Scope;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::web::Query;

use serde::Deserialize;

use crate::handler::extractors::W;
use crate::handler::middleware::auth_middleware::Auth;
use crate::service::sessions::SessionService;
use crate::service::workspaces::WorkspaceService;

use super::sessions;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspace/{workspace_id}")
            .wrap(Auth)
            .service(get_workspace_by_id)
            .service(get_user_workspace_sessions)
            .service(create_workspace_session)
            .configure(sessions::scope),
    );
}

#[get("")]
pub async fn get_workspace_by_id(
    workspace_service: Data<WorkspaceService>,
    user: W,
    workspace_id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_workspace_by_id(user.into_inner().id, workspace_id.to_string())
            .await?,
    ))
}

#[get("/session")]
pub async fn get_user_workspace_sessions(
    session_service: Data<SessionService>,
    user: W,
    workspace_id: Path<String>,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        session_service
            .get_user_workspace_sessions(
                workspace_id.to_string(),
                user.into_inner(),
                page.into_inner(),
            )
            .await?,
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct SessionInput {
    input: String,
}

#[post("/session")]
pub async fn create_workspace_session(
    session_service: Data<SessionService>,
    user: W,
    workspace_id: Path<String>,
    input: Json<SessionInput>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        session_service
            .create_workspace_session(user.into_inner(), workspace_id.to_string(), input.0.input)
            .await?,
    ))
}
