use crate::driver::id::id;
use crate::prelude::*;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash};
use chrono::{DateTime, Utc};
use diesel::prelude::Queryable;
use diesel::{Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl User {
    pub fn new_registered(username: String, email: String, password: String) -> Result<Self> {
        Ok(User {
            id: id!(),
            username,
            email,
            password: Argon2::default()
                .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))?
                .to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        })
    }

    pub fn is_correct_password(&self, password: &str) -> Result<()> {
        Argon2::default()
            .verify_password(password.as_bytes(), &PasswordHash::new(&self.password)?)
            .map_err(|_| Error::Unauthorized("Wrong email or password".into()))
    }
}

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::avatars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Avatar {
    pub id: String,
    pub user_id: String,
}

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::auth_options)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthOptions {
    pub id: String,
    pub user_id: String,
    pub two_factor_encoded: Option<String>,
}

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::deleted_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeletedUser {
    pub id: String,
    pub deleted_at: DateTime<Utc>,
}
