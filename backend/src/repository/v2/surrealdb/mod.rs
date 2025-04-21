use crate::prelude::*;

use async_trait::async_trait;
use builder::SurrealDBBuilder;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use thiserror::Error;

use crate::{repository::traits::Database, Error};

pub(crate) mod builder;
pub(crate) mod models;
pub(crate) mod relationships;
pub(crate) mod repository;

#[derive(Debug, Clone)]
pub struct SurrealDB {
    connection: Surreal<Client>,
}

impl SurrealDB {
    pub async fn from_env() -> Result<Self> {
        let url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "ws://localhost:8000".into());
        let auth_type = std::env::var("SURREALDB_TYPE").unwrap_or_else(|_| "root".to_string());
        let username = std::env::var("SURREALDB_USERNAME").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let namespace =
            std::env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "mirabel".to_string());
        let database =
            std::env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "mirabel".to_string());

        Self::from_vars(
            &url, &auth_type, &username, &password, &namespace, &database,
        )
        .await
    }

    pub async fn from_vars(
        url: &str,
        auth_type: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self> {
        match auth_type {
            "namespace" => {
                SurrealDBBuilder::new(url)
                    .with_namespace_user(namespace, username, password)
                    .use_db(database)
                    .build()
                    .await
            }
            "database" => {
                SurrealDBBuilder::new(url)
                    .with_database_user(namespace, database, username, password)
                    .build()
                    .await
            }
            _ => {
                SurrealDBBuilder::new(url)
                    .with_root(username, password)
                    .use_ns(namespace)
                    .use_db(database)
                    .build()
                    .await
            }
        }
    }
}

impl From<Surreal<Client>> for SurrealDB {
    fn from(connection: Surreal<Client>) -> Self {
        SurrealDB { connection }
    }
}

#[async_trait]
impl Database for SurrealDB {
    type Error = Error;

    async fn ping(&self) -> Result<bool> {
        self.connection.version().await?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::v2::surrealdb::builder::SurrealDBBuilder;
    use crate::repository::{self, Repository};
    use actix_web::web::Data;
    use dotenvy::EnvLoader;
    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::Surreal;

    #[tokio::test]
    async fn test() {
        EnvLoader::new().load().unwrap();
        env_logger::init();
        let db = SurrealDBBuilder::new("localhost:8000")
            .with_root("root", "root")
            .use_ns("test")
            .use_db("test")
            .build()
            .await
            .unwrap();
        repository::v2::tests::test_repository(db.clone()).await;
        // repository::v2::tests::test_field_searchable_repository(db).await;
        // repository::v2::tests::test_field_findable_repository(db).await;
        repository::v2::tests::test_public_entity_repository(db.clone()).await;
        repository::v2::tests::test_associated_entity_one_to_one(db.clone(), db.clone()).await;
        repository::v2::tests::test_associated_entity_one_to_many(db.clone(), db.clone()).await;
        repository::v2::tests::test_associated_entity_many_to_many(db.clone(), db.clone()).await;
    }
}
