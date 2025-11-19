use crate::{
    chunker::Chunker,
    embedder::Embedder,
    models::{Chunk, Document},
};
use anyhow::{Context, Result};
use sqlx::PgPool;
use std::{collections::HashMap, fs, path::Path};

pub struct Ingestor {
    pool: PgPool,
    chunker: Chunker,
    embedder: Embedder,
}

impl Ingestor {
    pub fn new(pool: PgPool, chunker: Chunker, embedder: Embedder) -> Self {
        Self {
            pool,
            chunker,
            embedder,
        }
    }

    pub async fn ingest_file(&self, path: &Path) -> Result<()> {
        tracing::info!("Ingesting file: {:?}", path);

        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))?;

        let document = Document {
            id: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            content,
            metadata: HashMap::new(),
        };

        let chunks = self.chunker.chunk(document)?;
        tracing::info!("Created {} chunks", chunks.len());

        for chunk in chunks {
            let embedding = self.embedder.embed(&chunk.text).await?;

            sqlx::query!(
                r#"
                INSERT INTO chunks (text, source, index, metadata, embedding)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                chunk.text,
                chunk.source,
                chunk.index,
                chunk.metadata,
                &embedding as &[f32],
            )
            .execute(&self.pool)
            .await?;
        }

        tracing::info!("Successfully ingested file: {:?}", path);
        Ok(())
    }
}
