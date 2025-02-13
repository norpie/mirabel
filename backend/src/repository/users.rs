use crate::{
    dto::page::PageRequest,
    model::user::{NewUser, UpdatedUser, User},
    prelude::*,
};

use async_trait::async_trait;

use super::Repository;

#[async_trait]
pub trait UserRepository: Repository {
    /// Create a new user.
    async fn create_user(&self, user: NewUser) -> Result<User>;

    /// Retrieve a user by ID.
    async fn get_user_by_id(&self, id: String) -> Result<Option<User>>;

    /// Retrieve all users.
    async fn get_all_users(&self, page: PageRequest) -> Result<Vec<User>>;

    /// Update an existing user.
    async fn update_user(&self, id: String, user: UpdatedUser) -> Result<User>;

    /// Delete a user by ID.
    async fn delete_user(&self, id: String) -> Result<()>;
}
