use crate::utils::id::id;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use chrono::DateTime;
use chrono::Utc;
use diesel::Insertable;
use diesel::Selectable;
use diesel::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;

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
    pub fn new_registered(
        username: String,
        email: String,
        password: String,
    ) -> crate::Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| crate::Error::Hash(e.to_string()))?
            .to_string();

        Ok(User {
            id: id!(),
            username,
            email,
            password: password_hash,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        })
    }

    pub fn is_correct_password(&self, password: &str) -> crate::Result<()> {
        let parsed_hash =
            PasswordHash::new(&self.password).map_err(|e| crate::Error::Hash(e.to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| crate::Error::PasswordVerification)
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
