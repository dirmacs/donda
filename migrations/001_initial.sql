-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- Chunks table with vector embeddings
CREATE TABLE IF NOT EXISTS chunks (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    source TEXT NOT NULL,
    index INTEGER NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    embedding vector(1024),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for similarity search
CREATE INDEX IF NOT EXISTS chunks_embedding_idx ON chunks
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- Conversation messages table
CREATE TABLE IF NOT EXISTS conversation_messages (
    id SERIAL PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for conversation queries
CREATE INDEX IF NOT EXISTS conversation_messages_conversation_id_idx
ON conversation_messages(conversation_id, created_at);
