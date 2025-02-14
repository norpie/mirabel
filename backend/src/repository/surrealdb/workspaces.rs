use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::dto::page::PageRequest;
use crate::model::workspace::{NewWorkspace, UpdatedWorkspace, Workspace};
use crate::prelude::*;

use crate::repository::workspaces::WorkspaceRepository;

use super::users::SurrealDBUser;
use super::SurrealDB;

#[derive(Debug, Clone, Deserialize)]
pub struct SurrealDBWorkspace {
    id: Thing,
    name: String,
}

impl From<SurrealDBWorkspace> for Workspace {
    fn from(workspace: SurrealDBWorkspace) -> Self {
        Workspace::new(workspace.id.id.to_string(), workspace.name)
    }
}

#[async_trait]
impl WorkspaceRepository for SurrealDB {
    /// Create a new workspace.
    async fn create_workspace(
        &self,
        user_id: String,
        workspace: NewWorkspace,
    ) -> Result<Workspace> {
        let workspace: SurrealDBWorkspace = self
            .0
            .create("workspace")
            .content(workspace)
            .await?
            .ok_or(Error::NotFound("new user".into()))?;

        self.0
            .query("RELATE ONLY $user -> owns -> $workspace")
            .bind(("user", Thing::from(("user", user_id.as_str()))))
            .bind(("workspace", workspace.id.clone()))
            .await?;

        Ok(workspace.into())
    }

    /// Retrieve a workspace by ID.
    async fn get_workspace_by_id(&self, id: String) -> Result<Option<Workspace>> {
        Ok(self
            .0
            .select::<Option<SurrealDBWorkspace>>(("workspace", id))
            .await?
            .map(|su| su.into()))
    }

    /// Retrieve user's workspaces.
    async fn get_users_workspaces(
        &self,
        user_id: String,
        page: PageRequest,
    ) -> Result<Vec<Workspace>> {
        Ok(self.0.query("SELECT * FROM (SELECT ->owns->workspace as workspaces FROM $user FETCH workspaces)[0].workspaces")
            .bind(("user", Thing::from(("user", user_id.as_str()))))
            .await?
            .take::<Vec<SurrealDBWorkspace>>(0)?
            .into_iter()
            .map(|su| su.into())
            .collect())
    }

    /// Update an existing workspace.
    async fn update_workspace(&self, id: String, workspace: UpdatedWorkspace) -> Result<Workspace> {
        self.0
            .update(("workspace", &id))
            .merge(workspace)
            .await?
            .ok_or(Error::NotFound(format!("Updated workspace: {}", &id)))
            .map(|su: SurrealDBWorkspace| su.into())
    }

    /// Delete a workspace.
    async fn delete_workspace(&self, id: String) -> Result<Workspace> {
        self.0
            .delete(("workspace", &id))
            .await?
            .ok_or(Error::NotFound(format!("Deleted workspace: {}", &id)))
            .map(|su: SurrealDBWorkspace| su.into())
    }
}
