use actix_web::{web, Scope};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/users"));
}
