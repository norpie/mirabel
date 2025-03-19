use std::collections::HashMap;

use crate::prelude::*;

use async_trait::async_trait;
use models::SearchPage;
use searxng::SearxNG;
use serde::{Deserialize, Serialize};
use traits::SearchEngine;

pub mod models;
pub mod traits;

mod searxng;

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
