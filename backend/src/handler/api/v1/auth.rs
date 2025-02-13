use actix_web::{
    web::{self, Data, Json},
    HttpRequest, Responder, Scope,
};
pub fn scope() -> Scope {
    web::scope("/auth")
}
