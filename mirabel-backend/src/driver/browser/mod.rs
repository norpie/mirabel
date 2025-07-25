use std::env;

use deadpool::unmanaged::Object;
use deadpool::unmanaged::Pool;
use fantoccini::Client;
use fantoccini::ClientBuilder;
use log::debug;
use log::error;

use crate::prelude::*;

pub struct Browsers {
    size: usize,
    pool: Pool<Client>,
}

impl Browsers {
    pub async fn new() -> Result<Self> {
        let mut clients = Vec::new();
        for var in env::vars().filter(|(k, _)| k.starts_with("WEBDRIVER_HOST")) {
            let res = ClientBuilder::native()
                .connect(&var.1)
                .await
                .map_err(Box::new);
            match res {
                Ok(client) => {
                    debug!("Connected to browser found in ({}:{})", var.0, var.1);
                    clients.push(client)
                }
                Err(e) => {
                    error!(
                        "Failed to connect to browser found in ({}:{}): {:?}",
                        var.0, var.1, e
                    );
                }
            }
        }
        let size = clients.len();
        let pool = Pool::new(size);
        for client in clients {
            pool.add(client).await.map_err(|(_, e)| e)?;
        }
        Ok(Self { size, pool })
    }

    pub async fn acquire(&self) -> Result<Object<Client>> {
        Ok(self.pool.get().await?)
    }

    pub async fn close(self) -> Result<()> {
        debug!("Closing {} browsers", self.size);
        for i in 0..self.size {
            let client = self.pool.remove().await?;
            client.close().await?;
            debug!("Closed browser {}/{}", i + 1, self.size);
        }
        debug!("Closed {} browsers", self.size);
        Ok(())
    }
}
