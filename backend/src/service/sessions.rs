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

pub struct SessionService {
    repository: Data<RepositoryProvider>,
    socket_registry: Data<Mutex<HashMap<String, Vec<(Uuid, Session)>>>>,
}

impl SessionService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self {
            repository,
            socket_registry: Data::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn handle_event(
        &self,
        session_id: &str,
        user: &User,
        event: SessionEvent,
    ) -> Result<SessionEvent> {
        debug!("Handling event for session {}: {:?}", session_id, event);
        Ok(SessionEvent {
            id: "lsakdjhflkasdhf".into(),
            source: "agent".into(),
            timestamp: Utc::now(),
            content: SessionEventContent::Acknowledgment {
                ack_type: AcknowledgmentType::Delivered,
            },
        })
    }

    pub async fn add_socket(&mut self, session_id: &str, session: (Uuid, Session)) -> Result<()> {
        debug!("Adding socket for session {}", session_id);
        let mut registry = self.socket_registry.lock().await;
        registry
            .entry(session_id.to_string())
            .or_default()
            .push(session);
        Ok(())
    }

    pub async fn remove_socket(&mut self, session_id: &str, socket_id: Uuid) -> Result<()> {
        debug!("Removing socket for session {}", session_id);
        let mut registry = self.socket_registry.lock().await;
        if let Some(sockets) = registry.get_mut(session_id) {
            sockets.retain(|s| s.0 != socket_id);
            if sockets.is_empty() {
                registry.remove(session_id);
            }
        }
        Ok(())
    }
}
