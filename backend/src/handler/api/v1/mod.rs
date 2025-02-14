use crate::{dto::api_response::ApiResponse, prelude::*};

use actix_web::{get, web, Responder, Scope};

pub mod auth;
pub mod me;
pub mod users;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/v1").configure(auth::scope));
    cfg.service(Scope::new("/v1").configure(me::scope));
    cfg.service(Scope::new("/v1").configure(users::scope));
}
