use crate::{
    dto::page::{PageRequest, PageResponse},
    repository::traits::{AssociatedEntityRepository, NamedStruct, PublicEntityRepository},
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
            .query("SELECT count() FROM type::table($table) GROUP ALL;")
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
            .query("RELATE ONLY $owner->$relationship->$subject")
            .bind(("owner", Thing::from((R::singular_name(), owner_id.to_string().as_str()))))
            .bind(("relationship", format!("owns_{}", T::singular_name())))
            .bind(("subject", Thing::from((T::singular_name(), subject_id.to_string().as_str()))))
            .await?;
        Ok(())
    }

    // One-to-Many relationship methods
    async fn find_children(&self, parent_id: &R::ID, page: PageRequest) -> Result<PageResponse<T>> {
        Ok(PageResponse::from(self.connection
            .query("SELECT ->$relationship->$table as $plural_table FROM $parent_id FETCH $plural_table LIMIT $limit START $start")
            .bind(("relationship", format!("has_child_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("plural_table", T::plural_name()))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("limit", page.size()))
            .bind(("start", (page.page() - 1) * page.size()))
            .await?
            .take::<Vec<T>>(0)?, page.page()))
    }

    async fn count_children(&self, parent_id: &R::ID) -> Result<u64> {
        self.connection
            .query("SELECT count(->$relationship->$table) FROM type::thing($table, $parent_id)")
            .bind(("table", T::singular_name()))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("relationship", format!("has_child_{}", T::singular_name())))
            .await?
            .take::<Option<u64>>((0, "count"))?
            .ok_or(SDBError::NotFoundRecentUpdate(T::singular_name().into()))
    }

    async fn create_child(&self, entity: T, parent_id: &R::ID) -> Result<T> {
        self.connection
            .query("$created = CREATE $table CONTENT $entity")
            .query("RELATE type::thing($table, $parent_id)->$relationship->$created")
            .query("RETURN $created")
            .bind(("table", T::singular_name()))
            .bind(("entity", entity))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("relationship", format!("has_child_{}", T::singular_name())))
            .await?
            .take::<Option<T>>(2)?
            .ok_or(SDBError::NotFoundRecentUpdate(T::singular_name().into()))
    }

    async fn create_children(&self, entities: Vec<T>, parent_id: &R::ID) -> Result<Vec<T>> {
        Ok(self
            .connection
            .query("$created = CREATE $table CONTENT $entities")
            .query("RELATE type::thing($table, $parent_id)->$relationship->$created")
            .query("RETURN $created")
            .bind(("table", T::singular_name()))
            .bind(("entities", entities))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("relationship", format!("has_child_{}", T::singular_name())))
            .await?
            .take::<Vec<T>>(2)?)
    }

    async fn delete_children(&self, parent_id: &R::ID) -> Result<()> {
        self.connection
            .query("DELETE $child_table WHERE <-$relationship<-($table WHERE id = type::thing($table, $parent_id))")
            .bind(("child_table", T::singular_name()))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("relationship", format!("has_child_{}", T::singular_name())))
            .await?
            .take::<Vec<()>>(0)?;
        Ok(())
    }

    // Many-to-Many relationship methods
    async fn find_associated(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>> {
        Ok(PageResponse::from(
            self.connection
                .query("SELECT ->$relationship->$table as $plural_table FROM $related_id FETCH $plural_table LIMIT $limit START $start")
                .bind(("relationship", format!("associates_with_{}", T::singular_name())))
                .bind(("table", T::singular_name()))
                .bind(("plural_table", T::plural_name()))
                .bind(("related_id", related_id.to_string()))
                .bind(("limit", page.size()))
                .bind(("start", (page.page() - 1) * page.size()))
                .await?
                .take::<Vec<T>>(0)?,
            page.page(),
        ))
    }

    async fn find_associated_to(
        &self,
        entity_id: &T::ID,
        page: PageRequest,
    ) -> Result<PageResponse<R>> {
        Ok(PageResponse::from(
            self.connection
                .query("SELECT <-$relationship<-$table as $plural_table FROM $entity_id FETCH $plural_table LIMIT $limit START $start")
                .bind(("relationship", format!("associates_with_{}", T::singular_name())))
                .bind(("table", R::singular_name()))
                .bind(("plural_table", R::plural_name()))
                .bind(("entity_id", entity_id.to_string()))
                .bind(("limit", page.size()))
                .bind(("start", (page.page() - 1) * page.size()))
                .await?
                .take::<Vec<R>>(0)?,
            page.page(),
        ))
    }

    async fn count_associated(&self, related_id: &R::ID) -> Result<u64> {
        self.connection
            .query("SELECT count(->$relationship->$table) FROM type::thing($r_table, $related_id)")
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("r_table", R::singular_name()))
            .bind(("related_id", related_id.to_string()))
            .await?
            .take::<Option<u64>>((0, "count"))?
            .ok_or(SDBError::NotFoundRecentUpdate(T::singular_name().into()))
    }

    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        self.connection
            .query("RELATE type::thing($r_table, $related_id)->$relationship->type::thing($table, $entity_id)")
            .bind(("r_table", R::singular_name()))
            .bind(("related_id", related_id.to_string()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .await?;
        Ok(())
    }

    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        self.connection
            .query("DELETE FROM $r_table->$relationship->$table WHERE in = type::thing($table, $entity_id) AND out = type::thing($r_table, $related_id)")
            .bind(("r_table", R::singular_name()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .bind(("related_id", related_id.to_string()))
            .await?;
        Ok(())
    }

    async fn create_associated(&self, entity: T, related_id: &R::ID) -> Result<T> {
        self.connection
            .query("$created = CREATE $table CONTENT $entity")
            .query("RELATE type::thing($r_table, $related_id)->$relationship->$created")
            .query("RETURN $created")
            .bind(("table", T::singular_name()))
            .bind(("entity", entity))
            .bind(("r_table", R::singular_name()))
            .bind(("related_id", related_id.to_string()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .await?
            .take::<Option<T>>(2)?
            .ok_or(SDBError::NotFoundRecentUpdate(T::singular_name().into()))
    }

    async fn is_associated(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<bool> {
        let result = self.connection
            .query("SELECT * FROM $r_table->$relationship->$table WHERE in = type::thing($table, $entity_id) AND out = type::thing($r_table, $related_id)")
            .bind(("r_table", R::singular_name()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .bind(("related_id", related_id.to_string()))
            .await?
            .take::<Vec<surrealdb::sql::Object>>(0)?;

        Ok(!result.is_empty())
    }

    async fn dissociate_all(&self, entity_id: &T::ID) -> Result<u64> {
        let result = self.connection
            .query("DELETE FROM $r_table->$relationship->$table WHERE in = type::thing($table, $entity_id)")
            .bind(("r_table", R::singular_name()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .await?
            .take::<Vec<surrealdb::sql::Object>>(0)?;

        Ok(result.len() as u64)
    }

    async fn dissociate_from_all(&self, related_id: &R::ID) -> Result<u64> {
        let result = self.connection
            .query("DELETE FROM $r_table->$relationship->$table WHERE out = type::thing($r_table, $related_id)")
            .bind(("r_table", R::singular_name()))
            .bind(("relationship", format!("associates_with_{}", T::singular_name())))
            .bind(("table", T::singular_name()))
            .bind(("related_id", related_id.to_string()))
            .await?
            .take::<Vec<surrealdb::sql::Object>>(0)?;

        Ok(result.len() as u64)
    }
}
