use std::time::Duration;

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
        self.client.goto(url).await?;
        Ok(self.client.source().await?)
    }
}
