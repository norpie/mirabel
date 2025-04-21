use crate::{dto::avatar::Avatar, prelude::*, repository::traits::FieldFindableRepository};

use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::model::user::User;

pub struct UserService {
    user_repo: Box<dyn FieldFindableRepository<User>>,
}

impl UserService {
    pub async fn get_user(&self, id: &String) -> Result<Option<User>> {
        self.user_repo.find(&id).await
    }
}
