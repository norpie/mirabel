use crate::{prelude::*, service::users::UserService};

use std::{future::Future, pin::Pin};

use actix_web::{FromRequest, HttpMessage, HttpRequest, web::Data};

use crate::model::user::User;

use super::Error;

impl FromRequest for User {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let mut id_opt = req.extensions().get::<String>().cloned();
        if id_opt.is_none() {
            let qstring = qstring::QString::from(req.query_string());
            id_opt = qstring.get("access_token").map(|s| s.to_string());
        }
        let db_opt = req.app_data::<Data<UserService>>().cloned();

        Box::pin(async move {
            let id = id_opt.ok_or(Error::Unauthorized("Invalid Token".into()))?;
            let db = db_opt.ok_or(Error::InternalServer)?;

            db.get_user(&id)
                .await?
                .ok_or(Error::Unauthorized("Invalid Token".into()))
        })
    }
}
