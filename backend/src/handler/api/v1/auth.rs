use std::sync::Arc;

use crate::{
    dto::{api_response::ApiResponse, token::AccessToken},
    model::user::{AuthUser, User},
    prelude::*,
    repository::{surrealdb::SurrealDB, Repository},
    security::jwt_util::TokenFactory,
    service::{auth, security},
};

use actix_web::{
    delete,
    http::header,
    post,
    web::{self, Data, Json},
    HttpMessage, HttpRequest, Responder, Scope,
};

use crate::model::user::NewUser;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/auth").configure(routes));
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(refresh);
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
}

#[post("/register")]
pub async fn register(
    db: Data<Box<dyn Repository>>,
    user: Json<AuthUser>,
) -> Result<impl Responder> {
    let token_pair = auth::register(db, user.0).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}

#[post("/login")]
pub async fn login(db: Data<Box<dyn Repository>>, user: Json<AuthUser>) -> Result<impl Responder> {
    let token_pair = auth::login(db, user.0).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}

#[delete("/logout")]
pub async fn logout(req: HttpRequest) -> Result<impl Responder> {
    Ok(ApiResponse::ok("Logged out successfully")
        .as_response()?
        .customize()
        .insert_header((header::SET_COOKIE, "refresh=; Max-Age=0; Path=/")))
}

#[post("/refresh")]
pub async fn refresh(req: HttpRequest) -> Result<impl Responder> {
    let cookie = req
        .cookie("refresh")
        .ok_or(Error::Unauthorized("No refresh cookie".into()))?;
    let token_pair = security::refresh(cookie.value().to_string())?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()?
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}
