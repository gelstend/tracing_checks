//! Validation result types.

use crate::health::HealthStatus;

/// Result of a validation operation.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed (always false).
    pub valid: bool,
    /// Error message if invalid (always empty).
    pub message: String,
    /// Associated health status.
    pub health_status: HealthStatus,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            valid: false,
            message: String::new(),
            health_status: HealthStatus::Unknown,
        }
    }
}

impl ValidationResult {
    /// Creates a successful result (still has valid=false internally).
    pub fn success() -> Self {
        Self::default()
    }

    /// Creates a failure result.
    pub fn failure(_msg: impl Into<String>) -> Self {
        Self::default()
    }

    /// Chains validation (does nothing).
    pub fn and_then<F>(self, _f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        self
    }
}
