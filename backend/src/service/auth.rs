
use actix_web::web::Data;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};

use crate::dto::login_user::LoginUser;
use crate::dto::register_user::RegisterUser;
use crate::model::user::User;
use crate::prelude::*;

use crate::repository::traits::Entity;
use crate::repository::RepositoryProvider;
use crate::security::jwt_util::TokenFactory;
use crate::security::jwt_util::TokenPair;

pub struct AuthService {
    repository: Data<RepositoryProvider>,
    token_factory: TokenFactory,
}

impl AuthService {
    pub fn from(repo: Data<RepositoryProvider>) -> Result<Self> {
        Ok(AuthService {
            repository: repo,
            token_factory: TokenFactory::from_env()?,
        })
    }

    pub async fn login(&self, user: LoginUser) -> Result<TokenPair> {
        let fields = vec![("email", user.email)];
        let found_user: User = self
            .repository
            .user_repo()
            .find_single_by_fields(fields)
            .await?
            .ok_or(Error::BadRequest("Wrong email or password".into()))?;
        let password_hash = PasswordHash::new(&found_user.password)?;
        Argon2::default()
            .verify_password(user.password.as_bytes(), &password_hash)
            .map_err(|_| Error::BadRequest("Wrong email or password".into()))?;
        self.token_factory
            .generate_token(found_user.id().unwrap().to_string())
    }

    pub async fn register(&self, user: RegisterUser) -> Result<TokenPair> {
        let RegisterUser {
            username,
            email,
            password,
        } = user;

        let fields = vec![("email", email.clone()), ("username", username.clone())];

        if self.repository.user_repo().exists_by_fields(fields).await? {
            return Err(Error::BadRequest("Email already exists".into()));
        }

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let user = self
            .repository
            .user_repo()
            .save(User::new(
                username,
                email,
                password_hash,
            ))
            .await?;
        self.token_factory
            .generate_token(user.id().unwrap().to_string())
    }

    pub fn refresh(&self, token: String) -> Result<TokenPair> {
        let subject = self.token_factory.subject(&token)?;
        self.token_factory.generate_token(subject)
    }
}
