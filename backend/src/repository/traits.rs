use crate::prelude::*;
use std::fmt::Display;

use async_trait::async_trait;
use futures::Stream;
use serde::{de::DeserializeOwned, Serialize};

use crate::dto::page::{PageRequest, PageResponse};

pub trait NamedStruct {
    fn singular_name() -> &'static str;
    fn plural_name() -> &'static str;
}

pub trait FieldFindableStruct: Entity {
    fn filterable_fields() -> &'static [&'static str];
}

pub trait FieldSearchableStruct: Entity {
    fn searchable_fields() -> &'static [&'static str];
}

pub trait FieldSortableStruct: Entity {
    fn sortable_fields() -> &'static [&'static str];
}

pub trait Entity:
    std::fmt::Debug + NamedStruct + Send + Sync + Serialize + DeserializeOwned + 'static
{
    type ID: std::fmt::Debug
        + Clone
        + Eq
        + Display
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + 'static;
    fn id(&self) -> Option<Self::ID>;
}

#[async_trait]
pub trait Repository<T: Entity>: Send + Sync + 'static {
    async fn find(&self, id: &T::ID) -> Result<Option<T>>;
    async fn save(&self, entity: T) -> Result<T>;
    async fn delete(&self, id: &T::ID) -> Result<()>;
    async fn exists(&self, id: &T::ID) -> Result<bool>;
    async fn count(&self) -> Result<u64>;
}

#[async_trait]
pub trait FieldSearchableRepository<T: Entity + FieldSearchableStruct>: Repository<T> {
    async fn search(
        &self,
        fields: &[&str],
        query: &str,
        page: PageRequest,
    ) -> Result<PageResponse<T>>;
}

#[async_trait]
pub trait FieldFindableRepository<T: Entity + FieldFindableStruct>: Repository<T> {
    async fn find_single_by_fields(&self, fields: Vec<(&'static str, String)>)
        -> Result<Option<T>>;

    async fn find_by_fields(
        &self,
        fields: Vec<(&'static str, String)>,
        page: PageRequest,
    ) -> Result<PageResponse<T>>;

    async fn exists_by_fields(&self, fields: Vec<(&'static str, String)>) -> Result<bool>;
}

#[async_trait]
pub trait PublicEntityRepository<T: Entity>: Repository<T> {
    async fn find_all(&self, page: PageRequest) -> Result<PageResponse<T>>;
}

#[async_trait]
pub trait AssociatedEntityRepository<T: Entity, R: Entity>: Repository<T> {
    // One-to-One relationship methods
    async fn find_related(&self, owner_id: &T::ID) -> Result<Option<R>>;
    async fn exists_related(&self, owner_id: &T::ID) -> Result<bool>;
    async fn create_owned(&self, subject: R, owner_id: &T::ID) -> Result<R>;
    async fn relate(&self, subject_id: &R::ID, owner_id: &T::ID) -> Result<()>;

    // One-to-Many relationship methods
    async fn find_children(&self, parent_id: &T::ID, page: PageRequest) -> Result<PageResponse<R>>;
    async fn count_children(&self, parent_id: &T::ID) -> Result<u64>;
    async fn create_child(&self, entity: R, parent_id: &T::ID) -> Result<R>;
    async fn create_children(&self, entities: Vec<R>, parent_id: &T::ID) -> Result<Vec<R>>;
    async fn delete_children(&self, parent_id: &T::ID) -> Result<()>;

    // Many-to-Many relationship methods
    async fn find_associated(
        &self,
        entity_id: &T::ID,
        page: PageRequest,
    ) -> Result<PageResponse<R>>;
    async fn find_associated_to(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>>;
    async fn count_associated(&self, entity_id: &T::ID) -> Result<u64>;
    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()>;
    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<()>;
    async fn create_associated(&self, entity_id: &T::ID, related: R) -> Result<R>;
    async fn is_associated(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<bool>;
    async fn dissociate_all(&self, entity_id: &T::ID) -> Result<()>;
    async fn dissociate_from_all(&self, related_id: &R::ID) -> Result<()>;
}

#[async_trait]
pub trait ThroughputRepository<T: Entity>: Repository<T> {
    async fn stream(&self) -> Result<impl Stream<Item = Result<T>>>;
}

#[async_trait]
pub trait LiveRepository<T: Entity>: Repository<T> {
    async fn live_single(&self, entity_id: &T::ID) -> Result<impl Stream<Item = Result<T>>>;
    async fn live_table(&self) -> Result<impl Stream<Item = Result<T>>>;
}

#[async_trait]
pub trait Database {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn ping(&self) -> Result<bool>;
}
