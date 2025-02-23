use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::dto::page::PageRequest;
use crate::model::session::{NewSession, Session, UpdatedSession};
use crate::prelude::*;

use crate::repository::sessions::SessionRepository;

use super::users::SurrealDBUser;
use super::SurrealDB;

#[derive(Debug, Clone, Deserialize)]
pub struct SurrealDBSession {
    id: Thing,
    workspace_id: Thing,
    user_id: Thing,
    generated_title: String,
    user_title: Option<String>,
}

impl From<SurrealDBSession> for Session {
    fn from(session: SurrealDBSession) -> Self {
        Session::new(
            session.id.id.to_string(),
            session.workspace_id.id.to_string(),
            session.user_id.id.to_string(),
            session.generated_title,
            session.user_title,
        )
    }
}

#[async_trait]
impl SessionRepository for SurrealDB {
    /// Create a new session.
    async fn create_session(&self, session: NewSession) -> Result<Option<Session>> {
        Ok(self
            .0
            .create("session")
            .content(session)
            .await?
            .map(|su: SurrealDBSession| su.into()))
    }

    /// Retrieve a session by ID.
    async fn get_session_by_id(&self, id: String) -> Result<Option<Session>> {
        Ok(self
            .0
            .select::<Option<SurrealDBSession>>(("session", id))
            .await?
            .map(|su| su.into()))
    }

    /// Update an existing session.
    async fn update_session(&self, id: String, session: UpdatedSession) -> Result<Option<Session>> {
        Ok(self
            .0
            .update(("session", &id))
            .merge(session)
            .await?
            .map(|su: SurrealDBSession| su.into()))
    }

    /// Delete a session.
    async fn delete_session(&self, id: String) -> Result<Option<Session>> {
        Ok(self
            .0
            .delete(("session", &id))
            .await?
            .map(|su: SurrealDBSession| su.into()))
    }

    /// Retrieve user's sessions.
    async fn get_workspaces_users_sessions(
        &self,
        workspace_id: String,
        user_id: String,
        page: PageRequest,
    ) -> Result<Vec<Session>> {
        Ok(self
            .0
            .query("SELECT * FROM session WHERE user_id = $user AND workspace_id = $workspace")
            .bind(("user", Thing::from(("user", user_id.as_str()))))
            .bind((
                "workspace",
                Thing::from(("workspace", workspace_id.as_str())),
            ))
            .await?
            .take::<Vec<SurrealDBSession>>(0)?
            .into_iter()
            .map(|su| su.into())
            .collect())
    }
}
