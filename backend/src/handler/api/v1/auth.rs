use crate::{
    dto::{
        api_response::ApiResponse, login_user::LoginUser, register_user::RegisterUser,
        token::AccessToken,
    },
    prelude::*,
    service::auth::AuthService,
};


use actix_web::{
    HttpRequest, Responder, Scope, delete,
    http::header,
    post,
    web::{self, Data, Json},
};


pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/auth")
            .service(refresh)
            .service(register)
            .service(login)
            .service(logout),
    );
}

#[post("/register")]
pub async fn register(auth: Data<AuthService>, user: Json<RegisterUser>) -> Result<impl Responder> {
    let token_pair = auth.register(user.into_inner()).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}

#[post("/login")]
pub async fn login(auth: Data<AuthService>, user: Json<LoginUser>) -> Result<impl Responder> {
    let token_pair = auth.login(user.into_inner()).await?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}

#[delete("/logout")]
pub async fn logout(_req: HttpRequest) -> Result<impl Responder> {
    Ok(ApiResponse::ok("Logged out successfully")
        .as_response()?
        .customize()
        .insert_header((header::SET_COOKIE, "refresh=; Max-Age=0; Path=/")))
}

#[post("/refresh")]
pub async fn refresh(auth_service: Data<AuthService>, req: HttpRequest) -> Result<impl Responder> {
    let cookie = req
        .cookie("refresh")
        .ok_or(Error::Unauthorized("No refresh cookie".into()))?;
    let token_pair = auth_service.refresh(cookie.value().to_string())?;
    Ok(ApiResponse::ok(AccessToken::from(token_pair.access()))
        .as_response()?
        .customize()
        .insert_header((
            header::SET_COOKIE,
            format!("refresh={}; Path=/", token_pair.refresh()),
        )))
}
