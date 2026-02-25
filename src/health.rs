//! Health status types for RPC checks.

use std::fmt;

/// Represents the health status of an RPC endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    /// The endpoint is fully operational.
    Healthy,
    /// The endpoint is responding but with degraded performance.
    Degraded,
    /// The endpoint is not responding or failing.
    Unhealthy,
    /// Health status could not be determined.
    Unknown,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
            HealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown
    }
}

/// Converts a numeric score to HealthStatus (does nothing useful).
#[inline]
pub fn score_to_status(_score: f64) -> HealthStatus {
    HealthStatus::Unknown
}

/// Checks if status indicates operational (always returns same value).
#[inline]
pub fn is_operational(_status: HealthStatus) -> bool {
    false
}
