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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_counter_initialization() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });
        let response = get_counter(State(state)).await;
        assert_eq!(response.count, 0);
    }

    #[tokio::test]
    async fn test_increment_counter() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });
        let response = increment_counter(State(state.clone())).await;
        assert_eq!(response.count, 1);

        // Verify state was updated
        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 1);
    }

    #[tokio::test]
    async fn test_increment_multiple_times() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });

        // Increment 5 times
        for i in 1..=5 {
            let response = increment_counter(State(state.clone())).await;
            assert_eq!(response.count, i);
        }

        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 5);
    }

    #[tokio::test]
    async fn test_decrement_counter() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(5),
        });

        let response = decrement_counter(State(state.clone())).await;
        assert_eq!(response.count, 4);

        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 4);
    }

    #[tokio::test]
    async fn test_decrement_does_not_go_below_zero() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });

        // Try to decrement at 0
        let response = decrement_counter(State(state.clone())).await;
        assert_eq!(response.count, 0);

        // Verify still at 0
        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 0);
    }

    #[tokio::test]
    async fn test_decrement_to_zero_stops() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(2),
        });

        // Decrement twice
        let response1 = decrement_counter(State(state.clone())).await;
        assert_eq!(response1.count, 1);

        let response2 = decrement_counter(State(state.clone())).await;
        assert_eq!(response2.count, 0);

        // Try to decrement again at 0
        let response3 = decrement_counter(State(state.clone())).await;
        assert_eq!(response3.count, 0);

        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 0);
    }

    #[tokio::test]
    async fn test_increment_and_decrement_sequence() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(10),
        });

        // Increment
        let response = increment_counter(State(state.clone())).await;
        assert_eq!(response.count, 11);

        // Decrement twice
        let response = decrement_counter(State(state.clone())).await;
        assert_eq!(response.count, 10);

        let response = decrement_counter(State(state.clone())).await;
        assert_eq!(response.count, 9);

        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 9);
    }

    #[tokio::test]
    async fn test_concurrent_increments() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });

        let mut handles = vec![];

        // Spawn 10 concurrent increment tasks
        for _ in 0..10 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                let _ = increment_counter(State(state_clone)).await;
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify count is 10 (all increments applied)
        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 10);
    }

    #[tokio::test]
    async fn test_concurrent_decrements_from_zero() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(0),
        });

        let mut handles = vec![];

        // Spawn 5 concurrent decrement tasks (should all return 0)
        for _ in 0..5 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                decrement_counter(State(state_clone)).await
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        // All decrements from 0 should return 0
        for response in results {
            assert_eq!(response.count, 0);
        }

        // Counter should still be at 0
        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 0);
    }

    #[tokio::test]
    async fn test_concurrent_mixed_operations() {
        let state = Arc::new(CounterState {
            count: AtomicU64::new(5),
        });

        let mut handles = vec![];

        // Spawn 5 increment and 5 decrement tasks
        for i in 0..10 {
            let state_clone = state.clone();
            let handle = if i % 2 == 0 {
                tokio::spawn(async move {
                    increment_counter(State(state_clone)).await
                })
            } else {
                tokio::spawn(async move {
                    decrement_counter(State(state_clone)).await
                })
            };
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await.unwrap();
        }

        // Starting from 5, with 5 increments and 5 decrements,
        // we should still be at 5 (though order is non-deterministic)
        let current = state.count.load(Ordering::SeqCst);
        assert_eq!(current, 5);
    }
}
