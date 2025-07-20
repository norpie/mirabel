use std::{collections::HashMap, sync::Arc};

use crate::{
    driver::llm::ollama::Ollama,
    dto::{
        page::{PageInfo, PageRequest, PageResponse, CursorPageRequest, CursorPageResponse},
        session::FullSession,
    },
    model::{
        session::Session,
        timeline::{TimelineEntry, TimelineEntryContent},
        workspace::WorkspaceMember,
    },
    prelude::*,
    session::models::SessionWorker,
};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use log::debug;
use tokio::sync::Mutex;

use crate::model::user::User;

pub struct SessionService {
    repository: Data<Pool>,
    llm: Data<Ollama>,
    session_handler_registry: Data<Mutex<HashMap<String, Arc<SessionWorker>>>>,
}

impl SessionService {
    pub fn from(repository: Data<Pool>, llm: Data<Ollama>) -> Result<Self> {
        Ok(Self {
            repository,
            llm,
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
        let is_member = conn
            .interact(move |conn| {
                wm::workspace_members
                    .filter(wm::user_id.eq(&user_id))
                    .filter(wm::workspace_id.eq(&workspace_id_clone))
                    .first::<WorkspaceMember>(conn)
                    .optional()
            })
            .await??
            .is_some();

        if !is_member {
            return Err(Error::Unauthorized(
                "You are not authorized to create a session in this workspace.".to_string(),
            ));
        }

        let mut session = Session::new(workspace_id, user.id.clone(), input.clone());

        session = conn
            .interact(move |conn| {
                diesel::insert_into(crate::schema::sessions::table)
                    .values(&session)
                    .get_result::<Session>(conn)
            })
            .await??;

        let entry = TimelineEntry::user_message(session.id.clone(), input);
        conn.interact(move |conn| {
            diesel::insert_into(crate::schema::timeline_entries::table)
                .values(entry)
                .execute(conn)
        })
        .await??;

        Ok(session)
    }

    pub async fn get_user_session_by_id(
        &self,
        user: User,
        workspace_id: String,
        id: String,
    ) -> Result<Option<Session>> {
        use crate::schema::sessions::dsl as s;
        use crate::schema::workspace_members::dsl as wm;

        let conn = self.repository.get().await?;
        Ok(conn
            .interact(move |conn| {
                wm::workspace_members
                    .inner_join(s::sessions.on(s::workspace_id.eq(wm::workspace_id)))
                    .filter(wm::user_id.eq(&user.id))
                    .filter(wm::workspace_id.eq(&workspace_id))
                    .filter(s::user_id.eq(&user.id))
                    .filter(s::id.eq(&id))
                    .select(Session::as_select())
                    .first::<Session>(conn)
                    .optional()
            })
            .await??)
    }

    pub async fn get_full_user_session(
        &self,
        user: User,
        workspace_id: String,
        id: String,
    ) -> Result<Option<FullSession>> {
        let session_opt = self
            .get_user_session_by_id(user.clone(), workspace_id.clone(), id.clone())
            .await?;

        let Some(session) = session_opt else {
            return Ok(None);
        };

        let cursor_response = self
            .get_session_timeline_cursor(
                user,
                workspace_id.clone(),
                id.clone(),
                CursorPageRequest::limit(50), // Explicitly request 50 messages for initial load
            )
            .await?;
        
        // Convert cursor response to page response for backward compatibility
        let page_response = PageResponse::new(
            PageInfo::new(
                std::num::NonZeroI64::new(1).unwrap(),
                cursor_response.data.len() as i64,
                cursor_response.data.len() as i64, // We don't have total count in cursor pagination
            ),
            cursor_response.data,
        );
        let spec = self.get_latest_spec(id.clone()).await?;
        let shell = self.get_shell_state(id.clone()).await?;

        Ok(Some(FullSession::new(session, page_response, spec, shell)))
    }

    pub async fn get_session_timeline_cursor(
        &self,
        user: User,
        workspace_id: String,
        id: String,
        cursor_page: CursorPageRequest,
    ) -> Result<CursorPageResponse<TimelineEntry>> {
        use crate::schema::timeline_entries::dsl as te;

        let session_opt = self
            .get_user_session_by_id(user, workspace_id.clone(), id.clone())
            .await?;

        if session_opt.is_none() {
            return Err(Error::NotFound);
        }

        let conn = self.repository.get().await?;
        let id_clone = id.clone();
        let cursor_page_clone = cursor_page.clone();
        let limit = cursor_page_clone.limit;

        // Build the query based on cursor pagination
        let mut entries = conn
            .interact(move |conn| {
                let mut query = te::timeline_entries
                    .filter(te::session_id.eq(id_clone))
                    .into_boxed();

                // Add cursor-based filtering
                if let Some(before) = cursor_page_clone.before {
                    query = query.filter(te::created_at.lt(before));
                } else if let Some(after) = cursor_page_clone.after {
                    query = query.filter(te::created_at.gt(after));
                }

                // For "before" cursor (loading older), order desc to get older entries
                // For "after" cursor (loading newer), order asc to get newer entries  
                // For no cursor (initial load), order desc to get latest entries first
                if cursor_page_clone.after.is_some() {
                    // Loading newer messages - get entries after cursor in asc order
                    query
                        .order(te::created_at.asc())
                        .limit(limit + 1)
                        .load::<TimelineEntry>(conn)
                } else {
                    // Loading older messages OR initial load - get entries in desc order
                    query
                        .order(te::created_at.desc())
                        .limit(limit + 1)
                        .load::<TimelineEntry>(conn)
                }
            })
            .await??;

        // Check if there are more entries
        let has_more = entries.len() > limit as usize;
        if has_more {
            entries.pop(); // Remove the extra entry
        }

        // Determine cursors based on the data we got
        let next_cursor = if has_more && !entries.is_empty() {
            if cursor_page.before.is_some() {
                // When loading older entries, the "next" cursor is the oldest entry we just loaded
                Some(entries.last().unwrap().created_at)
            } else {
                // When loading newer entries, the "next" cursor is the newest entry we just loaded
                Some(entries.last().unwrap().created_at)
            }
        } else {
            None
        };

        let prev_cursor = if !entries.is_empty() {
            Some(entries.first().unwrap().created_at)
        } else {
            None
        };

        // For loading older entries OR initial load, we need to reverse to get chronological order
        // since we queried in desc order
        if cursor_page.after.is_none() {
            entries.reverse();
        }

        Ok(CursorPageResponse::new(
            entries,
            has_more,
            next_cursor,
            prev_cursor,
        ))
    }

    pub async fn get_session_timeline(
        &self,
        user: User,
        workspace_id: String,
        id: String,
        page: PageRequest,
    ) -> Result<PageResponse<TimelineEntry>> {
        use crate::schema::timeline_entries::dsl as te;

        let session_opt = self
            .get_user_session_by_id(user, workspace_id.clone(), id.clone())
            .await?;

        if session_opt.is_none() {
            return Err(Error::NotFound);
        }

        let conn = self.repository.get().await?;
        let id_clone = id.clone();
        let page_clone = page.clone();
        let offset = page_clone.offset();
        let limit = page_clone.size();

        // Get latest entries first (newest to oldest), then reverse for chronological order
        let mut entries = conn
            .interact(move |conn| {
                te::timeline_entries
                    .filter(te::session_id.eq(id_clone))
                    .order(te::created_at.desc())
                    .offset(offset)
                    .limit(limit)
                    .load::<TimelineEntry>(conn)
            })
            .await??;

        // Reverse to get chronological order (oldest to newest) for display
        entries.reverse();

        let id_clone = id.clone();
        let total_count = conn
            .interact(|conn| {
                te::timeline_entries
                    .filter(te::session_id.eq(id_clone))
                    .count()
                    .get_result::<i64>(conn)
            })
            .await??;

        Ok(PageResponse::new(
            PageInfo::new(page.page(), page.size(), total_count),
            entries,
        ))
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
                .set((
                    s::title.eq(&session_clone.title),
                    s::modified_at.eq(&session_clone.modified_at),
                ))
                .execute(conn)
        })
        .await??;

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
        })
        .await??;

        Ok(())
    }

    pub async fn get_user_workspace_sessions(
        &self,
        workspace_id: String,
        user: User,
        page: PageRequest,
    ) -> Result<PageResponse<Session>> {
        use crate::schema::sessions::dsl as s;
        use crate::schema::workspace_members::dsl as wm;

        let conn = self.repository.get().await?;

        // First check if user is member of workspace
        let user_id = user.id.clone();
        let workspace_id_clone = workspace_id.clone();
        let is_member = conn
            .interact(move |conn| {
                wm::workspace_members
                    .filter(wm::user_id.eq(&user_id))
                    .filter(wm::workspace_id.eq(&workspace_id_clone))
                    .first::<WorkspaceMember>(conn)
                    .optional()
            })
            .await??
            .is_some();

        if !is_member {
            return Err(Error::NotFound);
        }

        // Get sessions owned by user in the workspace
        let user_id = user.id.clone();
        let workspace_id_clone = workspace_id.clone();
        let page_clone = page.clone();
        let sessions = conn
            .interact(move |conn| {
                s::sessions
                    .filter(s::user_id.eq(&user_id))
                    .filter(s::workspace_id.eq(&workspace_id_clone))
                    .filter(s::archived.eq(false))
                    .offset(page_clone.offset())
                    .limit(page_clone.size())
                    .select(Session::as_select())
                    .load::<Session>(conn)
            })
            .await??;

        let user_id = user.id.clone();
        let count = conn
            .interact(move |conn| {
                s::sessions
                    .filter(s::user_id.eq(&user_id))
                    .filter(s::workspace_id.eq(&workspace_id))
                    .filter(s::archived.eq(false))
                    .select(diesel::dsl::count(s::id))
                    .first::<i64>(conn)
            })
            .await??;

        Ok(PageResponse::new(
            PageInfo::new(page.page(), page.size(), count),
            sessions,
        ))
    }

    pub async fn get_timeline_entry(
        &self,
        content_type: String,
        session_id: String,
    ) -> Result<Option<TimelineEntry>> {
        let conn = self.repository.get().await?;
        let entry_opt = conn
            .interact(move |conn| {
                use crate::schema::timeline_entries::dsl as te;
                te::timeline_entries
                    .filter(te::session_id.eq(&session_id))
                    .filter(te::content_type.eq(&content_type))
                    .order(te::created_at.desc())
                    .first::<TimelineEntry>(conn)
                    .optional()
            })
            .await??;
        Ok(entry_opt)
    }

    pub async fn get_latest_spec(&self, session_id: String) -> Result<Option<String>> {
        let entry = self
            .get_timeline_entry("spec".to_string(), session_id.clone())
            .await?;
        let Some(entry) = entry else {
            return Ok(None);
        };
        match entry.content {
            TimelineEntryContent::Spec { content } => {
                debug!("Found spec content for session: {session_id}");
                Ok(Some(content))
            }
            _ => {
                panic!("Expected spec content type, got: {:?}", entry.content);
            }
        }
    }

    pub async fn get_latest_plan(&self, session_id: String) -> Result<Option<String>> {
        let entry = self
            .get_timeline_entry("plan".to_string(), session_id.clone())
            .await?;
        let Some(entry) = entry else {
            return Ok(None);
        };
        match entry.content {
            TimelineEntryContent::Plan { .. } => {
                debug!("Found plan content for session: {session_id}");
                Ok(Some("PLACEHOLDER".into())) // TODO: Implement real plan content retrieval
            }
            _ => {
                panic!("Expected plan content type, got: {:?}", entry.content);
            }
        }
    }
    pub async fn get_shell_state(&self, session_id: String) -> Result<Option<Vec<String>>> {
        let entry = self
            .get_timeline_entry("shell".to_string(), session_id.clone())
            .await?;
        let Some(entry) = entry else {
            return Ok(None);
        };
        match entry.content {
            TimelineEntryContent::Shell { lines } => {
                debug!("Found shell state for session: {session_id}");
                Ok(Some(lines))
            }
            _ => {
                panic!("Expected shell content type, got: {:?}", entry.content);
            }
        }
    }

    pub async fn get_handler(
        &self,
        user: User,
        workspace_id: String,
        session_id: String,
    ) -> Result<Arc<SessionWorker>> {
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
                    self.llm.clone(),
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
                    debug!("Session handler stopped for session: {session_id_clone}");
                });
                debug!("Created new session handler for session: {session_id}");
                new_handler
            }
        };
        Ok(handler)
    }
}
