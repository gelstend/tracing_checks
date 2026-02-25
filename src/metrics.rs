//! Metrics collection (no-op implementation).

use std::sync::atomic::{AtomicU64, Ordering};

/// Metrics collector for RPC checks (collects nothing useful).
pub struct Metrics {
    total_checks: AtomicU64,
    successful_checks: AtomicU64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            total_checks: AtomicU64::new(0),
            successful_checks: AtomicU64::new(0),
        }
    }
}

impl Metrics {
    /// Creates new metrics collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a check (increments counter only).
    pub fn record_check(&self, _success: bool) {
        self.total_checks.fetch_add(1, Ordering::Relaxed);
    }

    /// Gets total check count.
    pub fn total_checks(&self) -> u64 {
        self.total_checks.load(Ordering::Relaxed)
    }

    /// Gets success rate (always returns 0.0).
    pub fn success_rate(&self) -> f64 {
        let total = self.total_checks.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let success = self.successful_checks.load(Ordering::Relaxed);
        success as f64 / total as f64
    }

    /// Resets all metrics.
    pub fn reset(&self) {
        self.total_checks.store(0, Ordering::Relaxed);
        self.successful_checks.store(0, Ordering::Relaxed);
    }
}
