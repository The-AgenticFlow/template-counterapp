# Counter API Backend

A simple REST API for a counter with shared state, built with Rust and Axum.

## Features

- **GET /api/counter** - Returns the current counter value (starts at 0)
- **POST /api/counter/increment** - Increments the counter and returns the new value
- **POST /api/counter/decrement** - Decrements the counter (minimum 0) and returns the new value
- **CORS enabled** - Allows cross-origin requests from any origin

## Requirements

- Rust 1.70+ (edition 2021)
- Cargo

## Running

```bash
cargo run --release
```

The server will start on `http://localhost:3001`.

## API Usage

### Get current counter value
```bash
curl http://localhost:3001/api/counter
# Response: {"count":0}
```

### Increment counter
```bash
curl -X POST http://localhost:3001/api/counter/increment
# Response: {"count":1}
```

### Decrement counter
```bash
curl -X POST http://localhost:3001/api/counter/decrement
# Response: {"count":0}
```

## Testing

Run the full test suite:

```bash
# Start the server
cargo run --release &

# Run tests
curl -s http://localhost:3001/api/counter          # {"count":0}
curl -s -X POST http://localhost:3001/api/counter/increment  # {"count":1}
curl -s -X POST http://localhost:3001/api/counter/increment  # {"count":2}
curl -s -X POST http://localhost:3001/api/counter/decrement  # {"count":1}
curl -s -X POST http://localhost:3001/api/counter/decrement  # {"count":0}
curl -s -X POST http://localhost:3001/api/counter/decrement  # {"count":0} (min 0)
```

## Architecture

- **src/main.rs** - Application entry point, router setup, CORS configuration
- **src/handlers.rs** - Handler functions for each endpoint
- **src/models.rs** - Response data structures

The counter state is shared across all requests using `Arc<Mutex<i32>>` for thread-safe access.
