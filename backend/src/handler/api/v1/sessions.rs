use crate::{
    dto::session::event::SessionEvent, model::user::User, prelude::*, service::sessions::SessionService,
};

use actix_web::{
    HttpRequest, HttpResponse, Scope, get,
    web::{self, Data, Path},
};
use actix_ws::Message;
use futures_util::StreamExt;
use log::{error, debug};

use crate::handler::middleware::auth_middleware::Auth;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/session").wrap(Auth).service(session_socket));
}

#[get("/{session_id}")]
pub async fn session_socket(
    req: HttpRequest,
    stream: web::Payload,
    session_service: Data<SessionService>,
    user: User,
    session_id: Path<String>,
) -> Result<HttpResponse> {
    debug!("WebSocket connection for session: {}", session_id);
    let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    let result = handle_json_message(
                        session_service.clone(),
                        &session_id,
                        &user,
                        text.to_string(),
                    )
                    .await;
                    match result {
                        Ok(response) => {
                            if let Err(e) = session.text(response).await {
                                error!("Error sending message: {:?}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message: {:?}", e);
                            let json_error =
                                serde_json::to_string_pretty(&SessionEvent::error()).unwrap();
                            if let Err(e) = session.text(json_error).await {
                                error!("Error sending error message: {:?}", e);
                            }
                            break;
                        }
                    }
                }
                Message::Ping(bytes) => {
                    if let Err(e) = session.pong(&bytes).await {
                        error!("Error responding to ping: {:?}", e);
                        break;
                    }
                }
                Message::Close(_reason) => {
                    debug!("WebSocket connection closed for session: {}", session_id);
                    break;
                }
                _ => {}
            }
        }
        debug!("WebSocket loop ended for session: {}", session_id);
    });

    Ok(res)
}

pub async fn handle_json_message(
    session_service: Data<SessionService>,
    session_id: &Path<String>,
    user: &User,
    json: String,
) -> Result<String> {
    let parsed = serde_json::from_str::<SessionEvent>(&json)?;
    let response = session_service
        .handle_event(session_id, user, parsed)
        .await?;
    let value = serde_json::to_string_pretty(&response)?;
    Ok(value)
}
