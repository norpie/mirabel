use actix_web::{Scope, web};

mod v1;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/api").configure(v1::scope));
}
