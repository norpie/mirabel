use crate::{dto::api_response::ApiResponse, prelude::*};

use actix_web::{get, web, Responder, Scope};

pub mod auth;
pub mod me;
pub mod users;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(hello)
        .service(auth::scope())
        .service(me::scope())
        .service(users::scope())
}

#[get("/hello")]
pub async fn hello() -> Result<impl Responder> {
    let random: bool = rand::random();
    if random {
        return Err("Random error".into());
    }
    Ok(ApiResponse::ok("Hello, World!"))
}
