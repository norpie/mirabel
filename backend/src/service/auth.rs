use std::sync::Arc;

use actix_web::web::Data;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};
use backend_derive::named_struct;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::dto::login_user::LoginUser;
use crate::dto::page::PageRequest;
use crate::dto::register_user::RegisterUser;
use crate::model::user::User;
use crate::prelude::*;

use crate::repository::traits::{Entity, FieldFindableRepository};
use crate::security::jwt_util::TokenFactory;
use crate::{repository::surrealdb::SurrealDB, security::jwt_util::TokenPair};

pub struct AuthService {
    user_repo: Box<dyn FieldFindableRepository<User, Error = Error>>,
    token_factory: TokenFactory,
}

impl AuthService {
    fn from(db: Box<dyn FieldFindableRepository<User, Error = Error>>) -> Result<Self> {
        Ok(AuthService {
            user_repo: db,
            token_factory: TokenFactory::from_env()?,
        })
    }

    pub async fn login(&self, user: LoginUser) -> Result<TokenPair> {
        let mut fields = vec![("email", user.email)];
        let found_user: User = self
            .user_repo
            .find_single_by_fields(fields)
            .await?
            .ok_or(Error::BadRequest("Wrong email or password".into()))?;
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(&found_user.password)?;
        Argon2::default()
            .verify_password(user.password.as_bytes(), &password_hash)
            .map_err(|_| Error::BadRequest("Wrong email or password".into()))?;
        self.token_factory
            .generate_token(found_user.id().unwrap().to_string())
    }

    pub async fn register(&self, user: RegisterUser) -> Result<TokenPair> {
        let RegisterUser {
            email,
            username,
            password,
        } = user;

        let fields = vec![("email", email.clone()), ("username", username.clone())];

        if self.user_repo.exists_by_fields(fields).await? {
            return Err(Error::BadRequest("Email already exists".into()));
        }

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let user = self
            .user_repo
            .save(User::new(
                email,
                username,
                password,
            ))
            .await?;
        self.token_factory
            .generate_token(user.id().unwrap().to_string())
    }
}
