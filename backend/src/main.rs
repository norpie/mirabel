#![allow(dead_code, unused)]
use driver::browser::Browser;
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
            info!("Mirabel has finished running successfully");
        }
        Err(e) => {
            log::error!("An error occurred: {}", e);
        }
    }
    Ok(())
}

async fn run() -> Result<()> {
    info!("Running setup tasks");
    let db = SurrealDB::setup().await?;
    let browser = Browser::new().await?;
    info!("Running lifecycle tasks");
    handler::run(db).await?;
    info!("Running cleanup tasks");
    browser.close().await?;
    Ok(())
}
