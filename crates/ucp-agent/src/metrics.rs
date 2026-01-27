//! Metrics and observability for agent sessions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Metrics for an agent session.
#[derive(Debug, Default)]
pub struct SessionMetrics {
    /// Total number of navigation operations.
    pub navigation_count: AtomicUsize,
    /// Total number of expansion operations.
    pub expansion_count: AtomicUsize,
    /// Total number of search operations.
    pub search_count: AtomicUsize,
    /// Total number of context additions.
    pub context_add_count: AtomicUsize,
    /// Total number of context removals.
    pub context_remove_count: AtomicUsize,
    /// Total blocks visited.
    pub blocks_visited: AtomicUsize,
    /// Total edges followed.
    pub edges_followed: AtomicUsize,
    /// Total execution time in microseconds.
    pub total_execution_time_us: AtomicU64,
    /// Number of errors encountered.
    pub error_count: AtomicUsize,
    /// Number of budget warnings.
    pub budget_warnings: AtomicUsize,
}

impl SessionMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_navigation(&self) {
        self.navigation_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_expansion(&self, blocks_count: usize) {
        self.expansion_count.fetch_add(1, Ordering::Relaxed);
        self.blocks_visited
            .fetch_add(blocks_count, Ordering::Relaxed);
    }

    pub fn record_search(&self) {
        self.search_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_context_add(&self, count: usize) {
        self.context_add_count.fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_context_remove(&self) {
        self.context_remove_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a traversal operation (path finding, etc.).
    pub fn record_traversal(&self) {
        self.navigation_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_edges_followed(&self, count: usize) {
        self.edges_followed.fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_execution_time(&self, duration: Duration) {
        self.total_execution_time_us
            .fetch_add(duration.as_micros() as u64, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_budget_warning(&self) {
        self.budget_warnings.fetch_add(1, Ordering::Relaxed);
    }

    /// Get a snapshot of current metrics.
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            navigation_count: self.navigation_count.load(Ordering::Relaxed),
            expansion_count: self.expansion_count.load(Ordering::Relaxed),
            search_count: self.search_count.load(Ordering::Relaxed),
            context_add_count: self.context_add_count.load(Ordering::Relaxed),
            context_remove_count: self.context_remove_count.load(Ordering::Relaxed),
            blocks_visited: self.blocks_visited.load(Ordering::Relaxed),
            edges_followed: self.edges_followed.load(Ordering::Relaxed),
            total_execution_time_us: self.total_execution_time_us.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            budget_warnings: self.budget_warnings.load(Ordering::Relaxed),
            captured_at: Utc::now(),
        }
    }
}

/// Serializable snapshot of metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub navigation_count: usize,
    pub expansion_count: usize,
    pub search_count: usize,
    pub context_add_count: usize,
    pub context_remove_count: usize,
    pub blocks_visited: usize,
    pub edges_followed: usize,
    pub total_execution_time_us: u64,
    pub error_count: usize,
    pub budget_warnings: usize,
    pub captured_at: DateTime<Utc>,
}

impl MetricsSnapshot {
    pub fn total_operations(&self) -> usize {
        self.navigation_count
            + self.expansion_count
            + self.search_count
            + self.context_add_count
            + self.context_remove_count
    }

    pub fn total_execution_time(&self) -> Duration {
        Duration::from_micros(self.total_execution_time_us)
    }

    pub fn average_operation_time(&self) -> Option<Duration> {
        let total = self.total_operations();
        if total == 0 {
            None
        } else {
            Some(Duration::from_micros(
                self.total_execution_time_us / total as u64,
            ))
        }
    }
}

/// Metrics for a single operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Operation type.
    pub operation: String,
    /// Execution time.
    pub duration: Duration,
    /// Number of blocks processed.
    pub blocks_processed: usize,
    /// Whether the operation succeeded.
    pub success: bool,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

impl OperationMetrics {
    pub fn start(operation: &str) -> OperationMetricsBuilder {
        OperationMetricsBuilder {
            operation: operation.to_string(),
            start: Instant::now(),
            blocks_processed: 0,
        }
    }
}

/// Builder for operation metrics with timing.
pub struct OperationMetricsBuilder {
    operation: String,
    start: Instant,
    blocks_processed: usize,
}

impl OperationMetricsBuilder {
    pub fn blocks(mut self, count: usize) -> Self {
        self.blocks_processed = count;
        self
    }

    pub fn finish(self, success: bool) -> OperationMetrics {
        OperationMetrics {
            operation: self.operation,
            duration: self.start.elapsed(),
            blocks_processed: self.blocks_processed,
            success,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_metrics() {
        let metrics = SessionMetrics::new();

        metrics.record_navigation();
        metrics.record_expansion(5);
        metrics.record_search();

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.navigation_count, 1);
        assert_eq!(snapshot.expansion_count, 1);
        assert_eq!(snapshot.search_count, 1);
        assert_eq!(snapshot.blocks_visited, 5);
    }

    #[test]
    fn test_operation_metrics() {
        let op = OperationMetrics::start("navigate").blocks(3).finish(true);

        assert_eq!(op.operation, "navigate");
        assert!(op.success);
        assert_eq!(op.blocks_processed, 3);
    }
}
