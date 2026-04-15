use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct CounterState {
    count: Arc<AtomicU64>,
}

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

async fn get_counter(State(state): State<CounterState>) -> Json<CounterResponse> {
    let count = state.count.load(Ordering::Relaxed);
    Json(CounterResponse { count })
}

async fn increment_counter(State(state): State<CounterState>) -> Json<CounterResponse> {
    let count = state.count.fetch_add(1, Ordering::Relaxed) + 1;
    Json(CounterResponse { count })
}

async fn decrement_counter(State(state): State<CounterState>) -> Json<CounterResponse> {
    let mut current = state.count.load(Ordering::Relaxed);
    loop {
        if current == 0 {
            break;
        }
        match state.count.compare_exchange(
            current,
            current - 1,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                current = current - 1;
                break;
            }
            Err(actual) => current = actual,
        }
    }
    Json(CounterResponse { count: current })
}

#[tokio::main]
async fn main() {
    let state = CounterState {
        count: Arc::new(AtomicU64::new(0)),
    };

    let app = Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("Counter API server running on http://0.0.0.0:3001");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
