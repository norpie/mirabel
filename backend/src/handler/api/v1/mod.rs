use crate::{dto::api_response::ApiResponse, prelude::*};

use actix_web::{get, web, Responder, Scope};

pub mod auth;
pub mod me;
pub mod users;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(Scope::new("/v1").configure(auth::scope));
    cfg.service(Scope::new("/v1").configure(me::scope));
    cfg.service(Scope::new("/v1").configure(users::scope));
}

#[get("/hello")]
pub async fn hello() -> Result<impl Responder> {
    let random: bool = rand::random();
    if random {
        return Err("Random error".into());
    }
    Ok(ApiResponse::ok("Hello, World!"))
}
