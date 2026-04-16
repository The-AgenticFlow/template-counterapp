use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

/// Shared application state with thread-safe counter
#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u64>>,
}

/// JSON response structure for counter endpoint
#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

/// Handler: GET /api/counter - Returns current count
async fn get_counter(State(state): State<AppState>) -> impl IntoResponse {
    let count = *state.counter.lock().unwrap();
    (StatusCode::OK, Json(CounterResponse { count }))
}

/// Handler: POST /api/counter/increment - Increments count by 1
async fn increment_counter(State(state): State<AppState>) -> impl IntoResponse {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    let count = *counter;
    (StatusCode::OK, Json(CounterResponse { count }))
}

/// Handler: POST /api/counter/decrement - Decrements count by 1 (min 0)
async fn decrement_counter(State(state): State<AppState>) -> impl IntoResponse {
    let mut counter = state.counter.lock().unwrap();
    *counter = counter.saturating_sub(1);
    let count = *counter;
    (StatusCode::OK, Json(CounterResponse { count }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Initialize shared state with counter starting at 0
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    // Configure CORS for frontend access
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with routes and middleware
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(state);

    // Bind to port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Counter API server running on http://0.0.0.0:3001");

    // Run the server
    axum::serve(listener, app).await.unwrap();
}
