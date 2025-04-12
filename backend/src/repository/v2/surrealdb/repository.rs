use crate::{
    dto::page::{PageRequest, PageResponse},
    repository::traits::{
        AssociatedEntityRepository, NamedStruct, PublicEntityRepository,
    },
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

#[async_trait]
impl<T: Entity, R: Entity> AssociatedEntityRepository<T, R> for SurrealDB {
    // One-to-One relationship methods
    async fn find_related(&self, owner_id: &R::ID) -> Result<Option<T>> {
        Ok(self.connection
            .query("SELECT ->$relationship->$table as $plural_table FROM $owner_id FETCH $plural_table")
            .bind(("relationship", format!("owns_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("plural_table", T::plural_name()))
            .bind(("owner_id", owner_id.to_string()))
            .await?
            .take::<Option<T>>(0)?)
    }

    async fn exists_related(&self, owner_id: &R::ID) -> Result<bool> {
        Ok(self.connection
            .query("SELECT ->$relationship->$table as $plural_table FROM $owner_id FETCH $plural_table")
            .bind(("relationship", format!("owns_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("plural_table", T::plural_name()))
            .bind(("owner_id", owner_id.to_string()))
            .await?
            .take::<Option<T>>(0)?
            .is_some())
    }

    async fn create_owned(&self, subject: T, owner_id: &R::ID) -> Result<T> {
        self.connection
            .query("$created = CREATE $table CONTENT $subject")
            .query("RELATE ONLY type::thing($table, $owner_id)->$relationship->$created")
            .query("RETURN $created")
            .bind(("table", T::singular_name()))
            .bind(("subject", subject))
            .bind(("owner_id", owner_id.to_string()))
            .bind(("relationship", format!("owns_{}", T::singular_name())))
            .await?
            .take::<Option<T>>(2)?
            .ok_or(SDBError::NotFoundRecentUpdate(T::singular_name().into()))
    }

    async fn relate(&self, subject_id: &T::ID, owner_id: &R::ID) -> Result<()> {
        self.connection
            .query("RELATE ONLY $type::thing($table, $id)->$relationship->type::thing($related_table, $related_id)")
            .bind(("table", R::singular_name()))
            .bind(("id", owner_id.to_string()))
            .bind(("relationship", format!("owns_{}", T::singular_name())))
            .bind(("related_table", T::singular_name()))
            .bind(("related_id", subject_id.to_string()))
            .await?;
        Ok(())
    }

    // One-to-Many relationship methods
    async fn find_children(&self, parent_id: &R::ID, page: PageRequest) -> Result<PageResponse<T>> {
        todo!()
    }

    async fn count_children(&self, parent_id: &R::ID) -> Result<u64> {
        todo!()
    }

    async fn create_child(&self, entity: T, parent_id: &R::ID) -> Result<T> {
        todo!()
    }

    async fn create_children(&self, entities: Vec<T>, parent_id: &R::ID) -> Result<Vec<T>> {
        todo!()
    }

    async fn delete_children(&self, parent_id: &R::ID) -> Result<u64> {
        todo!()
    }

    // Many-to-Many relationship methods
    async fn find_associated(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>> {
        todo!()
    }

    async fn find_associated_to(
        &self,
        entity_id: &T::ID,
        page: PageRequest,
    ) -> Result<PageResponse<R>> {
        todo!()
    }

    async fn count_associated(&self, related_id: &R::ID) -> Result<u64> {
        todo!()
    }

    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        todo!()
    }

    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        todo!()
    }

    async fn create_associated(&self, entity: T, related_id: &R::ID) -> Result<T> {
        todo!()
    }

    async fn is_associated(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<bool> {
        todo!()
    }

    async fn dissociate_all(&self, entity_id: &T::ID) -> Result<u64> {
        todo!()
    }

    async fn dissociate_from_all(&self, related_id: &R::ID) -> Result<u64> {
        todo!()
    }
}
