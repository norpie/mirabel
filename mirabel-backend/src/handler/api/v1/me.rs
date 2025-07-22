use crate::prelude::*;
use mirabel_core::dto::frontend_user::FrontendUser;

use actix_web::{
    Responder, Scope, get,
    web::{self},
};

use crate::handler::extractors::W;
use crate::handler::middleware::auth_middleware::Auth;
use mirabel_core::dto::api_response::ApiResponse;

pub mod user_workspaces;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/me")
            .wrap(Auth)
            .service(get_me)
            // .service(update_me)
            // .service(delete_me)
            .configure(user_workspaces::scope),
    );
}

#[get("")]
pub async fn get_me(user: W) -> Result<impl Responder> {
    Ok(ApiResponse::ok(FrontendUser::from(user.into_inner())))
}

// #[patch("")]
// pub async fn update_me(
//     user_service: Data<UserService>,
//     mut user: User,
//     updated_user: Json<UpdatedUser>,
// ) -> Result<impl Responder> {
//     user = user_service
//         .update_user(
//             user,
//             updated_user.username.clone(),
//             updated_user.email.clone(),
//             updated_user.password.clone(),
//             updated_user.avatar.clone(),
//         )
//         .await?;
//     Ok(ApiResponse::ok(FrontendUser::from(user)))
// }

// #[delete("")]
// pub async fn delete_me(user_service: Data<UserService>, user: User) -> Result<impl Responder> {
//     user_service.delete_user(user).await?;
//     Ok(ApiResponse::ok(()))
// }
