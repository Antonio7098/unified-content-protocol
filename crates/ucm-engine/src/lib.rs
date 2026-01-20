//! # UCM Engine
//!
//! Transformation engine for applying operations to UCM documents.
//!
//! This crate provides:
//! - Transaction management for atomic operations
//! - Snapshot/restore functionality
//! - Operation execution
//! - Validation pipeline

pub mod config;
pub mod engine;
pub mod error;
pub mod operation;
pub mod section;
pub mod snapshot;
pub mod transaction;
pub mod traversal;
pub mod validate;

pub use engine::Engine;
pub use operation::{EditOperator, MoveTarget, Operation, OperationResult, PruneCondition};
pub use snapshot::{Snapshot, SnapshotId, SnapshotManager};
pub use transaction::{Transaction, TransactionId, TransactionManager, TransactionState};
pub use validate::{ValidationPipeline, ValidationResult};
