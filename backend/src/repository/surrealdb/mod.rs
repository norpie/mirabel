use crate::{dto::page::PageRequest, prelude::*};

use std::{env, sync::Arc};

use actix_web::web::Data;
use async_trait::async_trait;
use futures::TryFutureExt;
use include_dir::{include_dir, Dir};
use log::{debug, info};
use surrealdb::{
    engine::remote::ws::{self, Client, Ws},
    opt::auth::Root,
    Surreal,
};
use surrealdb_migrations::MigrationRunner;

use super::Repository;

pub mod avatars;
pub mod sessions;
pub mod users;
pub mod workspaces;

const MIGRATOR_DIR: Dir<'_> = include_dir!("../surrealdb");

#[derive(Clone)]
pub struct SurrealDB(Surreal<Client>);

impl Repository for SurrealDB {}

pub struct SurrealDbPagination {
    limit: i32,
    start: i32,
}

impl From<PageRequest> for SurrealDbPagination {
    fn from(page: PageRequest) -> Self {
        let limit = page.size();
        let start = page.page() * page.size();
        Self { limit, start }
    }
}

impl SurrealDB {
    pub async fn setup() -> Result<Data<Box<dyn Repository>>> {
        let host = env::var("DATABASE_HOST")?;
        let port = env::var("DATABASE_PORT")?;
        let ns = env::var("DATABASE_NS")?;
        let db = env::var("DATABASE_DB")?;
        let user = env::var("DATABASE_USER")?;
        let pass = env::var("DATABASE_PASS")?;

        info!("Connecting to database at `{}:{}`", &host, &port);
        let conn = Surreal::new::<Ws>(format!("{}:{}", &host, &port)).await?;

        // Signin as a namespace, database, or root user
        debug!("Signing in as `root` user");
        conn.signin(Root {
            username: &user,
            password: &pass,
        })
        .await?;

        debug!("Using namespace `{}` and database `{}`", &ns, &db);
        conn.use_ns(ns).use_db(db).await?;

        debug!("Running migrations");
        let runner = MigrationRunner::new(&conn);
        let runner = runner.load_files(&MIGRATOR_DIR);
        runner.validate_version_order().await?;
        runner.up().await?;

        Ok(Data::new(Box::new(Self(conn))))
    }
}
