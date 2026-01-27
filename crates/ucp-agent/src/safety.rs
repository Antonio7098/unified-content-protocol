//! Safety mechanisms: limits, circuit breakers, and guards.

use crate::error::{AgentError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Global limits across all sessions.
#[derive(Debug, Clone)]
pub struct GlobalLimits {
    /// Maximum concurrent sessions.
    pub max_sessions: usize,
    /// Maximum total blocks in memory across all contexts.
    pub max_total_context_blocks: usize,
    /// Maximum operations per second (rate limiting).
    pub max_ops_per_second: f64,
    /// Global timeout for any single operation.
    pub operation_timeout: Duration,
}

impl Default for GlobalLimits {
    fn default() -> Self {
        Self {
            max_sessions: 100,
            max_total_context_blocks: 100_000,
            max_ops_per_second: 1000.0,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

/// Per-session limits.
#[derive(Debug, Clone)]
pub struct SessionLimits {
    /// Maximum context window tokens.
    pub max_context_tokens: usize,
    /// Maximum context window blocks.
    pub max_context_blocks: usize,
    /// Maximum depth for single expansion.
    pub max_expand_depth: usize,
    /// Maximum blocks returned per operation.
    pub max_results_per_operation: usize,
    /// Maximum operations before forced pause.
    pub max_operations_before_checkpoint: usize,
    /// Session timeout (inactivity).
    pub session_timeout: Duration,
    /// Maximum navigation history size.
    pub max_history_size: usize,
    /// Budget for costly operations.
    pub budget: OperationBudget,
}

impl Default for SessionLimits {
    fn default() -> Self {
        Self {
            max_context_tokens: 8_000,
            max_context_blocks: 200,
            max_expand_depth: 10,
            max_results_per_operation: 100,
            max_operations_before_checkpoint: 1000,
            session_timeout: Duration::from_secs(30 * 60), // 30 minutes
            max_history_size: 100,
            budget: OperationBudget::default(),
        }
    }
}

/// Budget for costly operations.
#[derive(Debug, Clone)]
pub struct OperationBudget {
    /// Total allowed traversal operations.
    pub traversal_operations: usize,
    /// Total allowed search operations.
    pub search_operations: usize,
    /// Total blocks allowed to be read.
    pub blocks_read: usize,
}

impl Default for OperationBudget {
    fn default() -> Self {
        Self {
            traversal_operations: 10_000,
            search_operations: 100,
            blocks_read: 50_000,
        }
    }
}

/// Tracks budget usage.
#[derive(Debug, Default)]
pub struct BudgetTracker {
    pub traversal_ops_used: AtomicUsize,
    pub search_ops_used: AtomicUsize,
    pub blocks_read_used: AtomicUsize,
}

impl BudgetTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_traversal(&self) {
        self.traversal_ops_used.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_search(&self) {
        self.search_ops_used.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_blocks_read(&self, count: usize) {
        self.blocks_read_used.fetch_add(count, Ordering::Relaxed);
    }

    pub fn check_traversal_budget(&self, budget: &OperationBudget) -> Result<()> {
        let used = self.traversal_ops_used.load(Ordering::Relaxed);
        if used >= budget.traversal_operations {
            return Err(AgentError::BudgetExhausted {
                operation_type: "traversal".to_string(),
            });
        }
        Ok(())
    }

    pub fn check_search_budget(&self, budget: &OperationBudget) -> Result<()> {
        let used = self.search_ops_used.load(Ordering::Relaxed);
        if used >= budget.search_operations {
            return Err(AgentError::BudgetExhausted {
                operation_type: "search".to_string(),
            });
        }
        Ok(())
    }

    pub fn check_blocks_budget(&self, budget: &OperationBudget) -> Result<()> {
        let used = self.blocks_read_used.load(Ordering::Relaxed);
        if used >= budget.blocks_read {
            return Err(AgentError::BudgetExhausted {
                operation_type: "blocks_read".to_string(),
            });
        }
        Ok(())
    }

    pub fn reset(&self) {
        self.traversal_ops_used.store(0, Ordering::Relaxed);
        self.search_ops_used.store(0, Ordering::Relaxed);
        self.blocks_read_used.store(0, Ordering::Relaxed);
    }
}

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation.
    Closed,
    /// Failing, rejecting requests.
    Open,
    /// Testing recovery.
    HalfOpen,
}

/// Circuit breaker for detecting runaway operations.
pub struct CircuitBreaker {
    state: RwLock<CircuitState>,
    failure_count: AtomicUsize,
    failure_threshold: usize,
    recovery_timeout: Duration,
    last_failure: RwLock<Option<Instant>>,
    success_count_in_half_open: AtomicUsize,
    success_threshold: usize,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        Self {
            state: RwLock::new(CircuitState::Closed),
            failure_count: AtomicUsize::new(0),
            failure_threshold,
            recovery_timeout,
            last_failure: RwLock::new(None),
            success_count_in_half_open: AtomicUsize::new(0),
            success_threshold: 3, // Require 3 successes to close
        }
    }

    pub fn state(&self) -> CircuitState {
        *self.state.read().unwrap()
    }

    pub fn can_proceed(&self) -> Result<()> {
        let state = *self.state.read().unwrap();

        match state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                // Check if recovery timeout has passed
                let last_failure = self.last_failure.read().unwrap();
                if let Some(last) = *last_failure {
                    if last.elapsed() >= self.recovery_timeout {
                        // Transition to half-open
                        drop(last_failure);
                        *self.state.write().unwrap() = CircuitState::HalfOpen;
                        self.success_count_in_half_open.store(0, Ordering::Relaxed);
                        debug!("Circuit breaker transitioning to half-open");
                        return Ok(());
                    }
                }
                Err(AgentError::CircuitOpen {
                    reason: "Too many failures, circuit is open".to_string(),
                })
            }
            CircuitState::HalfOpen => {
                // Allow one request through to test
                Ok(())
            }
        }
    }

    pub fn record_success(&self) {
        let state = *self.state.read().unwrap();

        match state {
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::Relaxed);
            }
            CircuitState::HalfOpen => {
                let successes = self
                    .success_count_in_half_open
                    .fetch_add(1, Ordering::Relaxed)
                    + 1;
                if successes >= self.success_threshold {
                    // Transition back to closed
                    *self.state.write().unwrap() = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::Relaxed);
                    debug!("Circuit breaker closed after successful recovery");
                }
            }
            CircuitState::Open => {
                // Shouldn't happen, but ignore
            }
        }
    }

    pub fn record_failure(&self) {
        let state = *self.state.read().unwrap();

        match state {
            CircuitState::Closed => {
                let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                if failures >= self.failure_threshold {
                    *self.state.write().unwrap() = CircuitState::Open;
                    *self.last_failure.write().unwrap() = Some(Instant::now());
                    warn!(
                        "Circuit breaker opened after {} failures",
                        self.failure_threshold
                    );
                }
            }
            CircuitState::HalfOpen => {
                // Failure during recovery - go back to open
                *self.state.write().unwrap() = CircuitState::Open;
                *self.last_failure.write().unwrap() = Some(Instant::now());
                self.success_count_in_half_open.store(0, Ordering::Relaxed);
                warn!("Circuit breaker re-opened after failure during half-open");
            }
            CircuitState::Open => {
                // Update last failure time
                *self.last_failure.write().unwrap() = Some(Instant::now());
            }
        }
    }

    pub fn reset(&self) {
        *self.state.write().unwrap() = CircuitState::Closed;
        self.failure_count.store(0, Ordering::Relaxed);
        *self.last_failure.write().unwrap() = None;
        self.success_count_in_half_open.store(0, Ordering::Relaxed);
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5, Duration::from_secs(30))
    }
}

