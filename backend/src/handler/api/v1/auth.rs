use crate::{
    dto::{api_response::ApiResponse, token::AccessToken},
    model::user::AuthUser,
    prelude::*,
    repository::surrealdb::SurrealDB,
    service::auth,
};

use actix_web::{
    http::header,
    post,
    web::{self, Data, Json},
    HttpRequest, Responder, Scope,
};
use surrealdb::iam::Auth;

use crate::model::user::NewUser;

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
        .service(logout)
        .service(refresh)
}

#[post("/register")]
pub async fn register(db: Data<SurrealDB>, user: Json<AuthUser>) -> Result<impl Responder> {
    let token_pair = auth::register(db.into_inner(), user.0).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("session={}; Path=/", token_pair.refresh()),
        )))
}

#[post("/login")]
pub async fn login(db: Data<SurrealDB>, user: Json<AuthUser>) -> Result<impl Responder> {
    let token_pair = auth::login(db.into_inner(), user.0).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("session={}; Path=/", token_pair.refresh()),
        )))
}

#[post("/logout")]
pub async fn logout(req: HttpRequest) -> Result<impl Responder> {
    Ok(ApiResponse::ok("Logged out successfully")
        .as_response()?
        .customize()
        .insert_header((header::SET_COOKIE, "session=; Max-Age=0; Path=/")))
}

#[post("/refresh")]
pub async fn refresh() -> Result<impl Responder> {
    Ok(ApiResponse::ok("Hello, World!"))
}
