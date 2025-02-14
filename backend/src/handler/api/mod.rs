use actix_web::{web, Scope};

mod v1;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/api").configure(v1::scope));
}
