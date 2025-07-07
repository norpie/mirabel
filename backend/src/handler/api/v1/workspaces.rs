use crate::{
    dto::{api_response::ApiResponse, page::PageRequest, session::FullSession},
    model::user::User,
    prelude::*,
    service::{sessions::SessionService, workspaces},
};

use actix_web::{
    Responder, Scope, get, post,
    web::{self, Data, Json, Path, Query},
};
use serde::Deserialize;

use crate::handler::middleware::auth_middleware::Auth;

use self::workspaces::WorkspaceService;

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
    user: User,
    workspace_id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_workspace_by_id(user.id, workspace_id.to_string())
            .await?,
    ))
}

#[get("/session")]
pub async fn get_user_workspace_sessions(
    session_service: Data<SessionService>,
    user: User,
    workspace_id: Path<String>,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        session_service
            .get_user_workspace_sessions(workspace_id.to_string(), user, page.into_inner())
            .await?
            .to::<FullSession>(),
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct SessionInput {
    input: String,
}

#[post("/session")]
pub async fn create_workspace_session(
    session_service: Data<SessionService>,
    user: User,
    workspace_id: Path<String>,
    input: Json<SessionInput>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(Into::<FullSession>::into(
        session_service
            .create_workspace_session(user, workspace_id.to_string(), input.0.input)
            .await?,
    )))
}
