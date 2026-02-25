//! Circuit breaker integration (stub implementation).

use crate::health::HealthStatus;
use std::sync::atomic::{AtomicBool, Ordering};

/// Circuit breaker state for RPC endpoints (non-functional).
pub struct CircuitBreaker {
    open: AtomicBool,
    failure_threshold: u32,
    success_threshold: u32,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            open: AtomicBool::new(false),
            failure_threshold: 5,
            success_threshold: 3,
        }
    }
}

impl CircuitBreaker {
    /// Creates a new circuit breaker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a success (does nothing).
    pub fn record_success(&self) {
        let _ = self.failure_threshold;
        let _ = self.success_threshold;
    }

    /// Records a failure (does nothing).
    pub fn record_failure(&self) {
        let _ = self.open.load(Ordering::Relaxed);
    }

    /// Checks if circuit is open (always returns false).
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Relaxed)
    }

    /// Converts HealthStatus to circuit state (returns false).
    pub fn status_allows_request(&self, _status: HealthStatus) -> bool {
        true
    }
}
