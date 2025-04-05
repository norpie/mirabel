use crate::repository::traits::NamedStruct;
use async_trait::async_trait;
use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    model::user::User,
    repository::traits::{Entity, Repository},
    Error,
};

use super::SurrealDB;

#[derive(Debug, thiserror::Error)]
pub enum SDBError {
    #[error("Generic surrealdb error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("Not found recent update for `{0}`")]
    NotFoundRecentUpdate(String),
    #[error("Not found")]
    NotFound,
}

#[derive(Deserialize)]
struct SDBCount(Vec<Count>);

impl SDBCount {
    fn count(&self) -> u64 {
        self.0.first().map_or(0, |c| c.count)
    }
}

#[derive(Deserialize)]
struct Count {
    count: u64,
}

type Result<T> = std::result::Result<T, SDBError>;

#[async_trait]
impl<T: Entity> Repository<T> for SurrealDB {
    type Error = SDBError;

    async fn save(&self, entity: T) -> Result<T> {
        let name = T::singular_name();
        if let Some(id) = entity.id() {
            Ok(self
                .connection
                .update((name, id.to_string()))
                .content(entity)
                .await?
                .ok_or(SDBError::NotFoundRecentUpdate(name.into()))?)
        } else {
            Ok(self
                .connection
                .create(name)
                .content(entity)
                .await?
                .ok_or_else(|| SDBError::NotFoundRecentUpdate(name.into()))?)
        }
    }

    async fn delete(&self, id: &T::ID) -> Result<()> {
        self.connection
            .delete((T::singular_name(), id.to_string()))
            .await?
            .map(|e: T| ())
            .ok_or(SDBError::NotFound)
    }

    async fn exists(&self, id: &T::ID) -> Result<bool> {
        Ok(self
            .connection
            .query("SELECT id FROM type::thing($table, $id)")
            .bind(("table", T::singular_name()))
            .bind(("id", id.to_string()))
            .await?
            .take::<Option<Thing>>((0, "id"))?
            .is_some())
    }

    async fn count(&self) -> Result<u64> {
        Ok(self
            .connection
            .query("SELECT count(id) FROM type::table($table);")
            .bind(("table", T::singular_name()))
            .await?
            .take::<Option<u64>>((0, "count"))?
            .unwrap_or(0))
    }

    async fn find(&self, id: &T::ID) -> Result<Option<T>> {
        Ok(self
            .connection
            .select((T::singular_name(), id.to_string()))
            .await?)
    }
}
