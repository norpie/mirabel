use crate::{
    dto::{frontend_user::FrontendUser, updated_user::UpdatedUser},
    prelude::*,
    service::users::UserService,
};

use actix_web::{
    delete, get, patch,
    web::{self, Data, Json},
    Responder, Scope,
};

use crate::{
    dto::api_response::ApiResponse, handler::middleware::auth_middleware::Auth, model::user::User,
};

pub mod user_workspaces;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/me")
            .wrap(Auth)
            .service(get_me)
            .service(update_me)
            .service(delete_me)
            .configure(user_workspaces::scope),
    );
}

#[get("")]
pub async fn get_me(user: User) -> Result<impl Responder> {
    Ok(ApiResponse::ok(FrontendUser::from(user)))
}

#[patch("")]
pub async fn update_me(
    user_service: Data<UserService>,
    mut user: User,
    updated_user: Json<UpdatedUser>,
) -> Result<impl Responder> {
    user = user_service
        .update_user(
            user,
            updated_user.username.clone(),
            updated_user.email.clone(),
            updated_user.password.clone(),
            updated_user.avatar.clone(),
        )
        .await?;
    Ok(ApiResponse::ok(FrontendUser::from(user)))
}

#[delete("")]
pub async fn delete_me(user_service: Data<UserService>, user: User) -> Result<impl Responder> {
    user_service.delete_user(user).await?;
    Ok(ApiResponse::ok(()))
}
