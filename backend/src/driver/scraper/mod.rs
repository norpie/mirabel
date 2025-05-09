use log::{debug, warn};
use std::time::Duration;
use tokio::time;

use crate::prelude::*;

use fantoccini::Client;

pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn scrape(&self, url: &str, timeout: Duration) -> Result<String> {
        debug!("Scraping URL: {}", url);
        match time::timeout(timeout, self.client.goto(url)).await {
            Ok(result) => {
                result?;
            }
            Err(_) => {
                warn!(
                    "Navigation timeout after {:?} - proceeding with partial page",
                    timeout
                );
            }
        }
        Ok(self.client.source().await?)
    }
}
