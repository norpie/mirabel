use crate::{dto::session::{AcknowledgmentType, SessionEventContent}, prelude::*, repository::RepositoryProvider};

use actix_web::web::Data;
use chrono::Utc;
use log::debug;

use crate::{dto::session::SessionEvent, model::user::User};

pub struct SessionService {
    repository: Data<RepositoryProvider>,
}

impl SessionService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self { repository })
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
                ack_type: AcknowledgmentType::Delivered
            },
        })
    }
}
