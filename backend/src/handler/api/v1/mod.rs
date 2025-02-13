use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/v1").route("/hello", web::get().to(|| async { "hello world" }))
}
