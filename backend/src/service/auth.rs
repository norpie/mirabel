use std::sync::Arc;

use actix_web::web::Data;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};

use crate::model::user::NewUser;
use crate::prelude::*;

use crate::repository::users::UserRepository;
use crate::repository::Repository;
use crate::security::jwt_util::TokenFactory;
use crate::{
    model::user::AuthUser, repository::surrealdb::SurrealDB, security::jwt_util::TokenPair,
};

pub async fn register(db: Data<Box<dyn Repository>>, user: AuthUser) -> Result<TokenPair> {
    let AuthUser { email, password } = user;

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let user = db.create_user(NewUser::new(email, password_hash)).await?;
    TokenFactory::from_env()?.generate_token(user.id().to_string())
}

pub async fn login(db: Data<Box<dyn Repository>>, user: AuthUser) -> Result<TokenPair> {
    let AuthUser { email, password } = user;
    let found_user = db
        .get_user_by_email(email)
        .await?
        .ok_or(Error::Unauthorized("Missing email".into()))?;

    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(found_user.password())?;
    Argon2::default().verify_password(password.as_bytes(), &password_hash)?;

    TokenFactory::from_env()?.generate_token(found_user.id().to_string())
}
