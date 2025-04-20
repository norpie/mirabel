use std::{fmt::Display, marker::PhantomData};

use async_trait::async_trait;
use futures::Stream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::dto::page::{PageRequest, PageResponse};

pub trait NamedStruct {
    fn singular_name() -> &'static str;
    fn plural_name() -> &'static str;
}

pub trait FieldFindableStruct {
    fn filterable_fields() -> &'static [&'static str];
}

pub trait FieldSearchableStruct {
    fn searchable_fields() -> &'static [&'static str];
}

pub trait FieldSortableStruct {
    fn sortable_fields() -> &'static [&'static str];
}

pub trait Entity: std::fmt::Debug + NamedStruct + Send + Sync + Serialize + DeserializeOwned + 'static {
    type ID: std::fmt::Debug + Clone + Eq + Display + Send + Sync + Serialize + DeserializeOwned + 'static;
    fn id(&self) -> Option<Self::ID>;
}

#[async_trait]
pub trait Repository<T: Entity> {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn find(&self, id: &T::ID) -> Result<Option<T>, Self::Error>;
    async fn save(&self, entity: T) -> Result<T, Self::Error>;
    async fn delete(&self, id: &T::ID) -> Result<(), Self::Error>;
    async fn exists(&self, id: &T::ID) -> Result<bool, Self::Error>;
    async fn count(&self) -> Result<u64, Self::Error>;
}

#[async_trait]
pub trait FieldSearchableRepository<T: Entity + FieldSearchableStruct>: Repository<T> {
    async fn search(&self, fields: &[&str], query: &str, page: PageRequest) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait FieldFindableRepository<T: Entity + FieldFindableStruct>: Repository<T> {
    async fn find_by_field(
        &self,
        field: &str,
        value: &str,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
    async fn exists_by_field(&self, field: &str, value: &str) -> Result<bool, Self::Error>;
}

#[async_trait]
pub trait PublicEntityRepository<T: Entity>: Repository<T> {
    async fn find_all(&self, page: PageRequest) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait AssociatedEntityRepository<T: Entity, R: Entity>: Repository<T> {
    // One-to-One relationship methods
    async fn find_related(&self, owner_id: &R::ID) -> Result<Option<T>, Self::Error>;
    async fn exists_related(&self, owner_id: &R::ID) -> Result<bool, Self::Error>;
    async fn create_owned(&self, subject: T, owner_id: &R::ID) -> Result<T, Self::Error>;
    async fn relate(&self, subject_id: &T::ID, owner_id: &R::ID) -> Result<(), Self::Error>;

    // One-to-Many relationship methods
    async fn find_children(
        &self,
        parent_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
    async fn count_children(&self, parent_id: &R::ID) -> Result<u64, Self::Error>;
    async fn create_child(&self, entity: T, parent_id: &R::ID) -> Result<T, Self::Error>;
    async fn create_children(&self, entities: Vec<T>, parent_id: &R::ID) -> Result<Vec<T>, Self::Error>;
    async fn delete_children(&self, parent_id: &R::ID) -> Result<(), Self::Error>;

    // Many-to-Many relationship methods
    async fn find_associated(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
    async fn find_associated_to(
        &self,
        entity_id: &T::ID,
        page: PageRequest,
    ) -> Result<PageResponse<R>, Self::Error>;
    async fn count_associated(&self, related_id: &R::ID) -> Result<u64, Self::Error>;
    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<(), Self::Error>;
    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<(), Self::Error>;
    async fn create_associated(&self, entity: T, related_id: &R::ID) -> Result<T, Self::Error>;
    async fn is_associated(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<bool, Self::Error>;
    async fn dissociate_all(&self, entity_id: &T::ID) -> Result<u64, Self::Error>;
    async fn dissociate_from_all(&self, related_id: &R::ID) -> Result<u64, Self::Error>;
}

#[async_trait]
pub trait ThroughputRepository<T: Entity>: Repository<T> {
    async fn stream(&self) -> Result<impl Stream<Item = Result<T, Self::Error>>, Self::Error>;
}

#[async_trait]
pub trait LiveRepository<T: Entity>: Repository<T> {
    async fn live_single(&self, entity_id: &T::ID) -> Result<impl Stream<Item = Result<T, Self::Error>>, Self::Error>;
    async fn live_table(&self) -> Result<impl Stream<Item = Result<T, Self::Error>>, Self::Error>;
}

#[async_trait]
pub trait Database {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn ping(&self) -> Result<bool, Self::Error>;
}
