use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    driver::id::id, dto::session::event::{AcknowledgmentType, SessionEventContent}, model::{chat::ChatMessage, session::Session}, prelude::*
};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use log::warn;
use models::{SessionWorker, SessionWorkerState, WorkerEvent};
use tokio::{
    sync::{
        Mutex,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    time::sleep,
};

use crate::dto::session::event::SessionEvent;

pub mod models;

impl SessionWorker {
    pub fn new(session: Session, repository: Data<Pool>) -> Self {
        let (event_sender, event_receiver) = unbounded_channel::<WorkerEvent>();
        Self {
            session: Arc::new(Mutex::new(session)),
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
                            worker.session.lock().await.id,
                            err
                        );
                    };
                });
            }
            log::info!(
                "Session handler stopped for session: {}",
                self.session.lock().await.id
            );
        });
    }

    pub async fn subscribe(
        &self,
        sender: UnboundedSender<SessionEvent>,
    ) -> Result<(String, UnboundedSender<WorkerEvent>)> {
        let id = id!();
        let mut subscribers = self.subscribers.lock().await;
        if subscribers.contains_key(&id) {
            return Err(Error::DoubleSubscription);
        }
        subscribers.insert(id.clone(), sender);
        Ok((id, self.sender.clone()))
    }

    async fn handle_event(&self, event: WorkerEvent) -> Result<()> {
        match event {
            WorkerEvent::SessionEvent(event) => {
                // Start by broadcasting the event to all subscribers, so we don't get any client-side
                // out-of-sync issues.
                self.handle_session_event(event).await?;
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

    async fn handle_session_event(&self, event: SessionEvent) -> Result<()> {
        self.broadcast(&event).await?;
        match event.content {
            SessionEventContent::AcknowledgmentContent { ack_type } => todo!(),
            SessionEventContent::MessageContent { author_id, message } => {
                self.handle_message_content(author_id, message).await?;
            }
            SessionEventContent::AgentActionContent {
                action,
                description,
            } => todo!(),
            SessionEventContent::AgentPromptContent {
                prompt_id,
                prompt,
                options,
                allow_other,
            } => todo!(),
            SessionEventContent::UserPromptResponseContent {
                prompt_id,
                response,
            } => todo!(),
            SessionEventContent::AgentNewTaskEvent {
                task_id,
                parent_id,
                description,
            } => todo!(),
            SessionEventContent::AgentTaskEvent { task_id, status } => todo!(),
            SessionEventContent::AgentSpecUpdateEvent { spec } => todo!(),
            SessionEventContent::AgentTerminalContentEvent { content } => todo!(),
        }
        Ok(())
    }

    async fn handle_message_content(&self, author_id: String, message: String) -> Result<()> {
        let mut session = self.session.lock().await;
        // session
        //     .chat
        //     .add_message(ChatMessage::new(author_id, message));
        // self.repository.session_repo().save(session.clone()).await?;
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
        let agent_id = "mirabel".to_string();
        let agent_message = "This is a test message from the agent.".to_string();
        // session
        //     .chat
        //     .add_message(ChatMessage::new(agent_id.clone(), agent_message.clone()));
        // self.repository.session_repo().save(session.clone()).await?;
        self.broadcast(&SessionEvent::new(SessionEventContent::MessageContent {
            author_id: agent_id,
            message: agent_message,
        }))
        .await?;
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
