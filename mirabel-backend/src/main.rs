#![allow(dead_code)]

use actix_web::web::Data;
use driver::browser::Browsers;
use driver::search::SearchEngines;
use driver::search::traits::SearchEngine;

use log::info;
use log::warn;

extern crate backend_derive;

use crate::driver::llm::ollama::Ollama;
use crate::prelude::*;

pub(crate) mod error;
pub(crate) mod prelude;

pub(crate) mod agent;
pub(crate) mod driver;
pub(crate) mod handler;
pub(crate) mod messaging;
pub(crate) mod session;
// pub(crate) mod repository;
pub(crate) mod db;
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
            log::error!("An error occurred: {e}");
        }
    }
    Ok(())
}

async fn run() -> Result<()> {
    info!("Running setup tasks");
    // let db = SurrealDB::from_env().await?;
    let db = db::connect().await?;
    // let repos = RepositoryProvider::new(db.into());
    let engines = SearchEngines::from_env();
    if !engines.available().await {
        warn!("No search engines are available");
    }
    let browsers = Browsers::new().await?;
    let llm = Ollama::default();
    info!("Running lifecycle tasks");
    handler::run(Data::new(db), Data::new(llm)).await?;
    info!("Running cleanup tasks");
    browsers.close().await?;
    Ok(())
}
