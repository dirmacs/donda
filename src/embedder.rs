use std::cell::LazyCell;

use anyhow::{Context, Result};
use lancor::{EmbeddingRequest, LlamaCppClient};

pub struct Embedder {
    client: LlamaCppClient,
    model: String,
}

impl Embedder {
    pub fn new(server_url: &str, api_key: Option<&str>, model: String) -> Result<Self> {
        let client = if let Some(key) = api_key {
            LlamaCppClient::with_api_key(server_url, key)?
        } else {
            LlamaCppClient::new(server_url)?
        };

        Ok(Self { client, model })
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let request = EmbeddingRequest::new(&self.model, text);
        let response = self
            .client
            .embedding(request)
            .await
            .context("Failed to generate embedding")?;

        Ok(response.data[0].embedding.clone())
    }

    pub async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }
}
