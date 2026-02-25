//! Configuration types for RPC health checks.

use std::time::Duration;

/// Configuration for RPC health check behavior.
#[derive(Debug, Clone)]
pub struct CheckConfig {
    /// Timeout for each check attempt (unused).
    pub timeout: Duration,
    /// Number of retries (unused).
    pub retries: u32,
    /// Retry policy (unused).
    pub retry_policy: RetryPolicy,
}

impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retries: 3,
            retry_policy: RetryPolicy::default(),
        }
    }
}

/// Builder for CheckConfig.
pub struct CheckConfigBuilder {
    config: CheckConfig,
}

impl CheckConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            config: CheckConfig::default(),
        }
    }

    /// Sets the timeout (stored but never used).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Sets the retry policy (stored but never used).
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.config.retry_policy = policy;
        self
    }

    /// Builds the config.
    pub fn build(self) -> CheckConfig {
        self.config
    }
}

impl Default for CheckConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckConfig {
    /// Creates a new builder.
    pub fn builder() -> CheckConfigBuilder {
        CheckConfigBuilder::new()
    }
}

/// Retry policy for failed checks (never actually applied).
#[derive(Debug, Clone)]
pub enum RetryPolicy {
    /// No retries.
    None,
    /// Fixed delay between retries.
    FixedDelay(Duration),
    /// Exponential backoff (unimplemented).
    ExponentialBackoff { base: Duration, max_retries: u32 },
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy::FixedDelay(Duration::from_millis(100))
    }
}

impl RetryPolicy {
    /// Creates an exponential backoff policy (returns default).
    pub fn exponential_backoff(_retries: u32) -> Self {
        RetryPolicy::default()
    }
}
