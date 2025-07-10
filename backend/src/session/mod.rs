use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    driver::id::id,
    dto::session::event::SessionEventContent,
    model::{
        session::Session,
        timeline::{AcknowledgmentType, AgentStatus, DatabaseTimelineEntry, TimelineEntry},
    },
    prelude::*,
    session::models::UserInteraction,
};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::RunQueryDsl;
use log::warn;
use models::{SessionWorker, SessionWorkerState, WorkerEvent};
use tokio::{
    sync::{
        Mutex,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    time::sleep,
};

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
        sender: UnboundedSender<TimelineEntry>,
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
            WorkerEvent::UserInteraction(event) => {
                self.handle_session_event(event).await?;
            }
            WorkerEvent::Unsubscribe(id) => {
                let sub = self.subscribers.lock().await.remove(&id);
                if sub.is_none() {
                    warn!("Tried to unsubscribe non-existing subscriber: {id}");
                }
            }
        };
        Ok(())
    }

    async fn handle_session_event(&self, event: UserInteraction) -> Result<()> {
        match event {
            UserInteraction::Message { content } => {
                self.broadcast_save(TimelineEntry::user_message(
                    self.session.lock().await.id.clone(),
                    content.clone(),
                ))
                .await?;
                self.handle_message_content(content).await?;
            }
            UserInteraction::PromptResponse {
                prompt_id,
                response,
            } => {
                self.broadcast_save(TimelineEntry::prompt_response(
                    self.session.lock().await.id.clone(),
                    prompt_id,
                    response,
                ))
                .await?
            }
        }
        Ok(())
    }

    async fn handle_message_content(&self, message: String) -> Result<()> {
        let session_id = self.session.lock().await.id.clone();
        sleep(Duration::from_secs(1)).await;
        self.broadcast_save(TimelineEntry::acknowledgment(
            session_id.clone(),
            AcknowledgmentType::Delivered,
        ))
        .await?;
        sleep(Duration::from_secs(1)).await;
        self.broadcast_save(TimelineEntry::acknowledgment(
            session_id.clone(),
            AcknowledgmentType::Seen,
        ))
        .await?;
        sleep(Duration::from_secs(1)).await;
        self.broadcast_save(TimelineEntry::status(
            session_id.clone(),
            AgentStatus::Thinking,
        ))
        .await?;
        sleep(Duration::from_secs(2)).await;
        self.broadcast_save(TimelineEntry::status(
            session_id.clone(),
            AgentStatus::Typing,
        ))
        .await?;
        sleep(Duration::from_secs(3)).await;
        self.broadcast_save(TimelineEntry::agent_message(
            self.session.lock().await.id.clone(),
            format!("Echo: {message}"),
        ))
        .await?;
        Ok(())
    }

    pub async fn broadcast_save(&self, event: TimelineEntry) -> Result<()> {
        use crate::schema::timeline_entries::dsl as te;
        self.broadcast(&event).await?;
        let db_entry: DatabaseTimelineEntry = event.try_into()?;
        self.repository
            .get()
            .await?
            .interact(|conn| {
                diesel::insert_into(te::timeline_entries)
                    .values(db_entry)
                    .execute(conn)
            })
            .await??;
        Ok(())
    }

    pub async fn broadcast(&self, event: &TimelineEntry) -> Result<()> {
        let subscribers = self.subscribers.lock().await;
        for (id, subscriber) in subscribers.iter() {
            if let Err(e) = subscriber.send(event.clone()) {
                log::warn!("Failed to send event to subscriber: {e}");
                self.subscribers.lock().await.remove(id);
            }
        }
        Ok(())
    }
}
