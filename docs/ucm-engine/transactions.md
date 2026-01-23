# Transactions

Transactions allow grouping multiple operations into a single atomic unit. If any operation fails, the entire transaction is rolled back.

## Transaction Structure

=== "Rust"
    ```rust
    pub struct Transaction {
        /// Unique transaction ID
        pub id: TransactionId,
        
        /// Operations in the transaction
        pub operations: Vec<Operation>,
        
        /// Transaction state (Pending, Committed, RolledBack)
        pub state: TransactionState,
        
        /// Creation timestamp
        pub created_at: DateTime<Utc>,
    }
    ```

=== "Python"
    *Transactions are managed internally by the engine but not directly exposed as objects in the high-level Python API. Use atomic batch operations via UCL.*

=== "JavaScript"
    *Transactions are managed internally by the engine but not directly exposed as objects in the high-level JavaScript API. Use atomic batch operations via UCL.*

## Basic Usage

=== "Rust"
    ```rust
    use ucm_engine::{Engine, Operation, EditOperator};
    use ucm_core::{Document, Content};

    let mut engine = Engine::new();
    let mut doc = Document::create();
    let root = doc.root.clone();

    // 1. Begin transaction
    let txn_id = engine.begin_transaction();

    // 2. Add operations
    engine.add_to_transaction(&txn_id, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("First block"),
        label: None,
        tags: vec![],
        semantic_role: None,
        index: None,
    }).unwrap();

    engine.add_to_transaction(&txn_id, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Second block"),
        label: None,
        tags: vec![],
        semantic_role: None,
        index: None,
    }).unwrap();

    // 3. Commit
    let results = engine.commit_transaction(&txn_id, &mut doc).unwrap();

    println!("Executed {} operations", results.len());
    ```

## Atomicity

If an operation fails during commit, previous operations in the transaction are rolled back:

=== "Rust"
    ```rust
    let txn_id = engine.begin_transaction();

    // Valid operation
    engine.add_to_transaction(&txn_id, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Valid"),
        label: None,
        tags: vec![],
        semantic_role: None,
        index: None,
    }).unwrap();

    // Invalid operation (reference to non-existent parent)
    engine.add_to_transaction(&txn_id, Operation::Append {
        parent_id: "blk_invalid".parse().unwrap(),
        content: Content::text("Invalid"),
        label: None,
        tags: vec![],
        semantic_role: None,
        index: None,
    }).unwrap();

    // Commit fails
    let result = engine.commit_transaction(&txn_id, &mut doc);
    assert!(result.is_err());

    // Document is unchanged (first operation was rolled back)
    assert_eq!(doc.block_count(), 1); // Only root exists
    ```

## Explicit Rollback

You can manually rollback a pending transaction:

=== "Rust"
    ```rust
    let txn_id = engine.begin_transaction();

    engine.add_to_transaction(&txn_id, Operation::Append {
        // ...
        # parent_id: root.clone(),
        # content: Content::text("Draft"),
        # label: None,
        # tags: vec![],
        # semantic_role: None,
        # index: None,
    }).unwrap();

    // Change mind
    engine.rollback_transaction(&txn_id).unwrap();

    // Transaction is now marked as RolledBack and cannot be committed
    let result = engine.commit_transaction(&txn_id, &mut doc);
    assert!(result.is_err());
    ```

## Transaction Manager

The engine uses a `TransactionManager` to track transaction lifecycle:

=== "Rust"
    ```rust
    use ucm_engine::transaction::{TransactionManager, TransactionState};

    let manager = TransactionManager::new();
    let txn_id = manager.begin();

    assert_eq!(manager.get_state(&txn_id), Some(TransactionState::Pending));
    ```

## Best Practices

### 1. Keep Transactions Short

Long-running transactions consume memory and may conflict with other operations.

### 2. Group Logically Related Changes

Use transactions for changes that must happen together, like:
- Moving a block and updating references
- Creating a section and its initial children
- Renaming a label and updating links to it

### 3. Handle Commit Failures

Always check the result of `commit_transaction`. If it fails, the document state is preserved (rolled back), but you may need to retry or report the error.

## See Also
let mut mgr = TransactionManager::with_timeout(Duration::from_secs(5));

let txn_id = mgr.begin();

// Wait too long...
std::thread::sleep(Duration::from_secs(6));

// Commit will fail
let result = mgr.commit(&txn_id);
assert!(result.is_err()); // TransactionTimeout
```

### Checking Timeout

```rust
if let Some(txn) = mgr.get(&txn_id) {
    if txn.is_timed_out() {
        println!("Transaction timed out!");
    }
}
```

## Savepoints

Savepoints allow partial rollback within a transaction:

```rust
use ucm_engine::transaction::Transaction;
use std::time::Duration;

let mut txn = Transaction::new(Duration::from_secs(30));

// Add some operations
txn.add_operation(op1).unwrap();
txn.add_operation(op2).unwrap();

// Create savepoint
txn.savepoint("before-risky-ops");

// Add more operations
txn.add_operation(op3).unwrap();
txn.add_operation(op4).unwrap();

// Savepoint records the operation index
let sp = &txn.savepoints[0];
println!("Savepoint '{}' at operation {}", sp.name, sp.operation_index);
```

## Transaction States

```
┌────────┐
│ Active │
└───┬────┘
    │
    ├─── commit() ───► Committed
    │
    ├─── rollback() ──► RolledBack
    │
    └─── timeout ─────► TimedOut
```

### State Transitions

```rust
use ucm_engine::transaction::TransactionState;

let txn_id = mgr.begin();

// Initially Active
let txn = mgr.get(&txn_id).unwrap();
assert_eq!(txn.state, TransactionState::Active);

