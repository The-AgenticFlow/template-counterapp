use axum::{
    routing::{get, post},
    Router,
};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// Thread-safe counter state using AtomicI32
struct AppState {
    counter: AtomicI32,
}

impl AppState {
    fn new() -> Self {
        Self {
            counter: AtomicI32::new(0),
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state = Arc::new(AppState::new());

    // Configure CORS for frontend access
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with all three endpoints
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(state);

    // Bind server to port 3001
    let addr = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port 3001");

    println!("Counter API server running on http://{}", addr);
    println!("Endpoints:");
    println!("  GET  /api/counter");
    println!("  POST /api/counter/increment");
    println!("  POST /api/counter/decrement");

    // Start server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// Placeholder handlers (will be implemented in Segment 3)
async fn get_counter() -> String {
    r#"{"count":0}"#.to_string()
}

async fn increment_counter() -> String {
    r#"{"count":1}"#.to_string()
}

async fn decrement_counter() -> String {
    r#"{"count":0}"#.to_string()
}
