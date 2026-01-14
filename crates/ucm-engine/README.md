# UCM Engine

**ucm-engine** provides the transformation engine for applying operations to UCM documents. It handles the execution of UCL commands and manages document state changes.

## Overview

UCM Engine is responsible for:

- **Operation Execution** - Applying transformations to documents
- **Transaction Management** - Atomic operations and rollback
- **Snapshot Management** - Document versioning and restoration
- **Validation** - Document integrity checking
- **Performance Optimization** - Efficient batch operations

## Installation

```toml
[dependencies]
ucm-engine = "0.1"
```

## Quick Example

```rust
use ucm_engine::{Engine, Operation};
use ucm_core::{Document, Content};

fn main() {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create engine
    let engine = Engine::new();
    
    // Add a block using operation
    let op = Operation::Append {
        parent_id: root,
        content: Content::text("Hello, World!"),
        label: Some("greeting".to_string()),
        tags: vec![],
        semantic_role: Some("paragraph".to_string()),
        index: None,
    };
    
    let result = engine.execute(&mut doc, op).unwrap();
    println!("Added block: {:?}", result.affected_blocks);
}
```

## Core Components

### Engine

The main engine for executing operations:

```rust
use ucm_engine::Engine;

let engine = Engine::new();
```

### Operations

All document transformations are represented as operations:

- **Edit** - Modify block content
- **Append** - Add new blocks
- **Move** - Relocate blocks
- **Delete** - Remove blocks
- **Link** - Create relationships
- **CreateSnapshot** - Save document state
- **RestoreSnapshot** - Restore document state

### Transactions

Group operations atomically:

```rust
use ucm_engine::{Transaction, TransactionManager};

let mut tx_manager = TransactionManager::new();
let tx = tx_manager.begin(&mut doc);

// Execute operations...
tx.execute(op1);
tx.execute(op2);

// Commit or rollback
tx.commit().unwrap();
// or
tx.rollback().unwrap();
```

## Public API

```rust
pub use engine::Engine;
pub use operation::{Operation, OperationResult, EditOperator, MoveTarget, PruneCondition};
pub use transaction::{Transaction, TransactionManager, TransactionState};
pub use snapshot::{Snapshot, SnapshotManager, serialize_document, deserialize_document};
pub use validation::{Validator, ValidationIssue, ValidationResult};
```

## See Also

- [Operations](../../docs/ucm-engine/operations.md) - All available operations
- [Transactions](../../docs/ucm-engine/transactions.md) - Transaction management
- [Snapshots](../../docs/ucm-engine/snapshots.md) - Version control
- [Validation](../../docs/ucm-engine/validation.md) - Document validation
