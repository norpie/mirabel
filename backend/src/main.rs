#![allow(dead_code)]
use actix_web::web::Data;
use driver::{
    browser::Browsers,
    search::{traits::SearchEngine, SearchEngines},
};
use log::{info, warn};
use repository::{surrealdb::SurrealDB, RepositoryProvider};

extern crate backend_derive;

use crate::prelude::*;

pub(crate) mod error;
pub(crate) mod prelude;

pub(crate) mod session;
#[macro_use]
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
    let db = SurrealDB::from_env().await?;
    let repos = RepositoryProvider::new(db.into());
    let engines = SearchEngines::from_env();
    if !engines.available().await {
        warn!("No search engines are available");
    }
    let browsers = Browsers::new().await?;
    info!("Running lifecycle tasks");
    handler::run(Data::new(repos)).await?;
    info!("Running cleanup tasks");
    browsers.close().await?;
    Ok(())
}
