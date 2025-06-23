use std::{collections::HashMap, sync::Arc};

use crate::{session::SessionWorker, prelude::*, repository::RepositoryProvider};

use actix_web::web::Data;
use log::debug;
use tokio::sync::Mutex;

use crate::model::user::User;

pub struct SessionService {
    repository: Data<RepositoryProvider>,
    session_handler_registry: Data<Mutex<HashMap<String, Arc<SessionWorker>>>>,
}

impl SessionService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self {
            repository,
            session_handler_registry: Data::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn get_handler(&self, user: User, session_id: String) -> Result<Arc<SessionWorker>> {
        // TODO: is user allowed to access this session?
        let mut registry = self.session_handler_registry.lock().await;
        let opt_handler = registry.get(&session_id);
        let handler = match opt_handler {
            Some(handler) => handler.clone(),
            None => {
                let new_handler = Arc::new(SessionWorker::new(
                    session_id.clone(),
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
