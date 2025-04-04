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
    fn filterable_fields() -> Vec<String>;
}

pub trait FieldSearchableStruct {
    fn searchable_fields() -> Vec<String>;
}

pub trait FieldSortableStruct {
    fn sortable_fields() -> Vec<String>;
}

pub trait Entity: NamedStruct + Send + Sync + Serialize + DeserializeOwned {
    type ID: Clone + Eq + Display;
    fn id(&self) -> Option<&Self::ID>;
}

#[async_trait]
pub trait Repository<T: Entity> {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn find(&self, id: &T::ID) -> Result<Option<T>, Self::Error>;
    async fn save(&self, entity: &T) -> Result<T, Self::Error>;
    async fn delete(&self, id: &T::ID) -> Result<(), Self::Error>;
    async fn exists(&self, id: &T::ID) -> Result<bool, Self::Error>;
    async fn count(&self) -> Result<i64, Self::Error>;
}

#[async_trait]
pub trait SoftDeletableRepository<T: Entity>: Repository<T> {
    async fn soft_delete(&self, id: &T::ID) -> Result<(), Self::Error>;
    async fn restore(&self, id: &T::ID) -> Result<(), Self::Error>;
    async fn exists_deleted(&self, id: &T::ID) -> Result<bool, Self::Error>;
}

#[async_trait]
pub trait SearchableRepository<T: Entity + FieldSearchableStruct>: Repository<T> {
    async fn search(&self, query: &str, page: PageRequest) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait FieldFindableRepository<T: Entity + FieldFindableStruct>: Repository<T> {
    async fn find_by_field(
        &self,
        field: &str,
        value: &str,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait PublicEntityRepository<T: Entity>: Repository<T> {
    async fn find_all(&self, page: PageRequest) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait OneToManyRepository<T: Entity, R: Entity>: Repository<T> {
    async fn find_by_owner(
        &self,
        owner_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
}

#[async_trait]
pub trait OneToOneRepository<T: Entity, R: Entity>: Repository<T> {
    async fn find_by_related(&self, related_id: &R::ID) -> Result<Option<T>, Self::Error>;
    async fn exists_by_related(&self, related_id: &R::ID) -> Result<bool, Self::Error>;
}

#[async_trait]
pub trait ManyToManyRepository<T: Entity, R: Entity>: Repository<T> {
    async fn find_by_related(
        &self,
        related_id: &R::ID,
        page: PageRequest,
    ) -> Result<PageResponse<T>, Self::Error>;
    async fn associate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<(), Self::Error>;
    async fn dissociate(&self, entity_id: &T::ID, related_id: &R::ID) -> Result<(), Self::Error>;
    async fn is_associated(
        &self,
        entity_id: &T::ID,
        related_id: &R::ID,
    ) -> Result<bool, Self::Error>;
}

#[async_trait]
pub trait ThroughputRepository<T: Entity>: Repository<T> {
    async fn stream(&self) -> Result<impl Stream<Item = Result<T, Self::Error>>, Self::Error>;
}

#[async_trait]
pub trait LiveRepository<T: Entity>: Repository<T> {
    async fn live(&self) -> Result<impl Stream<Item = Result<T, Self::Error>>, Self::Error>;
}

#[async_trait]
pub trait Database {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn ping(&self) -> Result<bool, Self::Error>;
}
