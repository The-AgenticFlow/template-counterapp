use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

// Shared application state
struct AppState {
    counter: Mutex<u64>,
}

// JSON response structure
#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create shared state with initial count of 0
    let state = Arc::new(AppState {
        counter: Mutex::new(0),
    });

    // Configure CORS for frontend access
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(state);

    // Run the server on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    info!("Counter API server listening on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}

// GET /api/counter - Returns current count
async fn get_counter(State(state): State<Arc<AppState>>) -> Json<CounterResponse> {
    let count = state.counter.lock().unwrap();
    Json(CounterResponse { count: *count })
}

// POST /api/counter/increment - Increments count and returns new value
async fn increment_counter(State(state): State<Arc<AppState>>) -> Json<CounterResponse> {
    let mut count = state.counter.lock().unwrap();
    *count += 1;
    Json(CounterResponse { count: *count })
}

// POST /api/counter/decrement - Decrements count (min 0) and returns new value
async fn decrement_counter(State(state): State<Arc<AppState>>) -> Json<CounterResponse> {
    let mut count = state.counter.lock().unwrap();
    if *count > 0 {
        *count -= 1;
    }
    Json(CounterResponse { count: *count })
}