use crate::prelude::*;

use deadpool_diesel::postgres::Manager;
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::postgres::Runtime;
use diesel::Connection;
use diesel::PgConnection;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;
use log::debug;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub async fn connect() -> Result<Pool> {
    let opt_url = std::env::var("DATABASE_URL").ok();
    let url = match opt_url {
        Some(url) => url,
        None => {
            let protocol = std::env::var("DB_PROTOCOL").unwrap_or_else(|_| "postgres".to_string());
            let username = std::env::var("DB_USERNAME").unwrap_or_else(|_| "postgres".to_string());
            let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
            let database = std::env::var("DB_DATABASE").unwrap_or_else(|_| "mirabel".to_string());
            let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            format!("{protocol}://{username}:{password}@{host}:{port}/{database}")
        }
    };
    debug!("Connecting to database with connection string: {url}");
    {
        let mut conn = PgConnection::establish(&url)?;
        conn.run_pending_migrations(MIGRATIONS)?;
    }
    let manager = Manager::new(&url, Runtime::Tokio1);
    let pool = Pool::builder(manager).max_size(8).build()?;
    Ok(pool)
}
