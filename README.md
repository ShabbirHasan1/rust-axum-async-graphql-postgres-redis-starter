# rust-axum-async-graphql-postgres-redis-starter

A modern Rust backend starter template using:

- **Axum** for HTTP/WebSocket server
- **Async-GraphQL** for GraphQL APIs
- **PostgreSQL** via `tokio-postgres` and `bb8` async connection pool
- **Redis** via `bb8-redis` async connection pool
- **Firebase Auth** integration
- **WebSocket** support
- **Jemalloc** for improved memory performance

## Requirements

- Rust 1.85+ (2024 edition)
- PostgreSQL server
- Redis server

## Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/your-username/rust-axum-async-graphql-postgres-redis-starter.git
cd rust-axum-async-graphql-postgres-redis-starter
```

### 2. Setup environment

Create a `.env` file in the project root:

```
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=postgres

REDIS_HOST=redis://localhost:6379

FIREBASE_PROJECT_ID=firebase_project
ADMIN_SECRET=admin1234
```

### 3. Run the project

```bash
cargo run
```

### 4. Access the GraphQL Playground

Once running, open: [http://localhost:8000/graphql](http://localhost:8000/v1/graphql)

## Performance

This project uses `jemallocator` as the global allocator for improved memory allocation performance in high-throughput environments and simd-json for fast JSON operations.

```rust
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

## License

MIT
