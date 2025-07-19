use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use crate::{
    driver::{id::id, llm::ollama::Ollama},
    model::{
        session::Session,
        timeline::{AcknowledgmentType, AgentStatus, TimelineEntry},
    },
    prelude::*,
    session::models::{Interupt, Queueable, UserInteraction},
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
    pub fn new(session: Session, pool: Data<Pool>, llm: Data<Ollama>) -> Self {
        let (event_sender, event_receiver) = unbounded_channel::<WorkerEvent>();
        Self {
            session: Arc::new(Mutex::new(session)),
            pool,
            llm,
            receiver: Arc::new(Mutex::new(event_receiver)),
            sender: event_sender,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            state: Arc::new(Mutex::new(SessionWorkerState::Stopped)),
            is_processing: Arc::new(Mutex::new(false)),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            interupts: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn run(self: Arc<Self>) {
        let receiver = self.receiver.clone();
        let event_worker = self.clone();
        let queue_worker = self.clone();
        actix_web::rt::spawn(async move {
            while let Some(event) = receiver.lock().await.recv().await {
                let event_worker = event_worker.clone();
                actix_web::rt::spawn(async move {
                    if let Err(err) = event_worker.handle_event(event).await {
                        warn!(
                            "Failed to handle event in session ({}) worker: {}",
                            event_worker.session.lock().await.id,
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
        // Queue processing loop (new)
        actix_web::rt::spawn(async move {
            queue_worker.process_work_queue().await;
        });
    }

    async fn process_work_queue(self: Arc<Self>) {
        loop {
            // Check for interrupts first
            if let Err(e) = self.clone().handle_pending_interrupts().await {
                *self.state.lock().await = SessionWorkerState::Error(e.to_string().as_str().into());
                break;
            }

            // Process next work item
            if let Some(work_item) = self.clone().queue.lock().await.pop_front() {
                if let Err(err) = self.clone().execute_work_item(work_item).await {
                    *self.state.lock().await =
                        SessionWorkerState::Error(err.to_string().as_str().into());
                }
            } else {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    async fn execute_work_item(self: Arc<Self>, work_item: Queueable) -> Result<()> {
        match work_item {
            Queueable::Interupt(interrupt) => {
                // self.handle_interrupt(interrupt).await?;
            }
            Queueable::UserInteraction(interaction) => {
                // self.handle_event(event).await?;
            }
        }
        Ok(())
    }

    async fn handle_pending_interrupts(self: Arc<Self>) -> Result<()> {
        let mut interrupts = self.interupts.lock().await;
        while let Some(interrupt) = interrupts.pop_front() {
            if self.clone().is_immediate_interrupt(&interrupt).await? {
                self.queue
                    .lock()
                    .await
                    .push_front(Queueable::Interupt(interrupt));
            } else {
                self.queue
                    .lock()
                    .await
                    .push_back(Queueable::Interupt(interrupt));
            }
        }
        Ok(())
    }

    async fn is_immediate_interrupt(self: Arc<Self>, interrupt: &Interupt) -> Result<bool> {
        // TODO: implement logic to check if it is immediate
        Ok(true)
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
                self.handle_user_interaction(event).await?;
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

    async fn handle_user_interaction(&self, interaction: UserInteraction) -> Result<()> {
        let mut queue = self.queue.lock().await;
        if queue.is_empty() {
            queue.push_back(Queueable::UserInteraction(interaction.clone()));
        } else {
            self.interupts
                .lock()
                .await
                .push_back(Interupt::UserInteraction(interaction.clone()));
        }
        match interaction {
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
        self.pool
            .get()
            .await?
            .interact(|conn| {
                diesel::insert_into(te::timeline_entries)
                    .values(event)
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
