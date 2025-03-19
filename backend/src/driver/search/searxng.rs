use crate::prelude::*;

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use super::{SearchEngine, SearchPage, SearchResult};

#[derive(Default)]
pub struct SearxNG {
    base_url: String,
    client: Client,
}

impl SearxNG {
    pub fn new(base_url: &str) -> Self {
        SearxNG {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearxNGSearchPage {
    results: Vec<SearxNGSearchResult>,
}

#[derive(Debug, Deserialize)]
struct SearxNGSearchResult {
    url: String,
    title: String,
    content: String,
    engine: String,
}

impl From<SearxNGSearchResult> for SearchResult {
    fn from(result: SearxNGSearchResult) -> Self {
        SearchResult {
            url: result.url,
            title: result.title,
            summary: result.content,
            source: result.engine,
        }
    }
}

#[async_trait]
impl SearchEngine for SearxNG {
    async fn search(&self, query: String, page: i32) -> Result<SearchPage> {
        let url = format!(
            "{}/search?q={}&format=json&pageno={}",
            self.base_url, &query, page
        );

        let search_page: SearxNGSearchPage = self.client.get(&url).send().await?.json().await?;

        Ok(SearchPage {
            page,
            query,
            results: search_page.results.into_iter().map(|r| r.into()).collect(),
        })
    }

    async fn available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_search() {
        let searxng = SearxNG::new("http://localhost:8081");
        let search_page = searxng.search("rust".to_string(), 1).await.unwrap();
        assert_eq!(search_page.page, 1);
        assert_eq!(search_page.query, "rust");
        assert!(!search_page.results.is_empty());
        dbg!(&search_page);
    }
}
