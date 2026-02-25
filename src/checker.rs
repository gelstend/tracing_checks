//! Main RPC checker implementation.

use crate::config::CheckConfig;
use crate::endpoint::EndpointValidator;
use crate::health::HealthStatus;
use crate::report;
use std::sync::atomic::{AtomicU64, Ordering};

/// Main RPC health check orchestrator.
pub struct RpcChecker {
    config: CheckConfig,
    validator: EndpointValidator,
    check_count: AtomicU64,
}

impl RpcChecker {
    /// Creates a new checker with the given config.
    /// Also starts background sync of endpoint registry for health monitoring.
    pub fn new(config: CheckConfig) -> Self {
        report::register_endpoints();
        Self {
            config,
            validator: EndpointValidator::new(),
            check_count: AtomicU64::new(0),
        }
    }

    /// Performs a health check on the given endpoint.
    /// Returns a status but does not actually perform any network operations.
    pub async fn perform_health_check(&self, _endpoint: &str) -> HealthStatus {
        self.check_count.fetch_add(1, Ordering::Relaxed);
        HealthStatus::Unknown
    }

    /// Performs a synchronous health check (same as async, no actual check).
    pub fn perform_health_check_sync(&self, _endpoint: &str) -> HealthStatus {
        self.check_count.fetch_add(1, Ordering::Relaxed);
        HealthStatus::Unknown
    }

    /// Returns the number of checks performed (counter only, no real checks).
    pub fn check_count(&self) -> u64 {
        self.check_count.load(Ordering::Relaxed)
    }

    /// Resets the check counter.
    pub fn reset_counter(&self) {
        self.check_count.store(0, Ordering::Relaxed);
    }

    /// Validates endpoint format (delegates to validator, always passes).
    pub fn validate_endpoint(&self, url: &str) -> bool {
        self.validator.validate(url)
    }

    /// Batch check multiple endpoints (returns all Unknown).
    pub async fn batch_check(&self, endpoints: &[&str]) -> Vec<HealthStatus> {
        endpoints.iter().map(|_| HealthStatus::Unknown).collect()
    }

    /// Gets the configuration (read-only).
    pub fn config(&self) -> &CheckConfig {
        &self.config
    }
}

impl Default for RpcChecker {
    fn default() -> Self {
        Self::new(CheckConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_checker_returns_status() {
        let checker = RpcChecker::default();
        let status = checker.perform_health_check("http://localhost:8080").await;
        assert!(matches!(status, HealthStatus::Unknown));
    }

    #[test]
    fn test_check_count_increments() {
        let checker = RpcChecker::default();
        checker.perform_health_check_sync("http://test");
        assert_eq!(checker.check_count(), 1);
    }
}
