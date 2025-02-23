use crate::{dto::api_response::ApiResponse, prelude::*};

use actix_web::{get, web, Responder, Scope};

pub mod auth;
pub mod me;
// pub mod users;
pub mod workspaces;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/v1")
            .configure(auth::scope)
            .configure(me::scope)
            // .configure(users::scope),
            .configure(workspaces::scope),
    );
}
