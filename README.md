# donda

A Rust-based RAG (Retrieval-Augmented Generation) system with llama.cpp integration using the [`lancor`](https://github.com/dirmacs/lancor) crate.

## Features

- Document ingestion and chunking
- Vector embeddings using llama.cpp
- PostgreSQL with pgvector for similarity search
- Interactive chat with conversation history
- HTTP API server
- CLI interface

## Setup

1. Install PostgreSQL with pgvector extension
2. Copy `.env.example` to `.env` and configure
3. Run migrations: `cargo run -- db up`
4. Ingest documents: `cargo run -- ingest -s /path/to/docs`
5. Start server: `cargo run -- serve`
6. Or use interactive chat: `cargo run -- chat`

## Usage

```
# Database operations
donda db up
donda db status

# Ingest documents
donda ingest -s ./knowledge-base -m text

# Start HTTP server
donda serve -H 0.0.0.0 -p 7070

# Interactive chat
donda chat

# Show version
donda version
```

## API Endpoints

- `GET /health` - Health check
- `POST /chat` - Chat endpoint
  ```
  {
    "message": "Your question here",
    "conversation_id": "optional-conversation-id"
  }
  ```

## License

GPL-3.0
