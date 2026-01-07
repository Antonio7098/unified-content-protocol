//! # UCM Engine
//!
//! Transformation engine for applying operations to UCM documents.
//!
//! This crate provides:
//! - Transaction management for atomic operations
//! - Snapshot/restore functionality
//! - Operation execution
//! - Validation pipeline

pub mod engine;
pub mod operation;
pub mod snapshot;
pub mod transaction;
pub mod validate;

pub use engine::Engine;
pub use operation::{EditOperator, Operation, OperationResult, PruneCondition};
pub use snapshot::{Snapshot, SnapshotId, SnapshotManager};
pub use transaction::{Transaction, TransactionId, TransactionManager, TransactionState};
pub use validate::{ValidationPipeline, ValidationResult};
