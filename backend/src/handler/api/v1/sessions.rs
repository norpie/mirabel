use crate::{dto::session::SessionEvent, model::user::User, prelude::*};

use actix_web::{
    HttpRequest, HttpResponse, Scope, get,
    web::{self, Path},
};
use actix_ws::Message;
use futures_util::StreamExt;
use log::error;

use crate::handler::middleware::auth_middleware::Auth;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/session").wrap(Auth).service(session_socket));
}

#[get("/{session_id}")]
pub async fn session_socket(
    req: HttpRequest,
    stream: web::Payload,
    user: User,
    session_id: Path<String>,
) -> Result<HttpResponse> {
    let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    let result = handle_json_message(text.to_string()).await;
                    match result {
                        Ok(response) => {
                            if let Err(e) = session.text(response).await {
                                error!("Error sending message: {:?}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message: {:?}", e);
                            break;
                        }
                    }
                }
                Message::Close(_reason) => {
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}

pub async fn handle_json_message(json: String) -> Result<String> {
    let parsed = serde_json::from_str::<SessionEvent>(&json)?;
    let value = serde_json::to_string_pretty(&parsed)?;
    return Ok(value);
}
