use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::dto::login_user::LoginUser;
use crate::dto::register_user::RegisterUser;
use crate::model::user::User;
use crate::prelude::*;

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

    pub async fn login(&self, login_user: LoginUser) -> Result<TokenPair> {
        use crate::schema::users::dsl::*;
        let conn = self.repository.get().await?;
        let found_user = conn
            .interact(|conn| {
                users
                    .filter(email.eq(login_user.email.clone()))
                    .or_filter(username.eq(login_user.email))
                    .first::<User>(conn)
                    .optional()
            })
            .await??;
        let Some(user) = found_user else {
            return Err(Error::Unauthorized("Wrong email or password".into()));
        };
        user.is_correct_password(&login_user.password)?;
        self.token_factory.generate_token(user.id.to_string())
    }

    pub async fn register(&self, register_user: RegisterUser) -> Result<TokenPair> {
        use crate::schema::users::dsl::*;
        let conn = self.repository.get().await?;

        // Check if user already exists
        let register_user_clone = register_user.clone();
        let found_user = conn
            .interact(|conn| {
                users
                    .filter(email.eq(register_user_clone.email))
                    .or_filter(username.eq(register_user_clone.username))
                    .first::<User>(conn)
                    .optional()
            })
            .await??;

        if found_user.is_some() {
            return Err(Error::Conflict("User already exists".into()));
        }

        let user = User::new_registered(
            register_user.username,
            register_user.email,
            register_user.password,
        )?;

        let user_id = user.id.clone();
        conn.interact(move |conn| diesel::insert_into(users).values(&user).execute(conn))
            .await??;

        self.token_factory.generate_token(user_id.to_string())
    }

    pub fn refresh(&self, token: String) -> Result<TokenPair> {
        self.token_factory
            .generate_token(self.token_factory.subject(&token)?)
    }
}
