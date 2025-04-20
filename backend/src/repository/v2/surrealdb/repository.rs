use crate::{
    dto::page::{PageRequest, PageResponse},
    repository::traits::{AssociatedEntityRepository, NamedStruct, PublicEntityRepository},
};
use async_trait::async_trait;
use backend_derive::named_struct;
use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::{
    model::user::User,
    repository::traits::{Entity, Repository},
    Error,
};

use super::{
    models::SDBError,
    relationships::{owns_rel_name, has_child_rel_name, associates_with_rel_name},
    SurrealDB
};

type Result<T> = std::result::Result<T, SDBError>;

#[derive(Serialize, Deserialize, Debug)]
struct SurrealDBRelationship {
    id: Thing,
    r#in: Thing,
    out: Thing,
}

#[async_trait]
impl<T: Entity> Repository<T> for SurrealDB {
    type Error = SDBError;

    async fn save(&self, entity: T) -> Result<T> {
        let name = T::singular_name();
        if let Some(id) = entity.id() {
            debug!("Executing query: UPDATE {} {} WITH {:?}", name, id, entity);
            let result = self
                .connection
                .update((name, id.to_string()))
                .content(entity)
                .await?;
            debug!("Query result: {:?}", result);
            Ok(result.ok_or(SDBError::NotFoundRecentUpdate(name.into()))?)
        } else {
            debug!("Executing query: CREATE {} CONTENT {:?}", name, entity);
            let result = self
                .connection
                .create(name)
                .content(entity)
                .await?;
            debug!("Query result: {:?}", result);
            Ok(result.ok_or_else(|| SDBError::NotFoundRecentUpdate(name.into()))?)
        }
    }

    async fn delete(&self, id: &T::ID) -> Result<()> {
        debug!("Executing query: DELETE {}:{}", T::singular_name(), id);
        let result = self.connection
            .delete((T::singular_name(), id.to_string()))
            .await?;
        debug!("Query result: {:?}", result);
        result.map(|e: T| ())
            .ok_or(SDBError::NotFound)
    }

    async fn exists(&self, id: &T::ID) -> Result<bool> {
        let query = "SELECT id FROM type::thing($table, $id)";
        let id_str = id.to_string();
        let binds = [("table", T::singular_name()), ("id", id_str.as_str())];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self
            .connection
            .query(query)
            .bind(("table", T::singular_name()))
            .bind(("id", id.to_string()))
            .await?;

        let exists = result.take::<Option<Thing>>((0, "id"))?.is_some();
        debug!("Query result: exists = {}", exists);
        Ok(exists)
    }

    async fn count(&self) -> Result<u64> {
        let query = "SELECT count() FROM type::table($table) GROUP ALL;";
        let binds = [("table", T::singular_name())];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self
            .connection
            .query(query)
            .bind(("table", T::singular_name()))
            .await?;

        let count = result.take::<Option<u64>>((0, "count"))?.unwrap_or(0);
        debug!("Query result: count = {}", count);
        Ok(count)
    }

    async fn find(&self, id: &T::ID) -> Result<Option<T>> {
        debug!("Executing query: SELECT * FROM {}:{}", T::singular_name(), id);
        let result = self
            .connection
            .select((T::singular_name(), id.to_string()))
            .await?;
        debug!("Query result: {:?}", result);
        Ok(result)
    }
}

#[async_trait]
impl<T: Entity> PublicEntityRepository<T> for SurrealDB {
    async fn find_all(&self, page: PageRequest) -> Result<PageResponse<T>> {
        let query = "SELECT * FROM type::table($table) LIMIT $limit START $start;";
        let limit_str = page.size().to_string();
        let start_str = ((page.page() - 1) * page.size()).to_string();
        let binds = [
            ("table", T::singular_name()),
            ("limit", limit_str.as_str()),
            ("start", start_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("table", T::singular_name()))
            .bind(("limit", page.size()))
            .bind(("start", (page.page() - 1) * page.size()))
            .await?;

        let entities = result.take::<Vec<T>>(0)?;
        debug!("Query result: found {} entities", entities.len());
        Ok(PageResponse::from(entities, page.page()))
    }
}

#[async_trait]
impl<T: Entity, R: Entity> AssociatedEntityRepository<T, R> for SurrealDB {
    // One-to-One relationship methods
    async fn find_related(&self, owner_id: &T::ID) -> Result<Option<R>> {
        let query = format!("SELECT ->{}->{} as result FROM type::thing($owner_table, $owner_id) FETCH result", owns_rel_name(R::singular_name()), R::singular_name());
        let owner_id_str = owner_id.to_string();
        let binds = [("owner_table", T::singular_name()), ("owner_id", owner_id_str.as_str())];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("owner_table", T::singular_name()))
            .bind(("owner_id", owner_id.to_string()))
            .await?;

