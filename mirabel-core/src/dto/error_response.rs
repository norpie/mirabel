use actix_web::ResponseError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use log::debug;
use log::error;

use crate::Error;

use super::api_response::ApiResponse;

static DEFAULT_ERROR_JSON: &str = r#"{"error": "Internal Server Error"}"#;

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = self.to_string();
        let api_response = ApiResponse::error(self.status_code(), message.clone());
        let serde_res = serde_json::to_string(&api_response);
        match serde_res {
            Ok(json) => {
                debug!("Error: {message}");
                actix_web::HttpResponse::new(self.status_code()).set_body(BoxBody::new(json))
            }
            Err(e) => {
                error!("Failed to serialize error response: {e}");
                actix_web::HttpResponse::new(self.status_code())
                    .set_body(BoxBody::new(DEFAULT_ERROR_JSON))
            }
        }
    }
}
