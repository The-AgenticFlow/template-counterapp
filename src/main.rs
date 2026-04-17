//! Counter API Backend
//!
//! A simple REST API for a counter with shared state.

mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

/// Shared counter state wrapped in Arc<Mutex<>> for thread-safe access
pub type CounterState = Arc<Mutex<i32>>;

#[tokio::main]
async fn main() {
    // Initialize counter at 0
    let counter: CounterState = Arc::new(Mutex::new(0));

    // Configure CORS to allow all origins, methods, and headers
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router with routes and shared state
    let app = Router::new()
        .route("/api/counter", get(handlers::get_counter))
        .route("/api/counter/increment", post(handlers::increment_counter))
        .route("/api/counter/decrement", post(handlers::decrement_counter))
        .layer(cors)
        .with_state(counter);

    // Bind to port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("Counter API running on http://localhost:3001");
    println!("Endpoints:");
    println!("  GET  /api/counter           - Get current count");
    println!("  POST /api/counter/increment - Increment count");
    println!("  POST /api/counter/decrement - Decrement count (min 0)");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
