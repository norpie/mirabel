use async_trait::async_trait;

use crate::prelude::*;

#[async_trait]
pub trait AvatarRepository {
    /// Delete workspace avatar.
    async fn delete_avatar(&self, table: String, id: String) -> Result<String>;

    /// Set workspace avatar url.
    async fn set_avatar(&self, table: String, id: String, avatar_url: String) -> Result<String>;

    /// Get workspace avatar url.
    async fn get_avatar(&self, table: String, id: String) -> Result<Option<String>>;
}
