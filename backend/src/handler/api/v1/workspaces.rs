use crate::{
    dto::{
        api_response::ApiResponse, avatar::Avatar, page::PageRequest,
        updated_session::UpdatedSession,
    },
    model::user::User,
    prelude::*,
    repository::traits::Entity,
    service::workspaces,
};

use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path, Query},
    Responder, Scope,
};

use crate::handler::middleware::auth_middleware::Auth;

use self::workspaces::WorkspaceService;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspace")
            .wrap(Auth)
            .service(set_workspace_avatar)
            .service(get_user_workspace_sessions)
            .service(create_workspace_session)
            .service(get_workspace_session)
            .service(delete_user_session)
            .service(update_user_session),
    );
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

#[get("/{id}/sessions")]
pub async fn get_user_workspace_sessions(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_user_workspace_sessions(id.to_string(), user.id().unwrap(), page.into_inner())
            .await?,
    ))
}

#[post("/{id}/sessions")]
pub async fn create_workspace_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .create_workspace_session(user.id().unwrap(), id.to_string())
            .await?,
    ))
}

#[delete("/{id}/sessions/{session_id}")]
pub async fn delete_user_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    session_id: Path<String>,
) -> Result<impl Responder> {
    workspace_service
        .delete_user_session(user.id().unwrap(), id.to_string(), session_id.to_string())
        .await?;
    Ok(ApiResponse::ok(()))
}

#[patch("/{id}/sessions/{session_id}")]
pub async fn update_user_session(
    workspace_service: Data<WorkspaceService>,
    _id: Path<String>,
    session_id: Path<String>,
    session: Json<UpdatedSession>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .update_user_session(session_id.to_string(), session.into_inner().user_title)
            .await?,
    ))
}

#[get("/{id}/sessions/{session_id}")]
pub async fn get_workspace_session(
    workspace_service: Data<WorkspaceService>,
    user: User,
    id: Path<String>,
    session_id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspace_service
            .get_workspace_session_by_id(user.id().unwrap(), id.to_string(), session_id.to_string())
            .await?,
    ))
}
