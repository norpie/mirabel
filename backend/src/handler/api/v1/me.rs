use crate::{model::user::UpdatedUser, prelude::*, repository::Repository, service::users};

use actix_web::{
    get, patch,
    web::{self, Data, Json},
    Responder, Scope,
};

use crate::{
    dto::api_response::ApiResponse,
    handler::middleware::auth_middleware::Auth,
    model::user::{FrontendUser, User},
};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/me")
            .wrap(Auth)
            .service(get_me)
            .service(update_me),
    );
}

#[get("")]
pub async fn get_me(user: User) -> Result<impl Responder> {
    Ok(ApiResponse::ok(FrontendUser::from(user)))
}

#[patch("")]
pub async fn update_me(
    db: Data<Box<dyn Repository>>,
    user: User,
    updated_user: Json<UpdatedUser>,
) -> Result<impl Responder> {
    let user = users::update(db, user, updated_user.0).await?;
    Ok(ApiResponse::ok(FrontendUser::from(user)))
}
