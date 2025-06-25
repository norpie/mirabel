use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    dto::session::event::{AcknowledgmentType, SessionEventContent},
    prelude::*,
};

use actix_web::web::Data;
use log::warn;
use surrealdb::Uuid;
use tokio::{
    sync::{
        Mutex,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    time::sleep,
};

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

impl SessionWorker {
    pub fn new(session_id: String, repository: Data<RepositoryProvider>) -> Self {
        let (event_sender, event_receiver) = unbounded_channel::<WorkerEvent>();
        Self {
            session_id,
            repository,
            receiver: Arc::new(Mutex::new(event_receiver)),
            sender: event_sender,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            state: SessionWorkerState::Stopped,
        }
    }

    pub async fn run(self: Arc<Self>) {
        let receiver = self.receiver.clone();
        actix_web::rt::spawn(async move {
            let worker = self.clone();
            while let Some(event) = receiver.lock().await.recv().await {
                let worker = worker.clone();
                actix_web::rt::spawn(async move {
                    if let Err(err) = worker.handle_event(event).await {
                        warn!(
                            "Failed to handle event in session ({}) worker: {}",
                            worker.session_id, err
                        );
                    };
                });
            }
            log::info!("Session handler stopped for session: {}", self.session_id);
        });
    }

    pub async fn subscribe(
        &self,
        sender: UnboundedSender<SessionEvent>,
    ) -> Result<(Uuid, UnboundedSender<WorkerEvent>)> {
        let id = Uuid::new_v4();
        let mut subscribers = self.subscribers.lock().await;
        if subscribers.contains_key(&id) {
            return Err(Error::DoubleSubscription);
        }
        subscribers.insert(id, sender);
        Ok((id, self.sender.clone()))
    }

    async fn handle_event(&self, event: WorkerEvent) -> Result<()> {
        match event {
            WorkerEvent::SessionEvent(event) => {
                // Start by broadcasting the event to all subscribers, so we don't get any client-side
                // out-of-sync issues.
                self.broadcast(&event).await?;
                // Start a predefined sequence for testing purposes.
                sleep(Duration::from_secs(1)).await;
                self.broadcast(&SessionEvent::acknowledgment(AcknowledgmentType::Delivered))
                    .await?;
                sleep(Duration::from_secs(2)).await;
                self.broadcast(&SessionEvent::acknowledgment(AcknowledgmentType::Seen))
                    .await?;
                sleep(Duration::from_secs(3)).await;
                self.broadcast(&SessionEvent::acknowledgment(AcknowledgmentType::Thinking))
                    .await?;
                sleep(Duration::from_secs(5)).await;
                self.broadcast(&SessionEvent::acknowledgment(AcknowledgmentType::Typing))
                    .await?;
                sleep(Duration::from_secs(5)).await;
                self.broadcast(&SessionEvent::new(SessionEventContent::MessageContent {
                    author_id: "mirabel".into(),
                    message: "This is a test reply.".into(),
                }))
                .await?;
            }
            WorkerEvent::Unsubscribe(id) => {
                let sub = self.subscribers.lock().await.remove(&id);
                if sub.is_none() {
                    warn!("Tried to unsubscribe non-existing subscriber: {}", id);
                }
            }
        };
        Ok(())
    }

    pub async fn broadcast(&self, event: &SessionEvent) -> Result<()> {
        let subscribers = self.subscribers.lock().await;
        for (id, subscriber) in subscribers.iter() {
            if let Err(e) = subscriber.send(event.clone()) {
                log::warn!("Failed to send event to subscriber: {}", e);
                self.subscribers.lock().await.remove(id);
            }
        }
        Ok(())
    }
}
