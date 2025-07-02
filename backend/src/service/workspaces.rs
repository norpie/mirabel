use crate::{
    dto::{
        page::{PageRequest, PageResponse},
        workspace::FrontendWorkspace,
    },
    model::{session::Session, user::User, workspace::NewWorkspace},
    prelude::*,
    repository::{RepositoryProvider, traits::Entity},
};

use actix_web::web::Data;

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
    ) -> Result<FrontendWorkspace> {
        let workspace = self
            .repository
            .user_workspace_repo()
            .create_associated(&user_id, Workspace::new(workspace.name), "owns_workspace")
            .await?;
        self.repository
            .user_workspace_repo()
            .associate(&user_id, &workspace.id().unwrap(), "admins_workspace")
            .await?;
        self.repository
            .user_workspace_repo()
            .associate(&user_id, &workspace.id().unwrap(), "member_of_workspace")
            .await?;
        Ok(workspace.into())
    }

    pub async fn get_user_workspaces(
        &self,
        user_id: String,
        page: PageRequest,
    ) -> Result<PageResponse<FrontendWorkspace>> {
        Ok(self
            .repository
            .user_workspace_repo()
            .find_children(&user_id, "member_of_workspace", page)
            .await?
            .to())
    }

    pub async fn get_workspace_by_id(
        &self,
        user_id: String,
        workspace_id: String,
    ) -> Result<Option<FrontendWorkspace>> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user_id, &workspace_id, "member_of_workspace")
            .await?
        {
            return Err(Error::NotFound);
        }
        self.repository
            .workspace_repo()
            .find(&workspace_id)
            .await?
            .map(|w| Ok(w.into()))
            .transpose()
    }

    pub async fn set_workspace_avatar(
        &self,
        user_id: String,
        workspace_id: String,
        path: String,
    ) -> Result<Workspace> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user_id, &workspace_id, "admins_workspace")
            .await?
        {
            return Err(Error::Unauthorized(
                "You are not authorized to set the avatar for this workspace or it does not exist."
                    .to_string(),
            ));
        }
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
        user: User,
        workspace_id: String,
        input: String,
    ) -> Result<Session> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user.id().unwrap(), &workspace_id, "member_of_workspace")
            .await?
        {
            return Err(Error::Unauthorized(
                "You are not authorized to create a session in this workspace.".to_string(),
            ));
        }
        let mut session = self
            .repository
            .workspace_session_repo()
            .create_child(Session::new(input.clone()), &workspace_id, "owns_session")
            .await?;
        self.repository
            .user_session_repo()
            .associate(&user.id().unwrap(), &session.id().unwrap(), "owns_session")
            .await?;
        session.add_participant(user.clone());
        session.add_user_message(user.id().unwrap(), input);
        self.repository.session_repo().save(session.clone()).await?;
        Ok(session)
    }

    pub async fn get_user_session_by_id(
        &self,
        user_id: String,
        workspace_id: String,
        id: String,
    ) -> Result<Option<Session>> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user_id, &workspace_id, "member_of_workspace")
            .await?
        {
            return Err(Error::NotFound);
        }
        if !self
            .repository
            .user_session_repo()
            .is_associated(&user_id, &id, "owns_session")
            .await?
        {
            return Err(Error::NotFound);
        }
        Ok(self.repository.session_repo().find(&id).await.unwrap()) // We assume the session exists if the association is valid
    }

    pub async fn update_user_session(
        &self,
        user: User,
        workspace_id: String,
        id: String,
        title: String,
    ) -> Result<Session> {
        let mut session = self
            .get_user_session_by_id(user.id().unwrap(), workspace_id.clone(), id)
            .await?
            .unwrap();
        session.set_title(title);
        self.repository.session_repo().save(session).await
    }

    pub async fn delete_user_session(
        &self,
        user_id: String,
        workspace_id: String,
        id: String,
    ) -> Result<()> {
        let session_opt = self
            .get_user_session_by_id(user_id, workspace_id, id)
            .await?;
        let Some(mut session) = session_opt else {
            return Err(Error::NotFound);
        };
        session.archived = true;
        self.repository.session_repo().save(session).await?;
        Ok(())
    }

    pub async fn get_user_workspace_sessions(
        &self,
        workspace_id: String,
        user_id: String,
        page: PageRequest,
    ) -> Result<PageResponse<Session>> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user_id, &workspace_id, "member_of_workspace")
            .await?
        {
            return Err(Error::NotFound);
        }
        self.repository
            .user_session_repo()
            .find_children(&user_id, "owns_session", page)
            .await
    }

    pub async fn get_workspace_session_by_id(
        &self,
        user_id: String,
        workspace_id: String,
        session_id: String,
    ) -> Result<Session> {
        if !self
            .repository
            .user_workspace_repo()
            .is_associated(&user_id, &workspace_id, "member_of_workspace")
            .await?
        {
            return Err(Error::NotFound);
        }
        if !self
            .repository
            .user_session_repo()
            .is_associated(&user_id, &session_id, "owns_session")
            .await?
        {
            return Err(Error::NotFound);
        }
        Ok(self
            .repository
            .session_repo()
            .find(&session_id)
            .await?
            .unwrap()) // We assume the
        // session exists if
        // the association is
        // valid
    }
}