/// RAII guard for depth tracking.
pub struct DepthGuardHandle<'a> {
    guard: &'a DepthGuard,
}

impl<'a> Drop for DepthGuardHandle<'a> {
    fn drop(&mut self) {
        self.guard.current.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Depth guard prevents infinite recursion.
pub struct DepthGuard {
    current: AtomicUsize,
    max: usize,
}

impl DepthGuard {
    pub fn new(max: usize) -> Self {
        Self {
            current: AtomicUsize::new(0),
            max,
        }
    }

    /// Try to enter a deeper level. Returns a guard handle if successful.
    pub fn try_enter(&self) -> Result<DepthGuardHandle<'_>> {
        let current = self.current.fetch_add(1, Ordering::Relaxed);
        if current >= self.max {
            self.current.fetch_sub(1, Ordering::Relaxed);
            return Err(AgentError::DepthLimitExceeded {
                current: current + 1,
                max: self.max,
            });
        }
        Ok(DepthGuardHandle { guard: self })
    }

    pub fn current_depth(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    pub fn max_depth(&self) -> usize {
        self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_tracker() {
        let tracker = BudgetTracker::new();
        let budget = OperationBudget {
            traversal_operations: 3,
            search_operations: 2,
            blocks_read: 10,
        };

        // Record some operations
        tracker.record_traversal();
        tracker.record_traversal();
        assert!(tracker.check_traversal_budget(&budget).is_ok());

        tracker.record_traversal();
        assert!(tracker.check_traversal_budget(&budget).is_err());

        // Reset and try again
        tracker.reset();
        assert!(tracker.check_traversal_budget(&budget).is_ok());
    }

    #[test]
    fn test_circuit_breaker() {
        let cb = CircuitBreaker::new(3, Duration::from_millis(100));

        // Initially closed
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.can_proceed().is_ok());

        // Record failures until open
        cb.record_failure();
        cb.record_failure();
        assert!(cb.can_proceed().is_ok());

        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(cb.can_proceed().is_err());

        // Wait for recovery timeout
        std::thread::sleep(Duration::from_millis(150));
        assert!(cb.can_proceed().is_ok()); // Should transition to half-open
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Success in half-open should eventually close
        cb.record_success();
        cb.record_success();
        cb.record_success();
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_depth_guard() {
        let guard = DepthGuard::new(3);

        assert_eq!(guard.current_depth(), 0);

        {
            let _h1 = guard.try_enter().unwrap();
            assert_eq!(guard.current_depth(), 1);

            {
                let _h2 = guard.try_enter().unwrap();
                assert_eq!(guard.current_depth(), 2);

                {
                    let _h3 = guard.try_enter().unwrap();
                    assert_eq!(guard.current_depth(), 3);

                    // Should fail now
                    assert!(guard.try_enter().is_err());
                }
                assert_eq!(guard.current_depth(), 2);
            }
            assert_eq!(guard.current_depth(), 1);
        }
        assert_eq!(guard.current_depth(), 0);
    }
}
