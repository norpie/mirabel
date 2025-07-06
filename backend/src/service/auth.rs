use actix_web::web::Data;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash};
use deadpool_diesel::postgres::Pool;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::dto::login_user::LoginUser;
use crate::dto::register_user::RegisterUser;
use crate::model::user::User;
use crate::prelude::*;

use crate::repository::traits::Entity;
use crate::security::jwt_util::TokenFactory;
use crate::security::jwt_util::TokenPair;

pub struct AuthService {
    repository: Data<Pool>,
    token_factory: TokenFactory,
}

impl AuthService {
    pub fn from(repository: Data<Pool>) -> Result<Self> {
        Ok(AuthService {
            repository,
            token_factory: TokenFactory::from_env()?,
        })
    }

    pub async fn login(&self, user: LoginUser) -> Result<TokenPair> {
        use crate::schema::users::dsl::*;
        let found_user = users
            .filter(email.eq(user.email))
            .or_filter(username.eq(user.email))
            .first::<User>(&mut self.repository.get().await?.lock()?.into())
            .map_err(|_| Error::Unauthorized("Wrong email or password".into()))?;
        if !found_user.is_correct_password(&user.password)? {
            return Err(Error::Unauthorized("Wrong email or password".into()));
        }
        self.token_factory.generate_token(found_user.id.to_string())
    }

    pub async fn register(&self, register_user: RegisterUser) -> Result<TokenPair> {
        use crate::schema::users::dsl::*;
        let found_user = users
            .filter(email.eq(&register_user.email))
            .or_filter(username.eq(&register_user.username))
            .load::<User>(&mut self.repository.get().await?.lock()?.into())?
            .into_iter()
            .next();

        let user = User::new_registered(
            register_user.username,
            register_user.email,
            register_user.password,
        )?;

        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.repository.get().await?.lock()?.into())?;

        self.token_factory.generate_token(user.id)
    }

    pub fn refresh(&self, token: String) -> Result<TokenPair> {
        self.token_factory
            .generate_token(self.token_factory.subject(&token)?)
    }
}
