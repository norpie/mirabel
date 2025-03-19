use std::collections::HashMap;

use crate::prelude::*;

use async_trait::async_trait;
use searxng::SearxNG;
use serde::{Deserialize, Serialize};

pub mod searxng;

pub struct Engines {
    engines: Vec<Box<dyn SearchEngine>>,
}

#[async_trait]
impl SearchEngine for Engines {
    async fn search(&self, query: String, page: i32) -> Result<SearchPage> {
        // Priority is given to the first available engine
        for engine in &self.engines {
            if engine.available().await {
                return engine.search(query, page).await;
            }
        }
        Err(Error::NoAvailableEngine)
    }

    async fn available(&self) -> bool {
        for engine in &self.engines {
            if engine.available().await {
                return true;
            }
        }
        false
    }
}

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
pub trait SearchEngine: Send + Sync {
    async fn available(&self) -> bool;
    async fn search(&self, query: String, page: i32) -> Result<SearchPage>;
}
