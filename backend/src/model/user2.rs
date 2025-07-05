use crate::prelude::*;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash};
use chrono::{DateTime, Utc};
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(table = "users")]
pub struct User {
    #[welds(primary_key)]
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

    pub fn is_correct_password(&self, password: &str) -> Result<bool> {
        Argon2::default()
            .verify_password(password.as_bytes(), &PasswordHash::new(&self.password)?)
            .map(|_| true)
            .map_err(|_| Error::Unauthorized("Wrong email or password".into()))
    }
}

#[derive(Debug, WeldsModel)]
#[welds(table = "avatars")]
#[welds(BelongsToOne(username, User, "user_id"))]
pub struct Avatar {
    #[welds(primary_key)]
    pub id: String,
    pub user_id: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "auth_options")]
#[welds(BelongsToOne(username, User, "user_id"))]
pub struct AuthOptions {
    #[welds(primary_key)]
    pub id: String,
    pub user_id: String,
    pub two_factor_encoded: Option<String>,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "deleted_users")]
#[welds(BelongsToOne(username, User, "user_id"))]
pub struct DeletedUser {
    #[welds(primary_key)]
    pub id: String,
    pub user_id: String,
    pub deleted_at: DateTime<Utc>,
}
