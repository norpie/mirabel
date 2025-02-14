use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::dto::page::PageRequest;
use crate::model::user::{NewUser, UpdatedUser, User};
use crate::prelude::*;

use crate::repository::users::UserRepository;

use super::{SurrealDB, SurrealDbPagination};

#[derive(Debug, Clone, Deserialize)]
pub struct SurrealDBUser {
    id: Thing,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}

impl From<SurrealDBUser> for User {
    fn from(user: SurrealDBUser) -> Self {
        User::new(
            user.id.to_string(),
            user.email,
            user.password,
            user.created_at,
        )
    }
}

#[async_trait]
impl UserRepository for SurrealDB {
    /// Create a new user.
    async fn create_user(&self, user: NewUser) -> Result<User> {
        self.0
            .create("user")
            .content(user)
            .await?
            .ok_or(Error::NotFound("new user".into()))
            .map(|su: SurrealDBUser| su.into())
    }

    /// Retrieve a user by ID.
    async fn get_user_by_id(&self, id: String) -> Result<Option<User>> {
        Ok(self
            .0
            .select::<Option<SurrealDBUser>>(("user", id))
            .await?
            .map(|su| su.into()))
    }

    /// Retrieve a user by email.
    async fn get_user_by_email(&self, email: String) -> Result<Option<User>> {
        Ok(self
            .0
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email))
            .await?
            .take::<Option<SurrealDBUser>>(0)?
            .map(|su| su.into()))
    }

    /// Retrieve all users.
    async fn get_all_users(&self, page: PageRequest) -> Result<Vec<User>> {
        let pagination: SurrealDbPagination = page.into();
        Ok(self
            .0
            .query("SELECT * FROM user LIMIT $limit START $start")
            .bind(("limit", pagination.limit))
            .bind(("start", pagination.start))
            .await?
            .take::<Vec<SurrealDBUser>>(0)?
            .into_iter()
            .map(|su| su.into())
            .collect())
    }

    /// Update an existing user.
    async fn update_user(&self, id: String, user: UpdatedUser) -> Result<User> {
        self.0
            .update(("user", &id))
            .merge(user)
            .await?
            .ok_or(Error::NotFound(format!("Updated user: {}", &id)))
            .map(|su: SurrealDBUser| su.into())
    }

    /// Delete a user by ID.
    async fn delete_user(&self, id: String) -> Result<User> {
        self.0
            .delete(("user", &id))
            .await?
            .ok_or(Error::NotFound(format!("Deleted user: {}", &id)))
            .map(|su: SurrealDBUser| su.into())
    }
}
