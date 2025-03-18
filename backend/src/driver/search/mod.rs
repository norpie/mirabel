use crate::prelude::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod searxng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPage {
    pub page: i32,
    pub query: String,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub summary: String,
    pub source: String,
}

#[async_trait]
pub trait SearchEngine {
    async fn search(&self, query: String, page: i32) -> Result<SearchPage>;
}
