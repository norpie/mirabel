use std::{sync::Arc, time::Duration};

use crate::{
    dto::session::event::SessionEvent, model::user::User, prelude::*,
    service::sessions::SessionService,
};

use actix_web::{
    HttpRequest, HttpResponse, Scope, get,
    web::{self, Data, Path},
};
use actix_ws::Message;
use futures_util::StreamExt;
use log::{debug, error};
use surrealdb::Uuid;
use tokio::{
    sync::{Mutex, RwLock},
    time::Instant,
};

use crate::handler::middleware::auth_middleware::Auth;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/session").wrap(Auth).service(session_socket));
}

#[get("/{session_id}")]
pub async fn session_socket(
    req: HttpRequest,
    stream: web::Payload,
    session_service: Data<RwLock<SessionService>>,
    user: User,
    session_id: Path<String>,
) -> Result<HttpResponse> {
    debug!("WebSocket connection for session: {}", session_id);
    let (res, mut socket, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let alive = Arc::new(Mutex::new(Instant::now()));

    let mut socket2 = socket.clone();
    let socket3 = socket.clone();
    let socket_id = Uuid::new_v4();
    session_service
        .write()
        .await
        .add_socket(&session_id, (socket_id, Mutex::new(socket3)))
        .await?;
    let alive2 = alive.clone();
    let session_id2 = session_id.clone();
    actix_web::rt::spawn(async move {
        let session_id = session_id2;
        let mut interval = actix_web::rt::time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            debug!("Pinging WebSocket for session: {}", &session_id);
            if socket2.ping(b"").await.is_err() {
                debug!("WebSocket ping failed for session: {}", &session_id);
                break;
            }

            if Instant::now().duration_since(*alive2.lock().await) > Duration::from_secs(10) {
                debug!(
                    "WebSocket session {} is inactive, closing connection",
                    session_id
                );
                let _ = socket2.close(None).await;
                break;
            }
        }
    });

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
                    if let Err(e) = result {
                        error!("Error handling message: {:?}", e);
                        let json_error =
                            serde_json::to_string_pretty(&SessionEvent::error()).unwrap();
                        if let Err(e) = socket.text(json_error).await {
                            error!("Error sending error message: {:?}", e);
                        }
                        break;
                    }
                }
                Message::Ping(bytes) => {
                    debug!("Received ping for session: {}", &session_id);
                    if let Err(e) = socket.pong(&bytes).await {
                        error!("Error responding to ping: {:?}", e);
                        break;
                    }
                }
                Message::Pong(_) => {
                    debug!("Received pong for session: {}", &session_id);
                    *alive.lock().await = Instant::now();
                }
                Message::Close(_reason) => {
                    debug!("WebSocket connection closed for session: {}", &session_id);
                    break;
                }
                _ => {}
            }
        }

        session_service
            .write()
            .await
            .remove_socket(&session_id, socket_id)
            .await
            .unwrap_or_else(|e| error!("Error removing socket: {:?}", e));
        let _ = socket.close(None).await;
        debug!("WebSocket loop ended for session: {}", session_id);
    });

    Ok(res)
}

pub async fn handle_json_message(
    session_service: Data<RwLock<SessionService>>,
    session_id: &Path<String>,
    user: &User,
    json: String,
) -> Result<()> {
    let parsed = serde_json::from_str::<SessionEvent>(&json)?;
    session_service
        .read()
        .await
        .handle_event(session_id, user, parsed)
        .await
}
