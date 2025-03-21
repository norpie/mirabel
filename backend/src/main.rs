#![allow(dead_code, unused)]
use log::info;
use repository::{surrealdb::SurrealDB, Repository};

use crate::prelude::*;

pub(crate) mod error;
pub(crate) mod prelude;

pub(crate) mod driver;
pub(crate) mod dto;
pub(crate) mod handler;
pub(crate) mod messaging;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod security;
pub(crate) mod service;

#[dotenvy::load]
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let result = run().await;
    match result {
        Ok(_) => {
            info!("Finished running the application");
        }
        Err(e) => {
            log::error!("An error occurred: {}", e);
        }
    }
    Ok(())
}

async fn run() -> Result<()> {
    let db = SurrealDB::setup().await?;
    handler::run(db).await?;
    Ok(())
}
