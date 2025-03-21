use std::env;

use fantoccini::{Client, ClientBuilder};

use crate::prelude::*;

pub struct Browser {
    client: Client,
}

impl Browser {
    pub async fn new() -> Result<Self> {
        let browser_address = env::var("WEBDRIVER_HOST")?;
        let c = ClientBuilder::native()
            .connect(&browser_address)
            .await
            .map_err(Box::new)?;
        Ok(Self { client: c })
    }

    pub async fn close(self) -> Result<()> {
        self.client.close().await.map_err(Box::new)?;
        Ok(())
    }
}
