use crate::prelude::*;

use actix_web::{
    HttpResponse, HttpResponseBuilder, Responder, ResponseError,
    body::BoxBody,
    http::{StatusCode, header::ContentType},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    #[serde(skip_serializing)]
    status: StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            status: StatusCode::OK,
            data: Some(data),
            error: None,
        }
    }

    pub fn as_response(&self) -> Result<HttpResponse<BoxBody>> {
        let result = serde_json::to_string(&self)?;
        let mut builder = HttpResponseBuilder::new(self.status);
        builder.content_type(ContentType::json());
        Ok(builder.body(result))
    }
}

impl ApiResponse<()>
where
    (): Serialize,
{
    pub fn error(status: StatusCode, message: String) -> Self {
        Self {
            status,
            data: None,
            error: Some(message),
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let response_res = self.as_response();
        match response_res {
            Ok(response) => response,
            Err(e) => e.error_response(),
        }
    }
}
