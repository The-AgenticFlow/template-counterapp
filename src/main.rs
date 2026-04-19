use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// Shared counter state
#[derive(Debug)]
struct CounterState {
    count: AtomicU64,
}

// JSON response structure
#[derive(Debug, Serialize, Deserialize)]
struct CounterResponse {
    count: u64,
}

// GET /api/counter - returns current counter value
async fn get_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
    let count = state.count.load(Ordering::SeqCst);
    Json(CounterResponse { count })
}

// POST /api/counter/increment - increments counter
async fn increment_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
    let count = state.count.fetch_add(1, Ordering::SeqCst) + 1;
    Json(CounterResponse { count })
}

// POST /api/counter/decrement - decrements counter (minimum 0)
async fn decrement_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
    // Use a loop to ensure we don't go below 0
    loop {
        let current = state.count.load(Ordering::SeqCst);
        if current == 0 {
            return Json(CounterResponse { count: 0 });
        }
        // Try to decrement, but only if the value hasn't changed
        if state
            .count
            .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return Json(CounterResponse {
                count: current - 1,
            });
        }
        // If compare_exchange failed, another thread modified the counter, retry
    }
}

#[tokio::main]
async fn main() {
    // Initialize shared counter state starting at 0
    let counter_state = Arc::new(CounterState {
        count: AtomicU64::new(0),
    });

    // Configure CORS to allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // Build the router with all endpoints
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(counter_state);

    // Start server on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("Counter API server running on http://localhost:3001");
    println!("Endpoints:");
    println!("  GET  /api/counter            - Get current count");
    println!("  POST /api/counter/increment  - Increment count");
    println!("  POST /api/counter/decrement  - Decrement count (min 0)");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
