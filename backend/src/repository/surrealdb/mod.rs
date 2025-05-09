use crate::prelude::*;

use async_trait::async_trait;
use builder::SurrealDBBuilder;
use log::{debug, info};
use surrealdb::{engine::remote::ws::Client, Surreal};

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
        debug!("Initializing SurrealDB from environment variables");
        let url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "ws://localhost:8000".into());
        let auth_type = std::env::var("SURREALDB_TYPE").unwrap_or_else(|_| "root".to_string());
        let username = std::env::var("SURREALDB_USERNAME").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let namespace =
            std::env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "mirabel".to_string());
        let database =
            std::env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "mirabel".to_string());

        debug!("Connecting to SurrealDB at {url} with {auth_type} authentication");
        let db = Self::from_vars(
            &url, &auth_type, &username, &password, &namespace, &database,
        )
        .await?;
        info!("Successfully connected to SurrealDB at {url}");
        Ok(db)
    }

    pub async fn from_vars(
        url: &str,
        auth_type: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self> {
        debug!("Connecting to SurrealDB with explicit configuration");
        let result = match auth_type {
            "namespace" => {
                debug!("Using namespace authentication");
                SurrealDBBuilder::new(url)
                    .with_namespace_user(namespace, username, password)
                    .use_db(database)
                    .build()
                    .await
            }
            "database" => {
                debug!("Using database authentication");
                SurrealDBBuilder::new(url)
                    .with_database_user(namespace, database, username, password)
                    .build()
                    .await
            }
            _ => {
                debug!("Using root authentication");
                SurrealDBBuilder::new(url)
                    .with_root(username, password)
                    .use_ns(namespace)
                    .use_db(database)
                    .build()
                    .await
            }
        };
        match &result {
            Ok(_) => info!("Successfully established database connection"),
            Err(e) => debug!("Failed to connect to database: {e}"),
        }
        result
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
    use crate::repository::surrealdb::builder::SurrealDBBuilder;
    use crate::repository::{self};

    use dotenvy::EnvLoader;

    async fn get_test_db() -> SurrealDB {
        SurrealDBBuilder::new("localhost:8000")
            .with_root("root", "root")
            .use_ns("test")
            .use_db("test")
            .build()
            .await
            .unwrap()
    }

    fn init_test_env() {
        let _ = EnvLoader::new().load();
        let _ = env_logger::try_init();
    }

    #[tokio::test]
    async fn test_ping() {
        init_test_env();
        let db = get_test_db().await;
        assert!(db.ping().await.unwrap());
    }

    #[tokio::test]
    async fn test_repository() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_repository(db).await;
    }

    // #[tokio::test]
    // async fn test_field_searchable_repository() {
    //     init_test_env();
    //     let db = get_test_db().await;
    //     repository::tests::test_field_searchable_repository(db).await;
    // }

    #[tokio::test]
    async fn test_field_findable_repository() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_field_findable_repository(db).await;
    }

    #[tokio::test]
    async fn test_public_entity_repository() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_public_entity_repository(db).await;
    }

    #[tokio::test]
    async fn test_associated_entity_one_to_one() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_associated_entity_one_to_one(db.clone(), db).await;
    }

    #[tokio::test]
    async fn test_associated_entity_one_to_many() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_associated_entity_one_to_many(db.clone(), db).await;
    }

    #[tokio::test]
    async fn test_associated_entity_many_to_many() {
        init_test_env();
        let db = get_test_db().await;
        repository::tests::test_associated_entity_many_to_many(db.clone(), db).await;
    }
}
