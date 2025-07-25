use mirabel_core::dto::session::FullSession;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use crate::prelude::*;
use mirabel_core::dto::api_response::ApiResponse;
use mirabel_core::dto::page::CursorPageRequest;
use mirabel_core::dto::updated_session::UpdatedSession;

use crate::handler::extractors::W;
use crate::service::sessions::SessionService;
use crate::session::models::UserInteraction;
use crate::session::models::WorkerEvent;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Scope;
use actix_web::delete;
use actix_web::get;
use actix_web::patch;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::web::Query;

use actix_ws::Message;
use actix_ws::Session;
use futures::StreamExt;
use log::debug;
use log::warn;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::Instant;

// Constants
const PING_INTERVAL_SECS: u64 = 5;
const INACTIVE_TIMEOUT_SECS: u64 = 10;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/session/{session_id}")
            .service(get_workspace_session)
            .service(get_session_timeline)
            .service(archive_user_session)
            .service(update_user_session)
            .service(session_socket),
    );
}

#[delete("")]
pub async fn archive_user_session(
    session_service: Data<SessionService>,
    user: W,
    ids: Path<(String, String)>,
) -> Result<impl Responder> {
    let (workspace_id, session_id) = ids.into_inner();
    session_service
        .delete_user_session(user.into_inner(), workspace_id, session_id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[patch("")]
pub async fn update_user_session(
    session_service: Data<SessionService>,
    user: W,
    ids: Path<(String, String)>,
    session: Json<UpdatedSession>,
) -> Result<impl Responder> {
    let (id, session_id) = ids.into_inner();
    Ok(ApiResponse::ok(
        session_service
            .update_user_session(
                user.into_inner(),
                id,
                session_id,
                session.into_inner().title,
            )
            .await?,
    ))
}

#[get("/timeline")]
pub async fn get_session_timeline(
    session_service: Data<SessionService>,
    user: W,
    ids: Path<(String, String)>,
    page: Query<CursorPageRequest>,
) -> Result<impl Responder> {
    let (workspace_id, session_id) = ids.into_inner();
    let timeline = session_service
        .get_session_timeline_cursor(
            user.into_inner(),
            workspace_id,
            session_id,
            page.into_inner(),
        )
        .await?;
    Ok(ApiResponse::ok(timeline))
}

#[get("")]
pub async fn get_workspace_session(
    session_service: Data<SessionService>,
    user: W,
    ids: Path<(String, String)>,
) -> Result<impl Responder> {
    let (workspace_id, session_id) = ids.into_inner();
    let session: FullSession = session_service
        .get_full_user_session(user.into_inner(), workspace_id, session_id)
        .await?
        .ok_or(Error::Unauthorized(
            "You are You are not authorized to view this session.".into(),
        ))?;
    Ok(ApiResponse::ok(session))
}

#[get("/socket")]
pub async fn session_socket(
    req: HttpRequest,
    stream: web::Payload,
    session_service: Data<SessionService>,
    user: W,
    ids: Path<(String, String)>,
) -> Result<HttpResponse> {
    let (workspace_id, session_id) = ids.into_inner();
    debug!("WebSocket connection for session: {session_id}");
    let (res, session, stream) = actix_ws::handle(&req, stream)?;
    let handler = session_service
        .get_handler(user.into_inner(), workspace_id, session_id)
        .await?;

    let (sender, receiver) = mpsc::unbounded_channel();
    let (id, sender) = handler.subscribe(sender).await?;

    let session = Arc::new(Mutex::new(session));
    let stream = Rc::new(Mutex::new(stream));
    let open = Arc::new(Mutex::new(true));
    let alive = Arc::new(Mutex::new(Instant::now()));

    // Start handling outgoing events (from receiver to WebSocket)
    let session_clone = session.clone();
    let open_clone = open.clone();
    let mut receiver = receiver;
    actix_web::rt::spawn(async move {
        while let Some(event) = receiver.recv().await {
            if !*open_clone.lock().await {
                break;
            }

            match serde_json::to_string(&event) {
                Ok(serialized_event) => {
                    if session_clone
                        .lock()
                        .await
                        .text(serialized_event)
                        .await
                        .is_err()
                    {
                        *open_clone.lock().await = false;
                        break;
                    }
                }
                Err(e) => {
                    warn!("Error serializing event: {e:?}");
                }
            }
        }
        debug!("Outgoing event handler stopped");
    });

    // Start handling incoming messages (from WebSocket to sender)
    let session_clone = session.clone();
    let stream_clone = stream.clone();
    let open_clone = open.clone();
    let alive_clone = alive.clone();
    let sender_clone = sender.clone();
    actix_web::rt::spawn(async move {
        while let Some(msg) = stream_clone.lock().await.next().await {
            match msg {
                Ok(msg) => {
                    if let Err(e) = handle_message(
                        session_clone.clone(),
                        msg,
                        open_clone.clone(),
                        alive_clone.clone(),
                        sender_clone.clone(),
                    )
                    .await
                    {
                        warn!("Error handling WebSocket message: {e:?}");
                        break;
                    }
                }
                Err(e) => {
                    warn!("Error receiving WebSocket message: {e:?}");
                    break;
                }
            }

            if !*open_clone.lock().await {
                break;
            }
        }
        debug!("Incoming message handler stopped");

        let _ = sender_clone.send(WorkerEvent::Unsubscribe(id));
    });

    // Start keep-alive handler
    let session_clone = session.clone();
    let open_clone = open.clone();
    let alive_clone = alive.clone();
    actix_web::rt::spawn(async move {
        let mut interval = actix_web::rt::time::interval(Duration::from_secs(PING_INTERVAL_SECS));

        loop {
            interval.tick().await;

            if !*open_clone.lock().await {
                break;
            }

            if session_clone.lock().await.ping(b"").await.is_err() {
                *open_clone.lock().await = false;
                break;
            }

            let last_alive = *alive_clone.lock().await;
            if Instant::now().duration_since(last_alive)
                > Duration::from_secs(INACTIVE_TIMEOUT_SECS)
            {
                let session = session_clone.lock().await.clone();
                let _ = session.close(None).await;
                *open_clone.lock().await = false;
                break;
            }
        }
        debug!("Keep-alive handler stopped");
    });

    Ok(res)
}

async fn handle_message(
    session: Arc<Mutex<Session>>,
    msg: Message,
    open: Arc<Mutex<bool>>,
    alive: Arc<Mutex<Instant>>,
    sender: mpsc::UnboundedSender<WorkerEvent>,
) -> Result<()> {
    match msg {
        Message::Text(text) => match serde_json::from_str::<UserInteraction>(&text) {
            Ok(event) => {
                if sender.send(WorkerEvent::UserInteraction(event)).is_err() {
                    *open.lock().await = false;
                    return Err(Error::SocketClosed);
                }
            }
            Err(e) => {
                warn!("Error parsing SessionEvent: {e:?}");
            }
        },
        Message::Ping(bytes) => {
            if session.lock().await.pong(&bytes).await.is_err() {
                *open.lock().await = false;
                return Err(Error::SocketClosed);
            }
        }
        Message::Pong(_) => {
            *alive.lock().await = Instant::now();
        }
        Message::Close(reason) => {
            *open.lock().await = false;
            if let Some(reason) = reason {
                warn!("WebSocket connection closed: {reason:?}");
            }
        }
        _ => {
            warn!("Received unhandled message type: {msg:?}");
        }
    }
    Ok(())
}
