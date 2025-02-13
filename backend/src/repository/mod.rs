use crate::prelude::*;

use std::env;

use futures::TryFutureExt;
use surrealdb::{
    engine::remote::ws::{self, Client, Ws},
    opt::auth::Root,
    Surreal,
};
use surrealdb_migrations::MigrationRunner;

pub type SurrealDB = Surreal<Client>;

pub async fn setup() -> Result<SurrealDB> {
    let host = env::var("DATABASE_HOST")?;
    let port = env::var("DATABASE_PORT")?;
    let ns = env::var("DATABASE_NS")?;
    let db = env::var("DATABASE_DB")?;
    let user = env::var("DATABASE_USER")?;
    let pass = env::var("DATABASE_PASS")?;
    let conn = Surreal::new::<Ws>(format!("ws://{}:{}", &host, &port)).await?;

    // Signin as a namespace, database, or root user
    conn.signin(Root {
        username: &user,
        password: &pass,
    })
    .await?;

    conn.use_ns(ns).use_db(db).await?;

    MigrationRunner::new(&conn).up().await?;

    Ok(conn)
}
