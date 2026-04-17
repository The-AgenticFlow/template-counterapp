//! Handler functions for counter endpoints
//!
//! These handlers will be implemented in Segment 3.

use crate::models::CounterResponse;
use crate::CounterState;
use axum::extract::State;
use axum::Json;

/// GET /api/counter - Returns the current counter value
pub async fn get_counter(
    State(counter): State<CounterState>,
) -> Json<CounterResponse> {
    let count = *counter.lock().unwrap();
    Json(CounterResponse { count })
}

/// POST /api/counter/increment - Increments the counter and returns new value
pub async fn increment_counter(
    State(counter): State<CounterState>,
) -> Json<CounterResponse> {
    let mut count = counter.lock().unwrap();
    *count += 1;
    Json(CounterResponse { count: *count })
}

/// POST /api/counter/decrement - Decrements the counter (minimum 0) and returns new value
pub async fn decrement_counter(
    State(counter): State<CounterState>,
) -> Json<CounterResponse> {
    let mut count = counter.lock().unwrap();
    *count = (*count - 1).max(0);
    Json(CounterResponse { count: *count })
}
