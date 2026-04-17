use axum::{
    extract::State,
    http::{header, HeaderValue, Method},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

/// Response structure for counter endpoints
#[derive(Serialize)]
struct CountResponse {
    count: i64,
}

/// Shared application state
#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i64>>,
}

/// GET /api/counter - Returns current counter value
async fn get_counter(State(state): State<AppState>) -> Json<CountResponse> {
    let count = *state.counter.lock().unwrap();
    Json(CountResponse { count })
}

/// POST /api/counter/increment - Increments counter by 1
async fn increment_counter(State(state): State<AppState>) -> Json<CountResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    Json(CountResponse { count: *counter })
}

/// POST /api/counter/decrement - Decrements counter by 1, never below 0
async fn decrement_counter(State(state): State<AppState>) -> Json<CountResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter = (*counter - 1).max(0);
    Json(CountResponse { count: *counter })
}

#[tokio::main]
async fn main() {
    // Initialize shared state with counter starting at 0
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    // Configure CORS to allow any origin (for frontend access)
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("*"))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    // Build router with routes and state
    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(state);

    // Start server on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Counter API server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
