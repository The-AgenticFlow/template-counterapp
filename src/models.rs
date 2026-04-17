use serde::Serialize;

/// Response structure for counter endpoints
#[derive(Serialize)]
pub struct CounterResponse {
    pub count: i32,
}

impl CounterResponse {
    pub fn new(count: i32) -> Self {
        Self { count }
    }
}
