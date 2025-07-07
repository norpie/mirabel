use crate::{model::user::User, prelude::*};

use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub struct UserService {
    repository: Data<Pool>,
}

impl UserService {
    pub fn from(repository: Data<Pool>) -> Result<Self> {
        Ok(Self { repository })
    }

    pub async fn get_user(&self, user_id: &String) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;
        let conn = self.repository.get().await?;
        let user_id = user_id.clone();
        Ok(conn.interact(move |conn| {
            users
                .filter(id.eq(user_id))
                .select(User::as_select())
                .first::<User>(conn)
                .optional()
        })
        .await??)
    }
}
