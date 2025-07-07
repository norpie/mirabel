use crate::{
    dto::{
        page::{PageInfo, PageRequest, PageResponse},
        workspace::{FrontendWorkspace, NewWorkspace},
    },
    model::{
        user::User,
        workspace::{Workspace, WorkspaceMember, WorkspaceRole},
    },
    prelude::*,
};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::{dsl::count, Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

pub struct WorkspaceService {
    repository: Data<Pool>,
}

impl WorkspaceService {
    pub fn from(repository: Data<Pool>) -> Result<Self> {
        Ok(Self { repository })
    }

    pub async fn create_user_workspace(
        &self,
        user: User,
        new_workspace: NewWorkspace,
    ) -> Result<FrontendWorkspace> {
        let workspace = Workspace::new(new_workspace.name);
        let workspace_membership =
            WorkspaceMember::new(workspace.id, user.id, WorkspaceRole::Owner);
        self.repository
            .get()
            .await?
            .lock()?
            .transaction::<(), Error, _>(|t| {
                diesel::insert_into(crate::schema::workspaces::table)
                    .values(&workspace)
                    .execute(t)?;
                diesel::insert_into(crate::schema::workspace_members::table)
                    .values(&workspace_membership)
                    .execute(t)?;
                Ok(())
            })?;
        Ok(workspace.into())
    }

    pub async fn get_user_workspaces(
        &self,
        user: User,
        page: PageRequest,
    ) -> Result<PageResponse<FrontendWorkspace>> {
        use crate::schema::workspace_members::dsl::*;
        use crate::schema::workspaces::dsl::*;
        let query = workspace_members
            .inner_join(workspaces)
            .filter(user_id.eq(user.id));
        let workspace_member_pairs = query
            .offset(page.offset())
            .limit(page.size())
            .select((WorkspaceMember::as_select(), Workspace::as_select()))
            .load::<(WorkspaceMember, Workspace)>(
                &mut self.repository.get().await?.lock()?.into(),
            )?;
        let count = query
            .select(count(workspace_id))
            .first::<i64>(&mut self.repository.get().await?.lock()?.into())?;
        Ok(PageResponse::new(
            PageInfo::new(page.page(), page.size(), count),
            workspace_member_pairs
                .into_iter()
                .map(|p| p.1.into())
                .collect(),
        ))
    }

    pub async fn get_workspace_by_id(
        &self,
        user_id: String,
        workspace_id: String,
    ) -> Result<Option<FrontendWorkspace>> {
        use crate::schema::workspace_members::dsl::*;
        use crate::schema::workspaces::dsl::*;
        
        let result = workspace_members
            .inner_join(workspaces)
            .filter(user_id.eq(&user_id))
            .filter(workspace_id.eq(&workspace_id))
            .select(Workspace::as_select())
            .first::<Workspace>(&mut self.repository.get().await?.lock()?.into())
            .optional()?;
            
        Ok(result.map(|w| w.into()))
    }
}
