use crate::prelude::*;

pub(crate) mod models {}

#[derive(Debug)]
pub struct Ollama<'a> {
    base_url: &'a str,
}

impl Default for Ollama<'_> {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434",
        }
    }
}

impl Ollama<'_> {
    pub async fn generate() {}
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
    async fn test_generate_stream() {}

    #[tokio::test]
    #[serial]
    async fn test_generate_single() {}

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
