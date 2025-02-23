use crate::{
    dto::page::PageRequest,
    model::{
        session::{NewSession, Session, UpdatedSession},
        user::{NewUser, UpdatedUser, User},
    },
    prelude::*,
};

use async_trait::async_trait;

use super::Repository;

#[async_trait]
pub trait SessionRepository {
    /// Create a new session.
    async fn create_session(
        &self,
        workspace_id: String,
        user_id: String,
        new: NewSession,
    ) -> Result<Option<Session>>;

    /// Retrieve a session by ID.
    async fn get_session_by_id(&self, id: String) -> Result<Option<Session>>;

    /// Update an existing session.
    async fn update_session(&self, id: String, session: UpdatedSession) -> Result<Option<Session>>;

    /// Delete a session.
    async fn delete_session(&self, id: String) -> Result<Option<Session>>;

    /// Retrieve user's sessions.
    async fn get_workspaces_users_sessions(
        &self,
        workspace_id: String,
        user_id: String,
        page: PageRequest,
    ) -> Result<Vec<Session>>;
}
