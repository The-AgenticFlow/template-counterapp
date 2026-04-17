use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

/// Counter state shared across all handlers
type CounterState = Arc<Mutex<i32>>;

/// Response structure for all counter endpoints
#[derive(Debug, Serialize, Deserialize)]
struct CounterResponse {
    count: i32,
}

/// GET /api/counter - Returns current counter value
async fn get_counter(State(counter): State<CounterState>) -> Json<CounterResponse> {
    let count = *counter.lock().await;
    info!(count, "GET /api/counter");
    Json(CounterResponse { count })
}

/// POST /api/counter/increment - Increments counter and returns new value
async fn increment_counter(State(counter): State<CounterState>) -> Json<CounterResponse> {
    let mut count = counter.lock().await;
    *count += 1;
    let new_count = *count;
    info!(count = new_count, "POST /api/counter/increment");
    Json(CounterResponse { count: new_count })
}

/// POST /api/counter/decrement - Decrements counter (min 0) and returns new value
async fn decrement_counter(State(counter): State<CounterState>) -> Json<CounterResponse> {
    let mut count = counter.lock().await;
    *count = (*count - 1).max(0); // Ensure count never goes below 0
    let new_count = *count;
    info!(count = new_count, "POST /api/counter/decrement");
    Json(CounterResponse { count: new_count })
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    // Initialize counter state at 0
    let counter_state: CounterState = Arc::new(Mutex::new(0));

    // Configure CORS for frontend access
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // Build router with all routes
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(counter_state);

    // Bind to port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    info!("Counter API server running on http://0.0.0.0:3001");

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
