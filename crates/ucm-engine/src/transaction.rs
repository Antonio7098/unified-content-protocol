//! Transaction management for atomic operations.

use crate::operation::Operation;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use ucm_core::{Error, Result};

/// Transaction identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub String);

impl TransactionId {
    pub fn generate() -> Self {
        use chrono::Utc;
        #[cfg(not(target_arch = "wasm32"))]
        let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        #[cfg(target_arch = "wasm32")]
        let ts = 0; // Fallback for WASM if Utc::now() panics
        Self(format!("txn_{:x}", ts))
    }

    pub fn named(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl std::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Transaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    TimedOut,
}

/// A transaction groups operations for atomic execution
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Transaction ID
    pub id: TransactionId,
    /// Optional name
    pub name: Option<String>,
    /// Operations in this transaction
    pub operations: Vec<Operation>,
    /// Savepoints for partial rollback
    pub savepoints: Vec<Savepoint>,
    /// Current state
    pub state: TransactionState,
    /// Start time
    #[cfg(not(target_arch = "wasm32"))]
    pub started_at: Instant,
    /// Created timestamp
    #[cfg(not(target_arch = "wasm32"))]
    pub created_at: DateTime<Utc>,
    /// Timeout duration
    pub timeout: Duration,
}

/// Savepoint within a transaction
#[derive(Debug, Clone)]
pub struct Savepoint {
    pub name: String,
    pub operation_index: usize,
    #[cfg(not(target_arch = "wasm32"))]
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(timeout: Duration) -> Self {
        Self {
            id: TransactionId::generate(),
            name: None,
            operations: Vec::new(),
            savepoints: Vec::new(),
            state: TransactionState::Active,
            #[cfg(not(target_arch = "wasm32"))]
            started_at: Instant::now(),
            #[cfg(not(target_arch = "wasm32"))]
            created_at: Utc::now(),
            timeout,
        }
    }

    /// Create a named transaction
    pub fn named(name: impl Into<String>, timeout: Duration) -> Self {
        let name = name.into();
        Self {
            id: TransactionId::named(&name),
            name: Some(name),
            operations: Vec::new(),
            savepoints: Vec::new(),
            state: TransactionState::Active,
            #[cfg(not(target_arch = "wasm32"))]
            started_at: Instant::now(),
            #[cfg(not(target_arch = "wasm32"))]
            created_at: Utc::now(),
            timeout,
        }
    }

    /// Add an operation to the transaction
    pub fn add_operation(&mut self, op: Operation) -> Result<()> {
        if self.state != TransactionState::Active {
            return Err(Error::Internal(format!(
                "Cannot add operation to {:?} transaction",
                self.state
            )));
        }
        if self.is_timed_out() {
            self.state = TransactionState::TimedOut;
            return Err(Error::new(
                ucm_core::ErrorCode::E301TransactionTimeout,
                "Transaction timed out",
            ));
        }
        self.operations.push(op);
        Ok(())
    }

    /// Create a savepoint
    pub fn savepoint(&mut self, name: impl Into<String>) {
        self.savepoints.push(Savepoint {
            name: name.into(),
            operation_index: self.operations.len(),
            #[cfg(not(target_arch = "wasm32"))]
            created_at: Utc::now(),
        });
    }

    /// Check if transaction has timed out
    pub fn is_timed_out(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        return self.started_at.elapsed() > self.timeout;

        #[cfg(target_arch = "wasm32")]
        false
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        #[cfg(not(target_arch = "wasm32"))]
        return self.started_at.elapsed();

        #[cfg(target_arch = "wasm32")]
        Duration::from_secs(0)
    }

    /// Get operation count
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
}

