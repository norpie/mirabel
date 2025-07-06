use crate::prelude::*;

use diesel::{Connection, PgConnection};
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::debug;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type PostgresPool = Pool<AsyncPgConnection>;

pub async fn connect() -> Result<Pool<AsyncPgConnection>> {
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
    debug!("Connecting to database with connection string: {}", url);
    {
        let mut conn = PgConnection::establish(&url)?;
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&url);
    let pool: Pool<AsyncPgConnection> = Pool::builder().build(config).await?;
    Ok(pool)
}
