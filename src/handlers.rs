use crate::models::CounterResponse;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use std::sync::atomic::Ordering;

/// GET /api/counter - Returns current counter value
pub async fn get_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.load(Ordering::SeqCst);
    Json(CounterResponse::new(count))
}

/// POST /api/counter/increment - Increments counter and returns new value
pub async fn increment_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.fetch_add(1, Ordering::SeqCst) + 1;
    Json(CounterResponse::new(count))
}

/// POST /api/counter/decrement - Decrements counter (minimum 0) and returns new value
pub async fn decrement_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    // Use compare_exchange loop to ensure we don't go below 0
    loop {
        let current = state.counter.load(Ordering::SeqCst);
        if current == 0 {
            return Json(CounterResponse::new(0));
        }

        match state.counter.compare_exchange(
            current,
            current - 1,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(_) => return Json(CounterResponse::new(current - 1)),
            Err(_) => continue, // Retry if another thread modified the counter
        }
    }
}
