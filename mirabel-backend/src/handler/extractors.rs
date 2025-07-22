use crate::prelude::*;
use crate::service::users::UserService;
use mirabel_core::models::user::User;

use std::future::Future;
use std::pin::Pin;

use actix_web::FromRequest;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::web::Data;

pub struct W(User);

impl FromRequest for W {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let id_opt = req.extensions().get::<String>().cloned();
        let db_opt = req.app_data::<Data<UserService>>().cloned();

        Box::pin(async move {
            let id = id_opt.ok_or(Error::Unauthorized("Invalid Token".into()))?;
            let db = db_opt.ok_or(Error::InternalServer)?;

            let user = db
                .get_user(&id)
                .await?
                .ok_or(Error::Unauthorized("Invalid Token".into()))?;
            Ok(W(user))
        })
    }
}

impl W {
    pub fn into_inner(self) -> User {
        self.0
    }
}
