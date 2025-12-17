use crate::{
    embeddder::Embedder,
    models::{ChatRequest, ChatResponse, ConversationMessage},
};
use anyhow::{Context, Result};
use lancor::{ChatCompletionRequest, LlamaCppClient, Message};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ConversationManager {
    pool: PgPool,
    client: LlamaCppClient,
    embedder: Embedder,
    model: String,
    system_prompt: String,
    top_k: usize,
    temperature: f32,
    max_tokens: u32,
}

impl ConversationManager {
    pub fn new(
        pool: PgPool,
        client: LlamaCppClient,
        embedder: Embedder,
        model: String,
        system_prompt: String,
        top_k: usize,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        Self {
            pool,
            client,
            embedder,
            model,
            system_prompt,
            top_k,
            temperature,
            max_tokens,
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let conversation_id = request
            .conversation_id
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // Get relevant context from vector store
        let query_embedding = self.embedder.embed(&request.message).await?;
    }
}
