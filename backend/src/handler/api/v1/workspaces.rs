use crate::{
    dto::{api_response::ApiResponse, avatar::Avatar, page::PageRequest},
    model::{
        session::{NewSession, UpdatedSession},
        user::User,
    },
    prelude::*,
    repository::Repository,
    service::workspaces,
};

use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path},
    Responder, Scope,
};

use crate::handler::middleware::auth_middleware::Auth;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspaces")
            .wrap(Auth)
            .service(set_workspace_avatar)
            .service(get_workspace_avatar)
            .service(get_user_workspace_sessions)
            .service(create_workspace_session)
            .service(get_workspace_session)
            .service(delete_user_session)
            .service(update_user_session),
    );
}

#[post("/{id}/avatar")]
pub async fn set_workspace_avatar(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
    avatar: Json<Avatar>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::set_workspace_avatar(db, avatar.0.path, user.id().to_string(), id.to_string())
            .await?,
    ))
}

#[get("/{id}/avatar")]
pub async fn get_workspace_avatar(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::get_workspace_avatar(db, user.id().to_string(), id.to_string()).await?,
    ))
}

#[get("/{id}/sessions")]
pub async fn get_user_workspace_sessions(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
    page: Json<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::get_user_workspace_sessions(db, user.id().to_string(), id.to_string(), page.0)
            .await?,
    ))
}

#[post("/{id}/sessions")]
pub async fn create_workspace_session(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
    new: Json<NewSession>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::create_workspace_session(db, new.0).await?,
    ))
}

#[delete("/{id}/sessions/{session_id}")]
pub async fn delete_user_session(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
    session_id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::delete_user_session(
            db,
            user.id().to_string(),
            id.to_string(),
            session_id.to_string(),
        )
        .await?,
    ))
}

#[patch("/{id}/sessions/{session_id}")]
pub async fn update_user_session(
    db: Data<Box<dyn Repository>>,
    id: Path<String>,
    session_id: Path<String>,
    session: Json<UpdatedSession>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::update_user_session(db, session_id.to_string(), session.0).await?,
    ))
}

#[get("/{id}/sessions/{session_id}")]
pub async fn get_workspace_session(
    db: Data<Box<dyn Repository>>,
    user: User,
    id: Path<String>,
    session_id: Path<String>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::get_user_session_by_id(
            db,
            user.id().to_string(),
            id.to_string(),
            session_id.to_string(),
        )
        .await?,
    ))
}
