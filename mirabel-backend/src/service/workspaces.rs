use crate::prelude::*;
use mirabel_core::dto::page::PageInfo;
use mirabel_core::dto::page::PageRequest;
use mirabel_core::dto::page::PageResponse;
use mirabel_core::dto::workspace::FrontendWorkspace;
use mirabel_core::dto::workspace::NewWorkspace;
use mirabel_core::models::user::User;
use mirabel_core::models::workspace::Workspace;
use mirabel_core::models::workspace::WorkspaceMember;
use mirabel_core::models::workspace::WorkspaceRole;

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::dsl::count;

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
            WorkspaceMember::new(workspace.id.clone(), user.id, WorkspaceRole::Owner);
        let conn = self.repository.get().await?;
        let workspace_clone = workspace.clone();
        conn.interact(move |conn| {
            conn.transaction::<(), Error, _>(|t| {
                diesel::insert_into(mirabel_core::schema::workspaces::table)
                    .values(&workspace_clone)
                    .execute(t)?;
                diesel::insert_into(mirabel_core::schema::workspace_members::table)
                    .values(&workspace_membership)
                    .execute(t)?;
                Ok(())
            })
        })
        .await??;
        Ok(workspace.into())
    }

    pub async fn get_user_workspaces(
        &self,
        user: User,
        page: PageRequest,
    ) -> Result<PageResponse<FrontendWorkspace>> {
        use mirabel_core::schema::workspace_members::dsl as wm;
        use mirabel_core::schema::workspaces::dsl as w;

        let conn = self.repository.get().await?;
        let user_id_clone = user.id.clone();
        let page_clone = page.clone();
        let workspace_member_pairs = conn
            .interact(move |conn| {
                wm::workspace_members
                    .inner_join(w::workspaces)
                    .filter(wm::user_id.eq(user_id_clone))
                    .offset(page_clone.offset())
                    .limit(page_clone.size())
                    .select((WorkspaceMember::as_select(), Workspace::as_select()))
                    .load::<(WorkspaceMember, Workspace)>(conn)
            })
            .await??;

        let user_id_clone = user.id.clone();
        let count = conn
            .interact(move |conn| {
                wm::workspace_members
                    .inner_join(w::workspaces)
                    .filter(wm::user_id.eq(user_id_clone))
                    .select(count(w::id))
                    .first::<i64>(conn)
            })
            .await??;

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
        use mirabel_core::schema::workspace_members::dsl as wm;
        use mirabel_core::schema::workspaces::dsl as w;

        let conn = self.repository.get().await?;
        let result = conn
            .interact(move |conn| {
                wm::workspace_members
                    .inner_join(w::workspaces)
                    .filter(wm::user_id.eq(&user_id))
                    .filter(w::id.eq(&workspace_id))
                    .select(Workspace::as_select())
                    .first::<Workspace>(conn)
                    .optional()
            })
            .await??;

        Ok(result.map(|w| w.into()))
    }
}
