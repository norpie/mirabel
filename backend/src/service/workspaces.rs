use crate::{
    dto::page::{PageRequest, PageResponse},
    model::{session::Session, user::User, workspace::NewWorkspace},
    prelude::*,
    repository::{
        traits::{AssociatedEntityRepository, Repository},
        RepositoryProvider,
    },
};

use actix_web::web::Data;
use chrono::Utc;

use crate::model::workspace::Workspace;

pub struct WorkspaceService {
    repository: Data<RepositoryProvider>,
}

impl WorkspaceService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self { repository })
    }

    pub async fn create_user_workspace(
        &self,
        user_id: String,
        workspace: NewWorkspace,
    ) -> Result<Workspace> {
        let workspace = Workspace::new(workspace.name);
        self.repository
            .user_workspace_repo()
            .create_child(workspace, &user_id)
            .await
    }

    pub async fn get_user_workspaces(
        &self,
        user_id: String,
        page: PageRequest,
    ) -> Result<PageResponse<Workspace>> {
        self.repository
            .user_workspace_repo()
            .find_children(&user_id, page)
            .await
    }

    pub async fn set_workspace_avatar(
        &self,
        user_id: String,
        workspace_id: String,
        path: String,
    ) -> Result<Workspace> {
        let mut workspace = self
            .repository
            .workspace_repo()
            .find(&workspace_id)
            .await?
            .ok_or(Error::NotFound)?;
        workspace.set_avatar(path);
        self.repository.workspace_repo().save(workspace).await
    }

    pub async fn create_workspace_session(
        &self,
        user_id: String,
        workspace_id: String,
    ) -> Result<Session> {
        self.repository
            .workspace_session_repo()
            .create_child(
                Session::new(format!("New Session: {}", Utc::now().to_rfc2822()), None),
                &workspace_id,
            )
            .await
    }

    pub async fn get_user_session_by_id(
        &self,
        user_id: String,
        workspace_id: String,
        id: String,
    ) -> Result<Option<Session>> {
        self.repository.session_repo().find(&id).await
    }

    pub async fn update_user_session(&self, id: String, title: Option<String>) -> Result<Session> {
        let mut session = self
            .repository
            .session_repo()
            .find(&id)
            .await?
            .ok_or(Error::NotFound)?;
        session.set_user_title(title);
        self.repository.session_repo().save(session).await
    }

    pub async fn delete_user_session(
        &self,
        user_id: String,
        workspace_id: String,
        id: String,
    ) -> Result<()> {
        self.repository.session_repo().delete(&id).await
    }

    pub async fn get_user_workspace_sessions(
        &self,
        workspace_id: String,
        user_id: String,
        page: PageRequest,
    ) -> Result<PageResponse<Session>> {
        self.repository
            .workspace_session_repo()
            .find_children(&workspace_id, page)
            .await
    }

    pub async fn get_workspace_session_by_id(
        &self,
        user_id: String,
        workspace_id: String,
        session_id: String,
    ) -> Result<Option<Session>> {
        self.repository.session_repo().find(&session_id).await
    }
}
