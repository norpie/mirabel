use actix_web::Scope;
use actix_web::web;

mod v1;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/api").configure(v1::scope));
}
