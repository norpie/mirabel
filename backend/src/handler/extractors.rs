use crate::prelude::*;

use std::{future::Future, pin::Pin};

use actix_web::{web::Data, FromRequest, HttpMessage, HttpRequest};
use log::info;

use crate::{model::user::User, repository::Repository};

use super::Error;

impl FromRequest for User {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let id_opt = req.extensions().get::<String>().cloned();
        let db_opt = req.app_data::<Data<Box<dyn Repository>>>().cloned();

        Box::pin(async move {
            let id = id_opt.ok_or(Error::Unauthorized("Invalid Token".into()))?;
            info!("id: {}", id);
            let db = db_opt.ok_or(Error::InternalServer)?;

            db.get_user_by_id(id)
                .await?
                .ok_or(Error::Unauthorized("Invalid Token".into()))
        })
    }
}
