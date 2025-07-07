use std::{collections::HashMap, sync::Arc};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::{dto::session::event::SessionEvent, model::session::Session};

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
    Unsubscribe(String),
}

pub struct SessionWorker {
    pub session: Arc<Mutex<Session>>,
    pub repository: Data<Pool>,
    pub state: SessionWorkerState,
    pub receiver: Arc<Mutex<UnboundedReceiver<WorkerEvent>>>,
    pub sender: UnboundedSender<WorkerEvent>,
    pub subscribers: Arc<Mutex<HashMap<String, UnboundedSender<SessionEvent>>>>,
}
