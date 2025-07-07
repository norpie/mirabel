use std::{collections::HashMap, sync::Arc};

use crate::{
    dto::page::{PageInfo, PageRequest, PageResponse}, model::{session::Session, workspace::WorkspaceMember}, prelude::*, session::models::SessionWorker
};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use log::debug;
use tokio::sync::Mutex;

use crate::model::user::User;

pub struct SessionService {
    repository: Data<Pool>,
    session_handler_registry: Data<Mutex<HashMap<String, Arc<SessionWorker>>>>,
}

impl SessionService {
    pub fn from(repository: Data<Pool>) -> Result<Self> {
        Ok(Self {
            repository,
            session_handler_registry: Data::new(Mutex::new(HashMap::new())),
        })
    }
    
    pub async fn create_workspace_session(
        &self,
        user: User,
        workspace_id: String,
        input: String,
    ) -> Result<Session> {
        use crate::schema::workspace_members::dsl as wm;

        // Check if user is member of workspace
        let conn = self.repository.get().await?;
        let user_id = user.id.clone();
        let workspace_id_clone = workspace_id.clone();
        let is_member = conn.interact(move |conn| {
            wm::workspace_members
                .filter(wm::user_id.eq(&user_id))
                .filter(wm::workspace_id.eq(&workspace_id_clone))
                .first::<WorkspaceMember>(conn)
                .optional()
        }).await??
        .is_some();
            
        if !is_member {
            return Err(Error::Unauthorized(
                "You are not authorized to create a session in this workspace.".to_string(),
            ));
        }
        
        let mut session = Session::new(workspace_id, user.id.clone(), input.clone());
        
        session = conn.interact(move |conn| {
            diesel::insert_into(crate::schema::sessions::table)
                .values(&session)
                .get_result::<Session>(conn)
        }).await??;
            
        // TODO: Add participant functionality
        // TODO: Add user message functionality
        
        Ok(session)
    }

    pub async fn get_user_session_by_id(
        &self,
        user: User,
        workspace_id: String,
        id: String,
    ) -> Result<Option<Session>> {
        use crate::schema::workspace_members::dsl as wm;
        use crate::schema::sessions::dsl as s;
        
        let conn = self.repository.get().await?;
        Ok(conn.interact(move |conn| {
            wm::workspace_members
                .inner_join(s::sessions.on(s::workspace_id.eq(wm::workspace_id)))
                .filter(wm::user_id.eq(&user.id))
                .filter(wm::workspace_id.eq(&workspace_id))
                .filter(s::user_id.eq(&user.id))
                .filter(s::id.eq(&id))
                .select(Session::as_select())
                .first::<Session>(conn)
                .optional()
        }).await??)
    }

    pub async fn update_user_session(
        &self,
        user: User,
        workspace_id: String,
        id: String,
        title: String,
    ) -> Result<Session> {
        use crate::schema::sessions::dsl as s;
        
        let mut session = self
            .get_user_session_by_id(user, workspace_id, id.clone())
            .await?
            .ok_or(Error::NotFound)?;
            
        session.set_title(title);
        
        let conn = self.repository.get().await?;
        let session_clone = session.clone();
        let id_clone = id.clone();
        conn.interact(move |conn| {
            diesel::update(s::sessions.filter(s::id.eq(&id_clone)))
                .set((s::title.eq(&session_clone.title), s::modified_at.eq(&session_clone.modified_at)))
                .execute(conn)
        }).await??;
            
        Ok(session)
    }

    pub async fn delete_user_session(
        &self,
        user: User,
        workspace_id: String,
        id: String,
    ) -> Result<()> {
        use crate::schema::sessions::dsl as s;
        
        let session_opt = self
            .get_user_session_by_id(user, workspace_id, id.clone())
            .await?;
        let Some(_session) = session_opt else {
            return Err(Error::NotFound);
        };
        
        let conn = self.repository.get().await?;
        let id_clone = id.clone();
        conn.interact(move |conn| {
            diesel::update(s::sessions.filter(s::id.eq(&id_clone)))
                .set(s::archived.eq(true))
                .execute(conn)
        }).await??;
            
        Ok(())
    }

    pub async fn get_user_workspace_sessions(
        &self,
        workspace_id: String,
        user: User,
        page: PageRequest,
    ) -> Result<PageResponse<Session>> {
        use crate::schema::workspace_members::dsl as wm;
        use crate::schema::sessions::dsl as s;
        
        let conn = self.repository.get().await?;
        
        // First check if user is member of workspace
        let user_id = user.id.clone();
        let workspace_id_clone = workspace_id.clone();
        let is_member = conn.interact(move |conn| {
            wm::workspace_members
                .filter(wm::user_id.eq(&user_id))
                .filter(wm::workspace_id.eq(&workspace_id_clone))
                .first::<WorkspaceMember>(conn)
                .optional()
        }).await??
        .is_some();
            
        if !is_member {
            return Err(Error::NotFound);
        }
        
        // Get sessions owned by user in the workspace
        let user_id = user.id.clone();
        let workspace_id_clone = workspace_id.clone();
        let page_clone = page.clone();
        let sessions = conn.interact(move |conn| {
            s::sessions
                .filter(s::user_id.eq(&user_id))
                .filter(s::workspace_id.eq(&workspace_id_clone))
                .filter(s::archived.eq(false))
                .offset(page_clone.offset())
                .limit(page_clone.size())
                .select(Session::as_select())
                .load::<Session>(conn)
        }).await??;
            
        let user_id = user.id.clone();
        let count = conn.interact(move |conn| {
            s::sessions
                .filter(s::user_id.eq(&user_id))
                .filter(s::workspace_id.eq(&workspace_id))
                .filter(s::archived.eq(false))
                .select(diesel::dsl::count(s::id))
                .first::<i64>(conn)
        }).await??;
            
        Ok(PageResponse::new(
            PageInfo::new(page.page(), page.size(), count),
            sessions,
        ))
    }

    pub async fn get_handler(&self, user: User, workspace_id: String, session_id: String) -> Result<Arc<SessionWorker>> {
        let opt_session = self
            .get_user_session_by_id(user, workspace_id.clone(), session_id.clone())
            .await?;
        let session = match opt_session {
            Some(session) => session,
            None => {
                return Err(Error::NotFound);
            }
        };
        let mut registry = self.session_handler_registry.lock().await;
        let opt_handler = registry.get(&session_id);
        let handler = match opt_handler {
            Some(handler) => handler.clone(),
            None => {
                let new_handler = Arc::new(SessionWorker::new(
                    session.clone(),
                    self.repository.clone(),
                ));
                registry.insert(session_id.clone(), new_handler.clone());
                let runner_clone = new_handler.clone();
                let session_id_clone = session_id.clone();
                actix_web::rt::spawn(async move {
                    debug!(
                        "Starting session handler for session: {}",
                        &session_id_clone
                    );
                    runner_clone.run().await;
                    debug!("Session handler stopped for session: {}", session_id_clone);
                });
                debug!("Created new session handler for session: {}", session_id);
                new_handler
            }
        };
        Ok(handler)
    }
}