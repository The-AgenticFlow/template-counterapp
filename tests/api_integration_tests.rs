use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde::{Deserialize, Serialize};
use tower::util::ServiceExt; // for oneshot

#[derive(Debug, Serialize, Deserialize)]
struct CounterResponse {
    count: u64,
}

/// Create the test app with fresh counter state
fn create_test_app() -> Router {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use axum::extract::State;
    use axum::response::Json;
    use axum::routing::{get, post};
    use tower_http::cors::{Any, CorsLayer};
    use axum::http::Method;

    #[derive(Debug)]
    struct CounterState {
        count: AtomicU64,
    }

    async fn get_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
        let count = state.count.load(Ordering::SeqCst);
        Json(CounterResponse { count })
    }

    async fn increment_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
        let count = state.count.fetch_add(1, Ordering::SeqCst) + 1;
        Json(CounterResponse { count })
    }

    async fn decrement_counter(State(state): State<Arc<CounterState>>) -> Json<CounterResponse> {
        loop {
            let current = state.count.load(Ordering::SeqCst);
            if current == 0 {
                return Json(CounterResponse { count: 0 });
            }
            if state
                .count
                .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return Json(CounterResponse { count: current - 1 });
            }
        }
    }

    let counter_state = Arc::new(CounterState {
        count: AtomicU64::new(0),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/api/counter", get(get_counter))
        .route("/api/counter/increment", post(increment_counter))
        .route("/api/counter/decrement", post(decrement_counter))
        .layer(cors)
        .with_state(counter_state)
}

async fn send_request(app: &Router, method: &str, uri: &str) -> Response {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    app.clone().oneshot(request).await.unwrap()
}

async fn parse_response(response: Response) -> CounterResponse {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_api_get_counter_returns_zero_initially() {
    let app = create_test_app();

    let response = send_request(&app, "GET", "/api/counter").await;
    assert_eq!(response.status(), StatusCode::OK);

    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);
}

#[tokio::test]
async fn test_api_increment_returns_one() {
    let app = create_test_app();

    let response = send_request(&app, "POST", "/api/counter/increment").await;
    assert_eq!(response.status(), StatusCode::OK);

    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 1);
}

#[tokio::test]
async fn test_api_increment_multiple_times() {
    let app = create_test_app();

    // Increment 3 times
    for i in 1..=3 {
        let response = send_request(&app, "POST", "/api/counter/increment").await;
        assert_eq!(response.status(), StatusCode::OK);

        let counter_response = parse_response(response).await;
        assert_eq!(counter_response.count, i);
    }

    // Verify current count with GET
    let response = send_request(&app, "GET", "/api/counter").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 3);
}

#[tokio::test]
async fn test_api_decrement_returns_correct_value() {
    let app = create_test_app();

    // First increment to 3
    for _ in 0..3 {
        let _ = send_request(&app, "POST", "/api/counter/increment").await;
    }

    // Then decrement
    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    assert_eq!(response.status(), StatusCode::OK);

    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 2);
}

#[tokio::test]
async fn test_api_decrement_does_not_go_below_zero() {
    let app = create_test_app();

    // Try to decrement at 0
    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    assert_eq!(response.status(), StatusCode::OK);

    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);

    // Verify with GET
    let response = send_request(&app, "GET", "/api/counter").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);
}

#[tokio::test]
async fn test_api_increment_and_decrement_sequence() {
    let app = create_test_app();

    // Increment to 3
    for _ in 0..3 {
        let _ = send_request(&app, "POST", "/api/counter/increment").await;
    }

    // Decrement twice
    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 2);

    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 1);

    // Decrement to 0
    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);

    // Try to decrement below 0
    let response = send_request(&app, "POST", "/api/counter/decrement").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);
}

#[tokio::test]
async fn test_api_cors_headers_present() {
    use axum::http::header;

    let app = create_test_app();

    let request = Request::builder()
        .method("OPTIONS")
        .uri("/api/counter")
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // CORS layer should handle OPTIONS requests
    assert_eq!(response.status(), StatusCode::OK);

    // Check CORS headers are present
    let headers = response.headers();
    assert!(headers.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
}

#[tokio::test]
async fn test_api_full_workflow() {
    let app = create_test_app();

    // Start at 0
    let response = send_request(&app, "GET", "/api/counter").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 0);

    // Increment 5 times
    for i in 1..=5 {
        let response = send_request(&app, "POST", "/api/counter/increment").await;
        let counter_response = parse_response(response).await;
        assert_eq!(counter_response.count, i);
    }

    // Decrement 3 times
    for i in (2..=4).rev() {
        let response = send_request(&app, "POST", "/api/counter/decrement").await;
        let counter_response = parse_response(response).await;
        assert_eq!(counter_response.count, i);
    }

    // Final value should be 2
    let response = send_request(&app, "GET", "/api/counter").await;
    let counter_response = parse_response(response).await;
    assert_eq!(counter_response.count, 2);
}