/// Manages active transactions
#[derive(Debug, Default)]
pub struct TransactionManager {
    /// Active transactions
    transactions: HashMap<TransactionId, Transaction>,
    /// Default timeout
    default_timeout: Duration,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            default_timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            transactions: HashMap::new(),
            default_timeout: timeout,
        }
    }

    /// Begin a new transaction
    pub fn begin(&mut self) -> TransactionId {
        let txn = Transaction::new(self.default_timeout);
        let id = txn.id.clone();
        self.transactions.insert(id.clone(), txn);
        id
    }

    /// Begin a named transaction
    pub fn begin_named(&mut self, name: impl Into<String>) -> TransactionId {
        let txn = Transaction::named(name, self.default_timeout);
        let id = txn.id.clone();
        self.transactions.insert(id.clone(), txn);
        id
    }

    /// Get a transaction by ID
    pub fn get(&self, id: &TransactionId) -> Option<&Transaction> {
        self.transactions.get(id)
    }

    /// Get a mutable transaction by ID
    pub fn get_mut(&mut self, id: &TransactionId) -> Option<&mut Transaction> {
        self.transactions.get_mut(id)
    }

    /// Add operation to a transaction
    pub fn add_operation(&mut self, id: &TransactionId, op: Operation) -> Result<()> {
        let txn = self.transactions.get_mut(id).ok_or_else(|| {
            Error::new(ucm_core::ErrorCode::E303TransactionNotFound, id.to_string())
        })?;
        txn.add_operation(op)
    }

    /// Commit a transaction (returns operations to execute)
    pub fn commit(&mut self, id: &TransactionId) -> Result<Vec<Operation>> {
        let txn = self.transactions.get_mut(id).ok_or_else(|| {
            Error::new(ucm_core::ErrorCode::E303TransactionNotFound, id.to_string())
        })?;

        if txn.state != TransactionState::Active {
            return Err(Error::Internal(format!(
                "Cannot commit {:?} transaction",
                txn.state
            )));
        }

        if txn.is_timed_out() {
            txn.state = TransactionState::TimedOut;
            return Err(Error::new(
                ucm_core::ErrorCode::E301TransactionTimeout,
                "Transaction timed out",
            ));
        }

        txn.state = TransactionState::Committed;
        Ok(txn.operations.clone())
    }

    /// Rollback a transaction
    pub fn rollback(&mut self, id: &TransactionId) -> Result<()> {
        let txn = self.transactions.get_mut(id).ok_or_else(|| {
            Error::new(ucm_core::ErrorCode::E303TransactionNotFound, id.to_string())
        })?;

        if txn.state != TransactionState::Active {
            return Err(Error::Internal(format!(
                "Cannot rollback {:?} transaction",
                txn.state
            )));
        }

        txn.state = TransactionState::RolledBack;
        Ok(())
    }

    /// Remove completed transactions
    pub fn cleanup(&mut self) {
        self.transactions
            .retain(|_, txn| txn.state == TransactionState::Active && !txn.is_timed_out());
    }

    /// Get active transaction count
    pub fn active_count(&self) -> usize {
        self.transactions
            .values()
            .filter(|t| t.state == TransactionState::Active)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::PruneCondition;

    #[test]
    fn test_transaction_lifecycle() {
        let mut mgr = TransactionManager::new();

        let id = mgr.begin();
        assert_eq!(mgr.active_count(), 1);

        mgr.add_operation(
            &id,
            Operation::Prune {
                condition: Some(PruneCondition::Unreachable),
            },
        )
        .unwrap();

        let ops = mgr.commit(&id).unwrap();
        assert_eq!(ops.len(), 1);
    }

    #[test]
    fn test_named_transaction() {
        let mut mgr = TransactionManager::new();

        let id = mgr.begin_named("my-transaction");
        assert_eq!(id.0, "my-transaction");
    }

    #[test]
    fn test_rollback() {
        let mut mgr = TransactionManager::new();

        let id = mgr.begin();
        mgr.rollback(&id).unwrap();

        let txn = mgr.get(&id).unwrap();
        assert_eq!(txn.state, TransactionState::RolledBack);
    }

    #[test]
    fn test_timeout() {
        let mut mgr = TransactionManager::with_timeout(Duration::from_millis(1));

        let id = mgr.begin();
        std::thread::sleep(Duration::from_millis(10));

        let result = mgr.commit(&id);
        assert!(result.is_err());
    }
}
