use futures::{Stream, TryStreamExt};
use models::{GenerateRequest, GenerateRequestInternal, GenerateResponse, StreamGenerateResponse};
use reqwest::{Client, Method};
use reqwest_streams::JsonStreamResponse;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::*;

pub(crate) mod models;

#[derive(Debug)]
pub struct Ollama<'a> {
    base_url: &'a str,
    client: Client,
}

impl Default for Ollama<'_> {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434",
            client: Client::new(),
        }
    }
}

impl Ollama<'_> {
    async fn request<T, U>(&self, method: Method, route: &str, body: T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        self.client
            .request(method, format!("{}/{}", self.base_url, route))
            .body(serde_json::to_string(&body)?)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        self.request::<GenerateRequestInternal, GenerateResponse>(
            Method::POST,
            "api/generate",
            request.into(),
        )
        .await
    }

    pub async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<impl Stream<Item = Result<StreamGenerateResponse>>> {
        let request_internal = GenerateRequestInternal::from(request).with_stream();
        Ok(self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .body(serde_json::to_string(&request_internal)?)
            .send()
            .await?
            .json_array_stream::<StreamGenerateResponse>(1024)
            .map_err(Into::into))
    }

    pub async fn generate_structured() {}
    pub async fn chat() {}
    pub async fn create() {}
    pub async fn blob_exists() {}
    pub async fn blob_push() {}
    pub async fn tags() {}
    pub async fn show() {}
    pub async fn copy() {}
    pub async fn delete() {}
    pub async fn pull() {}
    pub async fn push() {}
    pub async fn generate_embeddings() {}
    pub async fn ps() {}
    pub async fn generate_embedding() {}
    pub async fn version() {}
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_generate_single() {
        println!(
            "{}",
            Ollama::default()
                .generate(
                    GenerateRequest::new(("llama3.2", "1b").into(), "Hello, my name is".into())
                        .raw(true),
                )
                .await
                .unwrap()
                .response
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_generate_stream() {
        Ollama::default()
            .generate_stream(
                GenerateRequest::new(("llama3.2", "1b").into(), "Hello, my name is".into())
                    .raw(true),
            )
            .await
            .unwrap()
            .try_for_each(|response| async move {
                print!("{}", &response.response);
                Ok(())
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_chat() {}

    #[tokio::test]
    #[serial]
    async fn test_create() {}

    #[tokio::test]
    #[serial]
    async fn test_blob_exists() {}

    #[tokio::test]
    #[serial]
    async fn test_blob_push() {}

    #[tokio::test]
    #[serial]
    async fn test_tags() {}

    #[tokio::test]
    #[serial]
    async fn test_show() {}

    #[tokio::test]
    #[serial]
    async fn test_copy() {}

    #[tokio::test]
    #[serial]
    async fn test_delete() {}

    #[tokio::test]
    #[serial]
    async fn test_pull() {}

    #[tokio::test]
    #[serial]
    async fn test_push() {}

    #[tokio::test]
    #[serial]
    async fn test_generate_embeddings() {}

    #[tokio::test]
    #[serial]
    async fn test_ps() {}

    #[tokio::test]
    #[serial]
    async fn test_generate_embedding() {}

    #[tokio::test]
    #[serial]
    async fn test_version() {}
}
