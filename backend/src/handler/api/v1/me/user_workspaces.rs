use crate::{
    dto::{api_response::ApiResponse, page::PageRequest},
    model::{user::User, workspace::NewWorkspace},
    prelude::*,
    service::workspaces,
};

use actix_web::{
    get, post,
    web::{self, Data, Json, Query},
    Responder, Scope,
};

use crate::repository::Repository;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/workspaces")
            .service(get_user_workspaces)
            .service(new_user_workspace),
    );
}

#[get("")]
pub async fn get_user_workspaces(
    db: Data<Box<dyn Repository>>,
    user: User,
    page: Query<PageRequest>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::get_user_workspaces(db, user.id().to_string(), page.0).await?,
    ))
}

#[post("")]
pub async fn new_user_workspace(
    db: Data<Box<dyn Repository>>,
    user: User,
    new_workspace: Json<NewWorkspace>,
) -> Result<impl Responder> {
    Ok(ApiResponse::ok(
        workspaces::create_user_workspace(db, user.id().to_string(), new_workspace.0).await?,
    ))
}
