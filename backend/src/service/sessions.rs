use std::collections::HashMap;

use crate::{
    dto::session::event::{AcknowledgmentType, SessionEventContent},
    prelude::*,
    repository::RepositoryProvider,
};

use actix_web::web::Data;
use actix_ws::Session;
use chrono::Utc;
use log::debug;
use surrealdb::Uuid;
use tokio::sync::Mutex;

use crate::{dto::session::event::SessionEvent, model::user::User};

#[derive(Default)]
pub struct SocketHandler {
    socket_registry: HashMap<String, Vec<(Uuid, Mutex<Session>)>>,
}

impl SocketHandler {
    pub async fn add_socket(
        &mut self,
        session_id: &str,
        session: (Uuid, Mutex<Session>),
    ) -> Result<()> {
        debug!("Adding socket for session {}", session_id);
        self.socket_registry
            .entry(session_id.to_string())
            .or_default()
            .push(session);
        Ok(())
    }

    pub async fn remove_socket(&mut self, session_id: &str, socket_id: Uuid) -> Result<()> {
        debug!("Removing socket for session {}", session_id);
        if let Some(sockets) = self.socket_registry.get_mut(session_id) {
            sockets.retain(|s| s.0 != socket_id);
            if sockets.is_empty() {
                self.socket_registry.remove(session_id);
            }
        }
        Ok(())
    }

    pub async fn send_session_event(&self, session_id: &str, event: SessionEvent) -> Result<()> {
        debug!("Sending event for session {}: {:?}", session_id, event);
        if let Some(sockets) = self.socket_registry.get(session_id) {
            for (socket_id, socket) in sockets {
                if let Err(e) = socket
                    .lock()
                    .await
                    .text(serde_json::to_string(&event)?)
                    .await
                {
                    debug!("Failed to send event to socket {}: {}", socket_id, e);
                }
            }
        }
        Ok(())
    }
}

pub struct SessionService {
    repository: Data<RepositoryProvider>,
    socket_handler: Data<Mutex<SocketHandler>>,
}

impl SessionService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self {
            repository,
            socket_handler: Data::new(Mutex::new(SocketHandler::default())),
        })
    }

    pub async fn handle_event(
        &self,
        session_id: &str,
        user: &User,
        event: SessionEvent,
    ) -> Result<()> {
        debug!("Handling event for session {}: {:?}", session_id, event);

        let session_id = session_id.to_string();
        let session_id_clone = session_id.clone();
        let socket_handler_clone = self.socket_handler.clone();

        tokio::spawn(async move {
            let _ = socket_handler_clone
                .lock()
                .await
                .send_session_event(
                    &session_id_clone,
                    SessionEvent {
                        id: "lsdjhfnasld".into(),
                        source: "agent".into(),
                        timestamp: Utc::now(),
                        content: SessionEventContent::AcknowledgmentContent {
                            ack_type: AcknowledgmentType::Delivered,
                        },
                    },
                )
                .await;
        });

        let session_id_clone = session_id.clone();
        let socket_handler_clone = self.socket_handler.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let _ = socket_handler_clone
                .lock()
                .await
                .send_session_event(
                    &session_id_clone,
                    SessionEvent {
                        id: "lsakdjhflkasdhf".into(),
                        source: "agent".into(),
                        timestamp: Utc::now(),
                        content: SessionEventContent::AcknowledgmentContent {
                            ack_type: AcknowledgmentType::Seen,
                        },
                    },
                )
                .await;
        });

        let session_id_clone = session_id.clone();
        let socket_handler_clone = self.socket_handler.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let _ = socket_handler_clone
                .lock()
                .await
                .send_session_event(
                    &session_id_clone,
                    SessionEvent {
                        id: "iascuiuvaisc".into(),
                        source: "agent".into(),
                        timestamp: Utc::now(),
                        content: SessionEventContent::AcknowledgmentContent {
                            ack_type: AcknowledgmentType::Thinking,
                        },
                    },
                )
                .await;
        });

        let session_id_clone = session_id.clone();
        let socket_handler_clone = self.socket_handler.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(7)).await;
            let _ = socket_handler_clone
                .lock()
                .await
                .send_session_event(
                    &session_id_clone,
                    SessionEvent {
                        id: "ivuaroiuvsnasi".into(),
                        source: "agent".into(),
                        timestamp: Utc::now(),
                        content: SessionEventContent::AcknowledgmentContent {
                            ack_type: AcknowledgmentType::Typing,
                        },
                    },
                )
                .await;
        });

        let socket_handler_clone = self.socket_handler.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(14)).await;
            let _ = socket_handler_clone
                .lock()
                .await
                .send_session_event(
                    &session_id,
                    SessionEvent {
                        id: "nsaevtiun".into(),
                        source: "agent".into(),
                        timestamp: Utc::now(),
                        content: SessionEventContent::MessageContent {
                            author_id: "mirabel".into(),
                            message: "Hello, this is a test message.".into(),
                        },
                    },
                )
                .await;
        });

        Ok(())
    }

    pub async fn add_socket(&self, session_id: &str, socket: (Uuid, Mutex<Session>)) -> Result<()> {
        self.socket_handler
            .lock()
            .await
            .add_socket(session_id, socket)
            .await
    }

    pub async fn remove_socket(&self, session_id: &str, socket_id: Uuid) -> Result<()> {
        self.socket_handler
            .lock()
            .await
            .remove_socket(session_id, socket_id)
            .await
    }
}
