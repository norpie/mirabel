use serde::Deserialize;
use serde::Serialize;

use crate::{
    dto::page::PageResponse,
    model::{session::Session, timeline::TimelineEntry},
};


pub mod event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSession {
    pub id: String,
    pub title: String,
    pub archived: bool,
    pub timeline: PageResponse<TimelineEntry>,
    pub spec: Option<String>,
    pub shell: Option<Vec<String>>,
}

impl FullSession {
    pub fn new(
        session: Session,
        timeline: PageResponse<TimelineEntry>,
        spec: Option<String>,
        shell: Option<Vec<String>>,
    ) -> FullSession {
        FullSession {
            id: session.id,
            title: session.title,
            archived: session.archived,
            timeline,
            spec,
            shell,
        }
    }
}
