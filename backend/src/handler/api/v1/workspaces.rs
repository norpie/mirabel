use crate::{
    dto::{
        api_response::ApiResponse, avatar::Avatar, page::PageRequest, session::FullSession,
        updated_session::UpdatedSession,
    },
    model::user::User,
    prelude::*,
    repository::traits::Entity,
    service::workspaces,
};

use actix_web::{
    Responder, Scope, delete, get, patch, post,
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
            .service(set_workspace_avatar)
            .service(get_user_workspace_sessions)
            .service(create_workspace_session)
            .configure(sessions::scope)
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
            .get_workspace_by_id(user.id().unwrap(), workspace_id.to_string())
            .await?,
    ))
}

#[post("/avatar")]
pub async fn set_workspace_avatar(
    workspace_service: Data<WorkspaceService>,
    user: User,
    workspace_id: Path<String>,
    avatar: Json<Avatar>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .set_workspace_avatar(workspace_id.to_string(), user.id().unwrap(), avatar.into_inner().path)
            .await?,
    ))
}

#[get("/session")]
pub async fn get_user_workspace_sessions(
    workspace_service: Data<WorkspaceService>,
    user: User,
    workspace_id: Path<String>,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_user_workspace_sessions(workspace_id.to_string(), user.id().unwrap(), page.into_inner())
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
    workspace_service: Data<WorkspaceService>,
    user: User,
    workspace_id: Path<String>,
    input: Json<SessionInput>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(Into::<FullSession>::into(
        workspace_service
            .create_workspace_session(user, workspace_id.to_string(), input.0.input)
            .await?,
    )))
}
