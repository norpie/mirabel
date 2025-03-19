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

const SEARXNG_HOST_ENV: &str = "SEARXNG_HOST";

pub struct SearchEngines {
    engines: Vec<Box<dyn SearchEngine>>,
}

impl SearchEngines {
    pub fn from_env() -> Self {
        let mut engines: Vec<Box<dyn SearchEngine>> = Vec::new();
        if let Ok(host) = std::env::var(SEARXNG_HOST_ENV) {
            engines.push(Box::new(SearxNG::new(host)));
        }
        Self { engines }
    }
}

#[async_trait]
impl SearchEngine for SearchEngines {
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
