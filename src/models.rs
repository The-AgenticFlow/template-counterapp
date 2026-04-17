//! Data structures for API responses
//!
//! This module contains the JSON response models for the Counter API.

use serde::Serialize;

/// Response structure for counter endpoints
#[derive(Serialize)]
pub struct CounterResponse {
    /// Current counter value
    pub count: i32,
}
