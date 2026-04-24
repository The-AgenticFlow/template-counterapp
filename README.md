# Counter App

A dual-language Counter API implementation with CI/CD pipeline support.

[![CI](https://github.com/The-Agentic-Flow/template-counterapp/actions/workflows/ci.yml/badge.svg)](https://github.com/The-Agentic-Flow/template-counterapp/actions/workflows/ci.yml)

## Overview

This project provides a Counter API with implementations in both Node.js and Rust. It serves as a template for demonstrating CI/CD pipeline best practices with dual-language support.

## Features

- **Dual Implementation**: Same API functionality in both Node.js and Rust
- **CI/CD Pipeline**: GitHub Actions workflow with comprehensive testing
- **Unit & Integration Testing**: Both languages have full test coverage
- **Code Quality**: Automated linting with clippy for Rust

## Project Structure

```
├── src/
│   ├── index.js          # Node.js Counter implementation
│   └── main.rs           # Rust Axum-based HTTP API
├── tests/
│   └── api_integration_tests.rs  # Rust integration tests
├── test/
│   └── index.test.js     # Node.js tests
├── .github/workflows/
│   └── ci.yml            # CI/CD pipeline configuration
├── Cargo.toml            # Rust dependencies
└── package.json          # Node.js dependencies
```

## Implementations

### Node.js
Simple Counter class with increment, decrement, and reset operations.

**Location**: `src/index.js`

**API**:
```javascript
const { Counter } = require('./src/index');
const counter = new Counter(0);
counter.increment();  // returns 1
counter.decrement();  // returns 0
counter.reset();      // returns 0
```

### Rust
Axum-based HTTP server providing REST API endpoints.

**Location**: `src/main.rs`

**Endpoints**:
- `GET /api/counter` - Get current count
- `POST /api/counter/increment` - Increment count
- `POST /api/counter/decrement` - Decrement count (minimum 0)

**Example**:
```bash
# Start the server
cargo run --release

# Get current count
curl http://localhost:3001/api/counter

# Increment
curl -X POST http://localhost:3001/api/counter/increment

# Decrement
curl -X POST http://localhost:3001/api/counter/decrement
```

## Build Instructions

### Prerequisites
- Node.js >= 20.0.0
- Rust toolchain (latest stable)

### Node.js Build

```bash
# Install dependencies
npm ci

# Build (runs the Counter demo)
npm run build

# Run tests
npm test
```

### Rust Build

```bash
# Build release version
cargo build --release

# Build debug version
cargo build

# Run the server
cargo run --release

# Run unit tests
cargo test

# Run integration tests
cargo test --test api_integration_tests

# Run clippy (linting)
cargo clippy -- -D warnings
```

## CI/CD Pipeline

The GitHub Actions workflow (`ci.yml`) provides comprehensive testing for both implementations.

### Workflow Jobs

| Job | Description | Dependencies |
|-----|-------------|--------------|
| `node-build` | Builds Node.js application | - |
| `node-test` | Runs Node.js tests | node-build |
| `rust-build` | Builds Rust application | - |
| `rust-test` | Runs Rust unit tests | rust-build |
| `rust-clippy` | Lints Rust code | rust-build |
| `rust-integration-test` | Runs API integration tests | rust-test |

### Features

- **Concurrency Control**: Redundant workflow runs are cancelled automatically
- **Caching**: 
  - Node.js dependencies cached via `actions/setup-node`
  - Cargo dependencies cached via `actions/cache@v4`
- **Timeouts**: All jobs have appropriate timeouts (10-15 minutes)
- **Fail-Fast**: Pipeline fails on first error to save time
- **Parallel Execution**: Independent jobs run in parallel

### Environment Variables

- `CARGO_TERM_COLOR: always` - Enables colored Cargo output
- `RUST_BACKTRACE: 1` - Enables Rust backtraces for debugging

## Testing

### Node.js Tests
Node.js tests use the built-in Node.js test runner:

```bash
npm test
```

Tests cover:
- Counter initialization (default and custom values)
- Increment/decrement operations
- Reset functionality
- Chained operations

### Rust Tests

#### Unit Tests
Located in `src/main.rs` within `#[cfg(test)]` module:

```bash
cargo test
```

Tests cover:
- Counter initialization
- Increment/decrement operations
- Boundary conditions (not going below zero)
- Concurrent operations

#### Integration Tests
Located in `tests/api_integration_tests.rs`:

```bash
cargo test --test api_integration_tests
```

Tests cover:
- API endpoints via HTTP requests
- CORS header validation
- Full workflow scenarios
- Error handling

## Development

### Adding New Features

1. Implement in both languages when applicable
2. Add unit tests for new functionality
3. Update integration tests if API changes
4. Ensure all CI checks pass

### Code Quality

- **Rust**: Clippy is enforced with zero warnings (`-D warnings`)
- **Node.js**: Follow standard JavaScript practices

## License

MIT

## Contributing

This is a template project for CI/CD demonstrations. Feel free to fork and adapt for your own use.
