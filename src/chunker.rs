use crate::models::{Chunk, Document};
use anyhow::Result;
use std::collections::HashMap;

pub struct Chunker {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl Chunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }

    pub fn chunk(&self, doc: Document) -> Result<Vec<Chunk>> {
        let text = &doc.content;
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut index = 0;

        while start < text.len() {
            let end = (start + self.chunk_size).min(text.len());
            let chunk_text = text[start..end].to_string();

            chunks.push(Chunk {
                id: 0,
                text: chunk_text,
                source: doc.id.clone(),
                index,
                metadata: serde_json::to_value(&doc.metadata)?,
                embedding: None,
            });

            index += 1;
            start = if end >= text.len() {
                text.len()
            } else {
                end.saturating_sub(self.chunk_overlap)
            };
        }

        Ok(chunks)
    }
}
