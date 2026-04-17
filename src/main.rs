mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// Thread-safe counter state using AtomicI32 wrapped in Arc for sharing
#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<AtomicI32>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicI32::new(0)),
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state = AppState::new();

    // Configure CORS for frontend access
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with all three endpoints
    let app = Router::new()
        .route("/api/counter", get(handlers::get_counter))
        .route("/api/counter/increment", post(handlers::increment_counter))
        .route("/api/counter/decrement", post(handlers::decrement_counter))
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
