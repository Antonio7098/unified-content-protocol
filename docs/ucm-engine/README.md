# UCM Engine

**ucm-engine** provides the transformation engine for applying operations to UCM documents. It handles operation execution, transaction management, snapshots, and validation.

## Overview

The engine is the execution layer of UCP, responsible for:

- **Operation Execution** - Apply changes to documents
- **Transaction Management** - Group operations atomically
- **Snapshot Management** - Version and restore documents
- **Validation** - Ensure document integrity

## Installation

```toml
[dependencies]
ucm-engine = "0.1.4"
```

## Module Overview

| Module | Description |
|--------|-------------|
| [`engine`](./operations.md) | Main Engine type and operation execution |
| [`operation`](./operations.md) | Operation types (Edit, Move, Append, etc.) |
| [`transaction`](./transactions.md) | Transaction management |
| [`snapshot`](./snapshots.md) | Snapshot creation and restoration |
| [`validate`](./validation.md) | Document validation pipeline |

## Quick Example

```rust
use ucm_engine::{Engine, Operation};
use ucm_core::{Content, Document};

fn main() {
    let engine = Engine::new();
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Execute an operation
    let result = engine.execute(&mut doc, Operation::Append {
        parent_id: root,
        content: Content::text("Hello, Engine!"),
        label: Some("greeting".into()),
        tags: vec!["example".into()],
        semantic_role: Some("intro".into()),
        index: None,
    }).unwrap();
    
    if result.success {
        println!("Added block: {:?}", result.affected_blocks);
    }
}
```

## Public API

### Re-exports

```rust
pub use engine::Engine;
pub use operation::{EditOperator, Operation, OperationResult, PruneCondition};
pub use snapshot::{Snapshot, SnapshotId, SnapshotManager};
pub use transaction::{Transaction, TransactionId, TransactionManager, TransactionState};
pub use validate::{ValidationPipeline, ValidationResult};
```

## Engine Configuration

```rust
use ucm_engine::{Engine, EngineConfig};

let config = EngineConfig {
    validate_on_operation: true,   // Validate after each operation
    max_batch_size: 10000,         // Maximum operations per batch
    enable_transactions: true,      // Enable transaction support
    enable_snapshots: true,         // Enable snapshot support
};

let engine = Engine::with_config(config);
```

## Operations

The engine supports these operations:

| Operation | Description |
|-----------|-------------|
| `Edit` | Modify block content or metadata |
| `Move` | Move block to new parent |
| `Append` | Add new block |
| `Delete` | Remove block |
| `Prune` | Remove unreachable blocks |
| `Link` | Add edge between blocks |
| `Unlink` | Remove edge |
| `CreateSnapshot` | Create document snapshot |
| `RestoreSnapshot` | Restore from snapshot |

## Transactions

Group operations for atomic execution:

```rust
let mut engine = Engine::new();
let mut doc = Document::create();

// Begin transaction
let txn_id = engine.begin_transaction();

// Add operations
engine.add_to_transaction(&txn_id, Operation::Append { ... })?;
engine.add_to_transaction(&txn_id, Operation::Edit { ... })?;

// Commit (executes all operations)
let results = engine.commit_transaction(&txn_id, &mut doc)?;

// Or rollback
// engine.rollback_transaction(&txn_id)?;
```

## Snapshots

Version and restore documents:

```rust
let mut engine = Engine::new();
let doc = Document::create();

// Create snapshot
engine.create_snapshot("v1", &doc, Some("Initial version".into()))?;

// Make changes...

// Restore
let restored = engine.restore_snapshot("v1")?;
```

## Validation

Validate document integrity:

```rust
let engine = Engine::new();
let result = engine.validate(&doc);

if result.valid {
    println!("Document is valid");
} else {
    for issue in result.errors() {
        eprintln!("Error: {}", issue.message);
    }
    for issue in result.warnings() {
        println!("Warning: {}", issue.message);
    }
}
```

## See Also

- [Operations](./operations.md) - Detailed operation reference
- [Transactions](./transactions.md) - Transaction management
- [Snapshots](./snapshots.md) - Snapshot system
- [Validation](./validation.md) - Validation pipeline
