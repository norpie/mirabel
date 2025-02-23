use crate::{dto::avatar::Avatar, prelude::*};

use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::repository::avatars::AvatarRepository;

use super::SurrealDB;

#[derive(Debug, Deserialize)]
pub struct SurrealDBAvatar {
    pub id: Thing,
    pub avatar: String,
}

#[async_trait]
impl AvatarRepository for SurrealDB {
    /// Delete workspace avatar.
    async fn delete_avatar(&self, table: String, id: String) -> Result<String> {
        Ok(self
            .0
            .query("DELETE (SELECT -> has_avatar -> avatar as avatars FROM $item FETCH avatars)[0].avatars[0] RETURN BEFORE;")
            .bind(("item", Thing::from((table.as_str(), id.as_str()))))
            .await?
            .take::<Option<SurrealDBAvatar>>(0)?
            .ok_or(Error::NotFound(format!("Avatar on {table}:{id}")))?.avatar)
    }

    /// Set workspace avatar url.
    async fn set_avatar(&self, table: String, id: String, avatar_url: String) -> Result<String> {
        let avatar: SurrealDBAvatar = self
            .0
            .create("avatar")
            .content(Avatar { path: avatar_url })
            .await?
            .ok_or(Error::NotFound("new avatar".into()))?;

        self.0
            .query("RELATE $item -> has_avatar -> $avatar")
            .bind(("item", Thing::from((table, id))))
            .bind(("avatar", avatar.id))
            .await?;
        Ok(avatar.avatar)
    }

    /// Get workspace avatar url.
    async fn get_avatar(&self, table: String, id: String) -> Result<Option<String>> {
        Ok(self.0.query("SELECT * FROM (SELECT ->has_avatar->avatar as avatars FROM $item FETCH avatars)[0].avatars[0]")
            .bind(("item", Thing::from((table, id))))
            .await?
            .take::<Option<SurrealDBAvatar>>(0)?
            .map(|su| su.avatar))
    }
}
