use crate::{
    dto::avatar::Avatar,
    prelude::*,
    repository::{
        traits::{Entity, FieldFindableRepository},
        RepositoryProvider,
    },
};

use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::model::user::User;

pub struct UserService {
    repository: Data<RepositoryProvider>,
}

impl UserService {
    pub fn from(repository: Data<RepositoryProvider>) -> Result<Self> {
        Ok(Self { repository })
    }

    pub async fn get_user(&self, id: &String) -> Result<Option<User>> {
        self.repository.user_repo().find(id).await
    }

    pub async fn update_user(
        &self,
        mut user: User,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        avatar: Option<String>,
    ) -> Result<User> {
        if let Some(username) = username {
            user.set_username(username);
        }
        if let Some(email) = email {
            user.set_email(email);
        }
        if let Some(password) = password {
            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)?
                .to_string();
            user.set_password(password_hash);
        }
        if let Some(avatar) = avatar {
            user.set_avatar(avatar);
        }
        self.repository.user_repo().save(user).await
    }

    pub async fn delete_user(&self, user: User) -> Result<()> {
        self.repository
            .user_repo()
            .delete(&user.id().unwrap())
            .await
    }
}
