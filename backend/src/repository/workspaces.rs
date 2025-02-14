use crate::{
    dto::page::PageRequest,
    model::{
        user::{NewUser, UpdatedUser, User},
        workspace::{NewWorkspace, UpdatedWorkspace, Workspace},
    },
    prelude::*,
};

use async_trait::async_trait;

use super::Repository;

#[async_trait]
pub trait WorkspaceRepository {
    /// Create a new workspace.
    async fn create_workspace(&self, user_id: String, workspace: NewWorkspace) -> Result<Workspace>;

    /// Retrieve a workspace by ID.
    async fn get_workspace_by_id(&self, id: String) -> Result<Option<Workspace>>;

    /// Retrieve user's workspaces.
    async fn get_users_workspaces(
        &self,
        user_id: String,
        page: PageRequest,
    ) -> Result<Vec<Workspace>>;

    /// Update an existing workspace.
    async fn update_workspace(&self, id: String, workspace: UpdatedWorkspace) -> Result<Workspace>;

    /// Delete a workspace.
    async fn delete_workspace(&self, id: String) -> Result<Workspace>;
}
