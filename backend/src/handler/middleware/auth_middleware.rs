use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::{debug, error, info};

use crate::security::jwt_util::TokenFactory;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Clone, Default)]
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let Some(token) = get_token(&req) else {
            info!("No token found in request");
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };

        let Ok(token_factory) = TokenFactory::from_env() else {
            error!("No token factory found in environment");
            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::InternalServerError()
                        .finish()
                        .map_into_right_body(),
                ))
            });
        };

        let Ok(sub) = token_factory.subject(token) else {
            debug!("Invalid token in request");
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };

        req.extensions_mut().insert(sub);
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

fn get_token(req: &ServiceRequest) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .split_whitespace()
        .last()
}
