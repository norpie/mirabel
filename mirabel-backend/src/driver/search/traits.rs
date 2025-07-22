use crate::prelude::*;

use async_trait::async_trait;

use super::models::SearchPage;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    async fn available(&self) -> bool;
    async fn search(&self, query: String, page: i32) -> Result<SearchPage>;
    fn name(&self) -> String;
}
