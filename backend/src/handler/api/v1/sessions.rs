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
use log::{debug, error, warn};
use surrealdb::Uuid;
use tokio::{
    sync::{Mutex, RwLock},
    time::Instant,
};

use crate::handler::middleware::auth_middleware::Auth;

// Constants
const PING_INTERVAL_SECS: u64 = 5;
const INACTIVE_TIMEOUT_SECS: u64 = 10;

struct MessageHandlerContext {
    socket: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    session_service: Data<RwLock<SessionService>>,
    session_id: String,
    user: User,
    socket_id: Uuid,
    alive: Arc<Mutex<Instant>>,
    session_open: Arc<Mutex<bool>>,
}

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

    let (res, socket, msg_stream) = actix_ws::handle(&req, stream)?;
    let socket_id = Uuid::new_v4();

    // Register socket with session service
    session_service
        .write()
        .await
        .add_socket(&session_id, (socket_id, Mutex::new(socket.clone())))
        .await?;

    let alive = Arc::new(Mutex::new(Instant::now()));
    let session_open = Arc::new(Mutex::new(true));

    // Spawn ping task
    spawn_ping_task(
        socket.clone(),
        session_id.clone(),
        alive.clone(),
        session_open.clone(),
    );

    // Spawn message handling task
    let context = MessageHandlerContext {
        socket: socket.clone(),
        msg_stream,
        session_service: session_service.clone(),
        session_id: session_id.to_string(),
        user,
        socket_id,
        alive,
        session_open,
    };
    spawn_message_handler(context);

    Ok(res)
}

fn spawn_ping_task(
    mut socket: actix_ws::Session,
    session_id: String,
    alive: Arc<Mutex<Instant>>,
    session_open: Arc<Mutex<bool>>,
) {
    actix_web::rt::spawn(async move {
        let mut interval = actix_web::rt::time::interval(Duration::from_secs(PING_INTERVAL_SECS));

        loop {
            interval.tick().await;

            if !*session_open.lock().await {
                debug!("Session {} is closed, stopping ping loop", session_id);
                break;
            }

            // debug!("Pinging WebSocket for session: {}", session_id);
            if socket.ping(b"").await.is_err() {
                warn!("WebSocket ping failed for session: {}", session_id);
                break;
            }

            let last_alive = *alive.lock().await;
            if Instant::now().duration_since(last_alive) > Duration::from_secs(INACTIVE_TIMEOUT_SECS) {
                warn!("WebSocket session {} is inactive, closing connection", session_id);
                let _ = socket.close(None).await;
                break;
            }
        }
    });
}

fn spawn_message_handler(mut context: MessageHandlerContext) {
    actix_web::rt::spawn(async move {
        while let Some(msg_result) = context.msg_stream.next().await {
            let msg = match msg_result {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving WebSocket message: {:?}", e);
                    break;
                }
            };

            if !handle_websocket_message(
                &mut context.socket,
                &context.session_service,
                &context.session_id,
                &context.user,
                &context.alive,
                msg,
            ).await {
                break;
            }
        }

        cleanup_session(
            &context.session_service,
            &context.session_id,
            context.socket_id,
            &context.session_open,
            &mut context.socket,
        ).await;
    });
}

async fn handle_websocket_message(
    socket: &mut actix_ws::Session,
    session_service: &Data<RwLock<SessionService>>,
    session_id: &str,
    user: &User,
    alive: &Arc<Mutex<Instant>>,
    msg: Message,
) -> bool {
    match msg {
        Message::Text(text) => {
            if let Err(e) = handle_json_message(
                session_service.clone(),
                &Path::from(session_id.to_string()),
                user,
                text.to_string(),
            ).await {
                error!("Error handling message for session {}: {:?}", session_id, e);
                send_error_response(socket).await;
                return false;
            }
        }
        Message::Ping(bytes) => {
            debug!("Received ping for session: {}", session_id);
            if let Err(e) = socket.pong(&bytes).await {
                error!("Error responding to ping for session {}: {:?}", session_id, e);
                return false;
            }
        }
        Message::Pong(_) => {
            // debug!("Received pong for session: {}", session_id);
            *alive.lock().await = Instant::now();
        }
        Message::Close(reason) => {
            debug!("WebSocket connection closed for session {}: {:?}", session_id, reason);
            return false;
        }
        _ => {
            debug!("Received unhandled message type for session: {}", session_id);
        }
    }
    true
}

async fn send_error_response(socket: &mut actix_ws::Session) {
    let error_event = SessionEvent::error();
    if let Ok(json_error) = serde_json::to_string(&error_event) {
        if let Err(e) = socket.text(json_error).await {
            error!("Error sending error message: {:?}", e);
        }
    }
}

async fn cleanup_session(
    session_service: &Data<RwLock<SessionService>>,
    session_id: &str,
    socket_id: Uuid,
    session_open: &Arc<Mutex<bool>>,
    socket: &mut actix_ws::Session,
) {
    debug!("Cleaning up WebSocket session: {}", session_id);

    *session_open.lock().await = false;

    if let Err(e) = session_service
        .write()
        .await
        .remove_socket(session_id, socket_id)
        .await
    {
        error!("Error removing socket for session {}: {:?}", session_id, e);
    }

    let _ = socket.clone().close(None).await;
    debug!("WebSocket cleanup completed for session: {}", session_id);
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