        let related = result
            .take::<Vec<Vec<R>>>((0, "result"))?
            .into_iter()
            .next()
            .and_then(|v| v.into_iter().next());

        debug!("Query result: related entity found: {}", related.is_some());
        Ok(related)
    }

    async fn exists_related(&self, owner_id: &T::ID) -> Result<bool> {
        let query = format!("SELECT ->{}->{} as result FROM type::thing($owner_table, $owner_id) FETCH result", owns_rel_name(R::singular_name()), R::singular_name());
        let owner_id_str = owner_id.to_string();
        let binds = [
            ("owner_table", T::singular_name()),
            ("owner_id", owner_id_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("owner_table", T::singular_name()))
            .bind(("owner_id", owner_id.to_string()))
            .await?;

        let exists = result
            .take::<Vec<Vec<R>>>((0, "result"))?
            .into_iter()
            .next()
            .and_then(|v| v.into_iter().next())
            .is_some();

        debug!("Query result: related entity exists = {}", exists);
        Ok(exists)
    }

    async fn create_owned(&self, subject: R, owner_id: &T::ID) -> Result<R> {
        let query1 = "$created = CREATE $table CONTENT $subject";
        let query2 = format!("RELATE $owner->{}->$created", owns_rel_name(R::singular_name()));
        let query3 = "RETURN $created";
        let owner_bind = Thing::from((T::singular_name(), owner_id.to_string().as_str()));

        debug!(
            "Executing multi-query: [1] {}, [2] {}, [3] {} with binds: table={}, subject={:?}, owner={:?}",
            query1, query2, query3, R::singular_name(), subject, owner_bind
        );

        let mut result = self.connection
            .query(query1)
            .query(query2)
            .query(query3)
            .bind(("table", R::singular_name()))
            .bind(("subject", subject))
            .bind(("owner", owner_bind))
            .await?;

        debug!("Multi-query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected in query {}: {:?}", index, err);
            return Err(err.into());
        }

        let created = result.take::<Option<R>>(2)?;
        debug!("Created owned entity: {:?}", created);
        created.ok_or(SDBError::NotFoundRecentUpdate(R::singular_name().into()))
    }

