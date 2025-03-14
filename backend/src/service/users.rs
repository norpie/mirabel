use crate::{dto::avatar::Avatar, prelude::*};

use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::{
    model::user::{UpdatedUser, User},
    repository::Repository,
};

pub async fn update(
    db: Data<Box<dyn Repository>>,
    user: User,
    mut updated_user: UpdatedUser,
) -> Result<User> {
    if let Some(password) = updated_user.password {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        updated_user.password = Some(password_hash);
    };
    db.update_user(user.id().to_string(), updated_user).await
}

pub async fn delete(db: Data<Box<dyn Repository>>, user: User) -> Result<User> {
    db.delete_user(user.id().to_string()).await
}

pub async fn avatar(db: Data<Box<dyn Repository>>, user: User) -> Result<Option<String>> {
    db.get_avatar("user".into(), user.id().into()).await
}