// After commit
mgr.commit(&txn_id).unwrap();
let txn = mgr.get(&txn_id).unwrap();
assert_eq!(txn.state, TransactionState::Committed);

// Or after rollback
// mgr.rollback(&txn_id).unwrap();
// assert_eq!(txn.state, TransactionState::RolledBack);
```

## Error Handling

### Adding to Non-Active Transaction

```rust
let txn_id = mgr.begin();
mgr.commit(&txn_id).unwrap();

// Can't add to committed transaction
let result = mgr.add_operation(&txn_id, operation);
assert!(result.is_err());
```

### Committing Non-Active Transaction

```rust
let txn_id = mgr.begin();
mgr.rollback(&txn_id).unwrap();

// Can't commit rolled back transaction
let result = mgr.commit(&txn_id);
assert!(result.is_err());
```

### Transaction Not Found

```rust
let fake_id = TransactionId::generate();
let result = mgr.commit(&fake_id);
assert!(result.is_err()); // TransactionNotFound
```

## Using with Engine

The `Engine` provides convenient transaction methods:

```rust
use ucm_engine::Engine;

let mut engine = Engine::new();
let mut doc = Document::create();

// Begin
let txn_id = engine.begin_transaction();
// or: engine.begin_named_transaction("my-txn")

// Add operations
engine.add_to_transaction(&txn_id, op1)?;
engine.add_to_transaction(&txn_id, op2)?;

// Commit (executes operations)
let results = engine.commit_transaction(&txn_id, &mut doc)?;

// Or rollback
// engine.rollback_transaction(&txn_id)?;
```

## Complete Example

```rust
use ucm_engine::{Engine, Operation, EditOperator};
use ucm_core::{Content, Document, EdgeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Transaction 1: Add structure
    let txn1 = engine.begin_named_transaction("add-structure");
    
    engine.add_to_transaction(&txn1, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Chapter 1"),
        label: Some("chapter-1".into()),
        tags: vec!["chapter".into()],
        semantic_role: Some("heading1".into()),
        index: None,
    })?;
    
    engine.add_to_transaction(&txn1, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Chapter 2"),
        label: Some("chapter-2".into()),
        tags: vec!["chapter".into()],
        semantic_role: Some("heading1".into()),
        index: None,
    })?;
    
    let results = engine.commit_transaction(&txn1, &mut doc)?;
    println!("Added {} blocks", results.len());
    
    let chapter1_id = results[0].affected_blocks[0].clone();
    let chapter2_id = results[1].affected_blocks[0].clone();
    
    // Transaction 2: Add content to chapters
    let txn2 = engine.begin_named_transaction("add-content");
    
    engine.add_to_transaction(&txn2, Operation::Append {
        parent_id: chapter1_id.clone(),
        content: Content::text("Introduction to the topic."),
        label: None,
        tags: vec![],
        semantic_role: Some("paragraph".into()),
        index: None,
    })?;
    
    engine.add_to_transaction(&txn2, Operation::Append {
        parent_id: chapter2_id.clone(),
        content: Content::text("Advanced concepts."),
        label: None,
        tags: vec![],
        semantic_role: Some("paragraph".into()),
        index: None,
    })?;
    
    // Add cross-reference
    engine.add_to_transaction(&txn2, Operation::Link {
        source: chapter2_id.clone(),
        edge_type: EdgeType::References,
        target: chapter1_id.clone(),
        metadata: None,
    })?;
    
    let results = engine.commit_transaction(&txn2, &mut doc)?;
    println!("Transaction 2: {} operations", results.len());
    
    // Transaction 3: Demonstrate rollback
    let txn3 = engine.begin_transaction();
    
    engine.add_to_transaction(&txn3, Operation::Delete {
        block_id: chapter1_id.clone(),
        cascade: true,
        preserve_children: false,
    })?;
    
    // Decide not to delete
    engine.rollback_transaction(&txn3)?;
    println!("Rolled back deletion");
    
    // Chapter 1 still exists
    assert!(doc.get_block(&chapter1_id).is_some());
    
    println!("Final document has {} blocks", doc.block_count());
    
    Ok(())
}
```

## Best Practices

### 1. Use Named Transactions for Clarity

```rust
// Good - descriptive name
let txn = engine.begin_named_transaction("import-chapter-3");

// Less ideal - anonymous
let txn = engine.begin_transaction();
```

### 2. Keep Transactions Short

```rust
// Good - focused transaction
let txn = engine.begin_transaction();
engine.add_to_transaction(&txn, op1)?;
engine.add_to_transaction(&txn, op2)?;
engine.commit_transaction(&txn, &mut doc)?;

// Less ideal - long-running transaction
let txn = engine.begin_transaction();
// ... many operations over time ...
// Risk of timeout
```

### 3. Handle Errors Appropriately

```rust
let txn = engine.begin_transaction();

match engine.add_to_transaction(&txn, operation) {
    Ok(_) => {},
    Err(e) => {
        engine.rollback_transaction(&txn)?;
        return Err(e.into());
    }
}

match engine.commit_transaction(&txn, &mut doc) {
    Ok(results) => {
        // Check individual results
        for result in &results {
            if !result.success {
                println!("Warning: {:?}", result.error);
            }
        }
    }
    Err(e) => {
        // Transaction failed
        return Err(e.into());
    }
}
```

### 4. Clean Up Periodically

```rust
// In long-running applications
mgr.cleanup(); // Remove completed/timed out transactions
```

## See Also

- [Operations](./operations.md) - Operation types
- [Snapshots](./snapshots.md) - Document versioning
- [UCL Commands](../ucl-parser/commands.md) - Transaction commands in UCL
