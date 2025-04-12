use crate::{
    dto::page::{PageRequest, PageResponse},
    repository::traits::{NamedStruct, PublicEntityRepository},
};
use async_trait::async_trait;
use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    model::user::User,
    repository::traits::{Entity, Repository},
    Error,
};

use super::{models::SDBError, SurrealDB};

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

#[async_trait]
impl<T: Entity> PublicEntityRepository<T> for SurrealDB {
    async fn find_all(&self, page: PageRequest) -> Result<PageResponse<T>> {
        Ok(PageResponse::from(
            self.connection
                .query("SELECT * FROM type::table($table) LIMIT $limit START $start;")
                .bind(("table", T::singular_name()))
                .bind(("limit", page.size()))
                .bind(("start", (page.page() - 1) * page.size()))
                .await?
                .take::<Vec<T>>(0)?,
            page.page(),
        ))
    }
}
