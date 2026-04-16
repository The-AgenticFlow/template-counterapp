use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u64>>,
}

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

async fn get_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = *state.counter.lock().unwrap();
    Json(CounterResponse { count })
}

async fn increment_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    Json(CounterResponse { count: *counter })
}

async fn decrement_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let mut counter = state.counter.lock().unwrap();
    *counter = counter.saturating_sub(1);
    Json(CounterResponse { count: *counter })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Counter API listening on http://0.0.0.0:3001");
    
    axum::serve(listener, app).await.unwrap();
}
