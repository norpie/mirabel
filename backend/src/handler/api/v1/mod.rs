use actix_web::{Scope, web};

pub mod auth;
pub mod me;
// pub mod users;
pub mod sessions;
pub mod workspaces;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/v1")
            .configure(auth::scope)
            .configure(me::scope)
            // .configure(users::scope),
            .configure(sessions::scope)
            .configure(workspaces::scope),
    );
}