    async fn relate(&self, subject_id: &R::ID, owner_id: &T::ID) -> Result<()> {
        let query_fmt = format!("RELATE ONLY $owner->{}->$subject", owns_rel_name(R::singular_name()));
        let subject_bind = Thing::from((R::singular_name(), subject_id.to_string().as_str()));
        let owner_bind = Thing::from((T::singular_name(), owner_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: subject={:?}, owner={:?}",
            query_fmt, subject_bind, owner_bind
        );

        let errors = self
            .connection
            .query(query_fmt)
            .bind((
                "owner",
                owner_bind,
            ))
            .bind((
                "subject",
                subject_bind,
            ))
            .await?
            .take_errors();

        debug!("Query executed, errors: {:?}", errors);
        if let Some((index, err)) = errors.into_iter().next() {
            return Err(err.into());
        }
        Ok(())
    }

    // One-to-Many relationship methods
    async fn find_children(&self, parent_id: &T::ID, page: PageRequest) -> Result<PageResponse<R>> {
        let query = format!("SELECT ->{}->{} as result FROM type::thing($parent_table, $parent_id) LIMIT $limit START $start FETCH result",
                          has_child_rel_name(T::singular_name()), R::singular_name());
        let parent_id_str = parent_id.to_string();
        let limit_str = page.size().to_string();
        let start_str = ((page.page() - 1) * page.size()).to_string();
        let binds = [
            ("parent_table", T::singular_name()),
            ("parent_id", parent_id_str.as_str()),
            ("limit", limit_str.as_str()),
            ("start", start_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("parent_table", T::singular_name()))
            .bind(("parent_id", parent_id.to_string()))
            .bind(("limit", page.size()))
            .bind(("start", (page.page() - 1) * page.size()))
            .await?;

        let children_nested = result.take::<Vec<Vec<R>>>((0, "result"))?;
        // Flatten the nested vectors
        let flattened: Vec<R> = children_nested.into_iter().flatten().collect();
        debug!("Query result: found {} children", flattened.len());
        Ok(PageResponse::from(flattened, page.page()))
    }

    async fn count_children(&self, parent_id: &T::ID) -> Result<u64> {
        let query = format!("SELECT count(->{}->{}) as count FROM type::thing($parent_table, $parent_id)",
                         has_child_rel_name(T::singular_name()), R::singular_name());
        let parent_id_str = parent_id.to_string();
        let binds = [
            ("parent_table", T::singular_name()),
            ("parent_id", parent_id_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("parent_table", T::singular_name()))
            .bind(("parent_id", parent_id.to_string()))
            .await?;

        let count = result.take::<Option<u64>>((0, "count"))?.unwrap_or(0);
        debug!("Query result: child count = {}", count);
        Ok(count)
    }

    async fn create_child(&self, entity: R, parent_id: &T::ID) -> Result<R> {
        let query1 = "$created = CREATE type::table($table) CONTENT $entity";
        let query2 = format!("RELATE $parent->{}->$created", has_child_rel_name(T::singular_name()));
        let query3 = "RETURN $created";
        let parent_bind = Thing::from((T::singular_name(), parent_id.to_string().as_str()));

        debug!(
            "Executing multi-query: [1] {}, [2] {}, [3] {} with binds: table={}, entity={:?}, parent={:?}",
            query1, query2, query3, R::singular_name(), entity, parent_bind
        );

        let mut result = self.connection
            .query(query1)
            .query(query2)
            .query(query3)
            .bind(("table", R::singular_name()))
            .bind(("entity", entity))
            .bind(("parent", parent_bind))
            .await?;

        debug!("Multi-query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected in query {}: {:?}", index, err);
            return Err(err.into());
        }

        let created = result.take::<Option<R>>(2)?;
        debug!("Created child entity: {:?}", created);
        created.ok_or(SDBError::NotFoundRecentUpdate(R::singular_name().into()))
    }

    async fn create_children(&self, entities: Vec<R>, parent_id: &T::ID) -> Result<Vec<R>> {
        let query1 = "$created = CREATE $table CONTENT $entities";
        let query2 = format!("RELATE $parent->{}->$created", has_child_rel_name(T::singular_name()));
        let query3 = "RETURN $created";
        let parent_bind = Thing::from((T::singular_name(), parent_id.to_string().as_str()));

        debug!(
            "Executing multi-query: [1] {}, [2] {}, [3] {} with binds: table={}, entities_count={}, parent={:?}",
            query1, query2, query3, R::singular_name(), entities.len(), parent_bind
        );

        let mut result = self
            .connection
            .query(query1)
            .query(query2)
            .query(query3)
            .bind(("table", R::singular_name()))
            .bind(("entities", entities))
            .bind(("parent", parent_bind))
            .await?;

        debug!("Multi-query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected in query {}: {:?}", index, err);
            return Err(err.into());
        }

        let created = result.take::<Vec<R>>(2)?;
        debug!("Created {} child entities", created.len());
        Ok(created)
    }

    async fn delete_children(&self, parent_id: &T::ID) -> Result<()> {
        let query = format!("DELETE {} WHERE <-{}<-$parent", R::singular_name(), has_child_rel_name(T::singular_name()));
        let parent_bind = Thing::from((T::singular_name(), parent_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: parent={:?}",
            query, parent_bind
        );

        let mut result = self.connection
            .query(query)
            .bind(("parent", parent_bind))
            .await?;

        debug!("Query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected: {:?}", err);
            return Err(err.into());
        }

        debug!("Children deleted successfully");
        Ok(())
    }

    // Many-to-Many relationship methods
    async fn find_associated(
        &self,
        entity_id: &T::ID,
        page: PageRequest,
    ) -> Result<PageResponse<R>> {
        let query = format!("SELECT ->{}->{} as result FROM type::thing($entity_table, $entity_id) LIMIT $limit START $start FETCH result",
                          associates_with_rel_name(R::singular_name()), R::singular_name());
        let entity_id_str = entity_id.to_string();
        let limit_str = page.size().to_string();
        let start_str = ((page.page() - 1) * page.size()).to_string();
        let binds = [
            ("entity_table", T::singular_name()),
            ("entity_id", entity_id_str.as_str()),
            ("limit", limit_str.as_str()),
            ("start", start_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("entity_table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .bind(("limit", page.size()))
            .bind(("start", (page.page() - 1) * page.size()))
            .await?;

        let associated_nested = result.take::<Vec<Vec<R>>>((0, "result"))?;
        // Flatten the nested vectors
        let flattened: Vec<R> = associated_nested.into_iter().flatten().collect();
        debug!("Query result: found {} associated entities", flattened.len());
        Ok(PageResponse::from(flattened, page.page()))
    }

    async fn find_associated_to(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>> {
        let query = format!("SELECT <-{}<-{} as result FROM type::thing($related_table, $related_id) LIMIT $limit START $start FETCH result",
                          associates_with_rel_name(R::singular_name()), T::singular_name());
        let related_id_str = related_id.to_string();
        let limit_str = page.size().to_string();
        let start_str = ((page.page() - 1) * page.size()).to_string();
        let binds = [
            ("related_table", R::singular_name()),
            ("related_id", related_id_str.as_str()),
            ("limit", limit_str.as_str()),
            ("start", start_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("related_table", R::singular_name()))
            .bind(("related_id", related_id.to_string()))
            .bind(("limit", page.size()))
            .bind(("start", (page.page() - 1) * page.size()))
            .await?;

        let associated_nested = result.take::<Vec<Vec<T>>>((0, "result"))?;
        // Flatten the nested vectors
        let flattened: Vec<T> = associated_nested.into_iter().flatten().collect();
        debug!("Query result: found {} associated entities", flattened.len());
        Ok(PageResponse::from(flattened, page.page()))
    }

    async fn count_associated(&self, entity_id: &T::ID) -> Result<u64> {
        let query = format!("SELECT count(->{}->{}) as count FROM type::thing($entity_table, $entity_id)",
                         associates_with_rel_name(R::singular_name()), R::singular_name());
        let entity_id_str = entity_id.to_string();
        let binds = [
            ("entity_table", T::singular_name()),
            ("entity_id", entity_id_str.as_str())
        ];
        debug!("Executing query: {} with binds: {:?}", query, binds);

        let mut result = self.connection
            .query(query)
            .bind(("entity_table", T::singular_name()))
            .bind(("entity_id", entity_id.to_string()))
            .await?;

        let count = result.take::<Option<u64>>((0, "count"))?.unwrap_or(0);
        debug!("Query result: associated count = {}", count);
        Ok(count)
    }

    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        let query_fmt = format!("RELATE $entity->{}->$related", associates_with_rel_name(R::singular_name()));
        let entity_bind = Thing::from((T::singular_name(), entity_id.to_string().as_str()));
        let related_bind = Thing::from((R::singular_name(), related_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: entity={:?}, related={:?}",
            query_fmt, entity_bind, related_bind
        );

        let errors = self
            .connection
            .query(query_fmt)
            .bind((
                "entity",
                entity_bind,
            ))
            .bind((
                "related",
                related_bind,
            ))
            .await?
            .take_errors();

        debug!("Query executed, errors: {:?}", errors);
        if let Some((index, err)) = errors.into_iter().next() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()> {
        let query_fmt = format!("DELETE {} WHERE in = $related AND out = $entity", associates_with_rel_name(R::singular_name()));
        let entity_bind = Thing::from((T::singular_name(), entity_id.to_string().as_str()));
        let related_bind = Thing::from((R::singular_name(), related_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: entity={:?}, related={:?}",
            query_fmt, entity_bind, related_bind
        );

        let mut result = self.connection
            .query(query_fmt)
            .bind(("entity", entity_bind))
            .bind(("related", related_bind))
            .await?;

        debug!("Query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected: {:?}", err);
            return Err(err.into());
        }

        debug!("Dissociation successful");
        Ok(())
    }

    async fn create_associated(&self, entity_id: &T::ID, related: R) -> Result<R> {
        let query1 = "$created = CREATE $table CONTENT $related";
        let query2 = format!("RELATE $entity->{}->$created", associates_with_rel_name(R::singular_name()));
        let query3 = "RETURN $created";
        let entity_bind = Thing::from((T::singular_name(), entity_id.to_string().as_str()));

        debug!(
            "Executing multi-query: [1] {}, [2] {}, [3] {} with binds: table={}, related={:?}, entity={:?}",
            query1, query2, query3, R::singular_name(), related, entity_bind
        );

        let mut result = self.connection
            .query(query1)
            .query(query2)
            .query(query3)
            .bind(("table", R::singular_name()))
            .bind(("related", related))
            .bind(("entity", entity_bind))
            .await?;

        debug!("Multi-query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected in query {}: {:?}", index, err);
            return Err(err.into());
        }

        let created = result.take::<Option<R>>(2)?;
        debug!("Created associated entity: {:?}", created);
        created.ok_or(SDBError::NotFoundRecentUpdate(R::singular_name().into()))
    }

    async fn is_associated(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<bool> {
        let query = format!("SELECT * FROM {} WHERE out = $entity AND in = $related", associates_with_rel_name(R::singular_name()));
        let entity_bind = Thing::from((T::singular_name(), entity_id.to_string().as_str()));
        let related_bind = Thing::from((R::singular_name(), related_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: entity={:?}, related={:?}",
            query, entity_bind, related_bind
        );

        let mut result = self.connection
            .query(query)
            .bind(("entity", entity_bind))
            .bind(("related", related_bind))
            .await?;

        debug!("Query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected: {:?}", err);
            return Err(err.into());
        }

        let is_associated = !result.take::<Vec<SurrealDBRelationship>>(0)?.is_empty();
        debug!("Query result: is_associated = {}", is_associated);
        Ok(is_associated)
    }

    async fn dissociate_all(&self, entity_id: &T::ID) -> Result<u64> {
        let query = format!("DELETE {} WHERE out = $entity", associates_with_rel_name(R::singular_name()));
        let entity_bind = Thing::from((T::singular_name(), entity_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: entity={:?}",
            query, entity_bind
        );

        let mut result = self.connection
            .query(query)
            .bind(("entity", entity_bind))
            .await?;

        debug!("Query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected: {:?}", err);
            return Err(err.into());
        }

        let objects = result.take::<Vec<surrealdb::sql::Object>>(0)?;
        let count = objects.len() as u64;
        debug!("Dissociated all: removed {} relationships", count);
        Ok(count)
    }

    async fn dissociate_from_all(&self, related_id: &R::ID) -> Result<u64> {
        let query = format!("DELETE {} WHERE in = $related", associates_with_rel_name(R::singular_name()));
        let related_bind = Thing::from((R::singular_name(), related_id.to_string().as_str()));

        debug!(
            "Executing query: {} with binds: related={:?}",
            query, related_bind
        );

        let mut result = self.connection
            .query(query)
            .bind(("related", related_bind))
            .await?;

        debug!("Query executed, checking for errors");
        // Check for errors
        if let Some((index, err)) = result.take_errors().into_iter().next() {
            debug!("Error detected: {:?}", err);
            return Err(err.into());
        }

        let objects = result.take::<Vec<surrealdb::sql::Object>>(0)?;
        let count = objects.len() as u64;
        debug!("Dissociated from all: removed {} relationships", count);
        Ok(count)
    }
}
