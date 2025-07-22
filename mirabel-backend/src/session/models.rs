use mirabel_core::models::session::Session;
use mirabel_core::models::timeline::TimelineEntry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use crate::driver::llm::ollama::Ollama;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SessionWorkerState {
    Stopped,
    Initializing,
    Idle,
    Paused,
    Running,
    Stopping,
    Error(SessionWorkerError),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SessionWorkerError {
    Generic(String),
}

impl From<&str> for SessionWorkerError {
    fn from(err: &str) -> Self {
        SessionWorkerError::Generic(err.to_string())
    }
}

pub enum WorkerEvent {
    UserInteraction(UserInteraction),
    Unsubscribe(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum UserInteraction {
    Message { content: String },
    PromptResponse { prompt_id: String, response: String },
}

pub struct SessionWorker {
    pub session: Arc<Mutex<Session>>,
    pub pool: Data<Pool>,
    pub llm: Data<Ollama>,
    pub state: Arc<Mutex<SessionWorkerState>>,
    // Listener for events from the subscribers
    pub receiver: Arc<Mutex<UnboundedReceiver<WorkerEvent>>>,
    // Sender for events to be processed by the worker, given to the subscribers
    pub sender: UnboundedSender<WorkerEvent>,
    // All websockets at the other side
    pub subscribers: Arc<Mutex<HashMap<String, UnboundedSender<TimelineEntry>>>>,
    // Queuing and processing state
    pub is_processing: Arc<Mutex<bool>>,
    pub queue: Arc<Mutex<VecDeque<Queueable>>>,
    pub interupts: Arc<Mutex<VecDeque<Interupt>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Interupt {
    UserInteraction(UserInteraction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Queueable {
    UserInteraction(UserInteraction),
    Interupt(Interupt),
}
