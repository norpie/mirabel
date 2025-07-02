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

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspace")
            .wrap(Auth)
            .service(get_workspace_by_id)
            .service(set_workspace_avatar)
            .service(get_user_workspace_sessions)
            .service(create_workspace_session)
            .service(get_workspace_session)
            .service(archive_user_session)
            .service(update_user_session),
    );
}

#[get("/{id}")]
pub async fn get_workspace_by_id(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_workspace_by_id(user.id().unwrap(), id.to_string())
            .await?,
    ))
}

#[post("/{id}/avatar")]
pub async fn set_workspace_avatar(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    avatar: Json<Avatar>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .set_workspace_avatar(id.to_string(), user.id().unwrap(), avatar.into_inner().path)
            .await?,
    ))
}

#[get("/{id}/session")]
pub async fn get_user_workspace_sessions(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_user_workspace_sessions(id.to_string(), user.id().unwrap(), page.into_inner())
            .await?
            .to::<FullSession>(),
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct SessionInput {
    input: String,
}

#[post("/{id}/session")]
pub async fn create_workspace_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    input: Json<SessionInput>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(Into::<FullSession>::into(
        workspace_service
            .create_workspace_session(user, id.to_string(), input.0.input)
            .await?,
    )))
}

#[delete("/{id}/session/{session_id}")]
pub async fn archive_user_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    ids: Path<(String, String)>,
) -> Result<impl Responder> {
    let (workspace_id, session_id) = ids.into_inner();
    workspace_service
        .delete_user_session(user.id().unwrap(), workspace_id, session_id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[patch("/{id}/session/{session_id}")]
pub async fn update_user_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    ids: Path<(String, String)>,
    session: Json<UpdatedSession>,
) -> Result<impl Responder> {
    let (id, session_id) = ids.into_inner();
    Ok(ApiResponse::ok(
        workspace_service
            .update_user_session(user, id, session_id, session.into_inner().title)
            .await?,
    ))
}

#[get("/{id}/session/{session_id}")]
pub async fn get_workspace_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    ids: Path<(String, String)>,
) -> Result<impl Responder> {
    let (workspace_id, session_id) = ids.into_inner();
    let session: FullSession = workspace_service
        .get_workspace_session_by_id(user.id().unwrap(), workspace_id, session_id)
        .await?
        .into();
    Ok(ApiResponse::ok(session))
}
