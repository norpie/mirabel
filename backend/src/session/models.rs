use std::{collections::HashMap, sync::Arc};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::model::{session::Session, timeline::TimelineEntry};

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
    UserInteraction(UserInteraction),
    Unsubscribe(String),
}

pub struct SessionWorker {
    pub session: Arc<Mutex<Session>>,
    pub repository: Data<Pool>,
    pub state: SessionWorkerState,
    pub receiver: Arc<Mutex<UnboundedReceiver<WorkerEvent>>>,
    pub sender: UnboundedSender<WorkerEvent>,
    pub subscribers: Arc<Mutex<HashMap<String, UnboundedSender<TimelineEntry>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum UserInteraction {
    Message { content: String },
    PromptResponse { prompt_id: String, response: String },
}
