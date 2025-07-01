use std::{collections::HashMap, sync::Arc};

use actix_web::web::Data;
use tokio::sync::{mpsc::{UnboundedReceiver, UnboundedSender}, Mutex};
use uuid::Uuid;

use crate::{dto::session::event::SessionEvent, repository::RepositoryProvider};

pub enum SessionWorkerState {
    Stopped,
    Initializing,
    Idle,
    Paused,
    Running,
    Stopping,
    Error,
}

pub enum WorkerEvent {
    SessionEvent(SessionEvent),
    Unsubscribe(Uuid),
}

pub struct SessionWorker {
    pub session_id: String,
    pub repository: Data<RepositoryProvider>,
    pub state: SessionWorkerState,
    pub receiver: Arc<Mutex<UnboundedReceiver<WorkerEvent>>>,
    pub sender: UnboundedSender<WorkerEvent>,
    pub subscribers: Arc<Mutex<HashMap<Uuid, UnboundedSender<SessionEvent>>>>,
}

