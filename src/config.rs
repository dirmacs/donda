use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub llm_server_url: String,
    pub llm_api_key: Option<String>,
    pub llm_model: String,
    pub embedding_model: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            llm_server_url: env::var("LLM_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            llm_api_key: env::var("LLM_API_KEY").ok(),
            llm_model: env::var("LLM_MODEL")
                .unwrap_or_else(|_| "default".to_string()),
            embedding_model: env::var("EMBEDDING_MODEL")
                .unwrap_or_else(|_| "default".to_string()),
            chunk_size: env::var("CHUNK_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            chunk_overlap: env::var("CHUNK_OVERLAP")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(200),
            top_k: env::var("TOP_K")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            temperature: env::var("TEMPERATURE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.7),
            max_tokens: env::var("MAX_TOKENS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2000),
            system_prompt: env::var("SYSTEM_PROMPT").unwrap_or_else(|_| {
                "You are a helpful assistant. Use the provided context to answer questions accurately.".to_string()
            }),
        })
    }
}
