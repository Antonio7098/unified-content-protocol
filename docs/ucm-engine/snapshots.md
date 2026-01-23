# Snapshots

Snapshots provide document versioning, allowing you to save and restore document states.

## Snapshot Structure

=== "Rust"
    ```rust
    pub struct Snapshot {
        pub id: SnapshotId,
        pub description: Option<String>,
        pub created_at: DateTime<Utc>,
        pub document_version: DocumentVersion,
        pub data: SnapshotData,
    }

    pub enum SnapshotData {
        Full(SerializedDocument),
        Delta { base: SnapshotId, changes: Vec<SnapshotChange> },
    }
    ```

=== "Python"
    ```python
    class SnapshotInfo:
        @property
        def name(self) -> str: ...
        
        @property
        def description(self) -> Optional[str]: ...
        
        @property
        def created_at(self) -> str: ...
        
        @property
        def version(self) -> int: ...
    ```

=== "JavaScript"
    ```typescript
    interface SnapshotInfo {
        name: string;
        description?: string;
        createdAt: string;
        version: number;
    }
    ```

## Basic Usage

### Create Snapshot

=== "Rust"
    ```rust
    use ucm_engine::Engine;
    use ucm_core::Document;

    let mut engine = Engine::new();
    let doc = Document::create();

    // Create snapshot
    engine.create_snapshot("v1", &doc, Some("Initial version".into())).unwrap();

    // Create without description
    engine.create_snapshot("v2", &doc, None).unwrap();
    ```

=== "Python"
    ```python
    from ucp_content import SnapshotManager, Document
    
    doc = Document.create()
    manager = SnapshotManager()
    
    # Create snapshot
    manager.create("v1", doc, description="Initial version")
    
    # Create without description
    manager.create("v2", doc)
    ```

=== "JavaScript"
    ```javascript
    import { Document, SnapshotManager } from 'ucp-content';
    
    const doc = Document.create();
    const manager = new SnapshotManager();
    
    // Create snapshot
    manager.create("v1", doc, "Initial version");
    
    // Create without description
    manager.create("v2", doc);
    ```

### Restore Snapshot

=== "Rust"
    ```rust
    // Make changes to document...

    // Restore to previous state
    let restored_doc = engine.restore_snapshot("v1").unwrap();

    println!("Restored document has {} blocks", restored_doc.block_count());
    ```

=== "Python"
    ```python
    # Make changes...
    
    # Restore
    restored_doc = manager.restore("v1")
    print(f"Restored document has {restored_doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    // Make changes...
    
    // Restore
    const restoredDoc = manager.restore("v1");
    console.log(`Restored document has ${restoredDoc.blockCount()} blocks`);
    ```

### List Snapshots

=== "Rust"
    ```rust
    let snapshots = engine.list_snapshots();
    for name in snapshots {
        println!("Snapshot: {}", name);
    }
    ```

=== "Python"
    ```python
    snapshots = manager.list()
    for snap in snapshots:
        print(f"Snapshot: {snap.name}")
    ```

=== "JavaScript"
    ```javascript
    const snapshots = manager.list();
    for (const snap of snapshots) {
        console.log(`Snapshot: ${snap.name}`);
    }
    ```

### Delete Snapshot

=== "Rust"
    ```rust
    let deleted = engine.delete_snapshot("v1");
    if deleted {
        println!("Snapshot deleted");
    } else {
        println!("Snapshot not found");
    }
    ```

=== "Python"
    ```python
    if manager.delete("v1"):
        print("Snapshot deleted")
    else:
        print("Snapshot not found")
    ```

=== "JavaScript"
    ```javascript
    if (manager.delete("v1")) {
        console.log("Snapshot deleted");
    } else {
        console.log("Snapshot not found");
    }
    ```

## Snapshot Manager

For direct snapshot management:

=== "Rust"
    ```rust
    use ucm_engine::snapshot::{SnapshotManager, SnapshotId};

    // Create manager
    let mut mgr = SnapshotManager::new();

    // With max snapshots limit
    let mut mgr = SnapshotManager::with_max_snapshots(10);

    // Create snapshot
    let id = mgr.create("v1", &doc, Some("Description".into())).unwrap();

    // Check existence
    if mgr.exists("v1") {
        println!("Snapshot exists");
    }

    // Get snapshot metadata
    if let Some(snapshot) = mgr.get("v1") {
        println!("Created: {}", snapshot.created_at);
        println!("Description: {:?}", snapshot.description);
    }

    // List all snapshots (sorted by creation time, newest first)
    let snapshots = mgr.list();
    for snapshot in snapshots {
        println!("{}: {:?}", snapshot.id, snapshot.description);
    }

    // Restore
    let doc = mgr.restore("v1").unwrap();

    // Delete
    mgr.delete("v1");

    // Count
    println!("Total snapshots: {}", mgr.count());
    ```

=== "Python"
    ```python
    from ucp_content import SnapshotManager
    
    # Create manager (default max_snapshots=None)
    mgr = SnapshotManager()
    
    # With limit
    mgr = SnapshotManager(max_snapshots=10)
    
    # Create
    mgr.create("v1", doc, "Description")
    
    # Check existence
    if mgr.exists("v1"):
        print("Snapshot exists")
        
    # Get metadata
    snap = mgr.get("v1")
    if snap:
        print(f"Created: {snap.created_at}")
        print(f"Description: {snap.description}")
        
    # List
    for snap in mgr.list():
        print(f"{snap.name}: {snap.description}")
        
    # Restore
    doc = mgr.restore("v1")
    
    # Delete
    mgr.delete("v1")
    
    # Count
    print(f"Total snapshots: {len(mgr)}")
    ```

=== "JavaScript"
    ```javascript
    import { SnapshotManager } from 'ucp-content';
    
    // Create manager
    const mgr = new SnapshotManager();
    
    // With limit
    const limitedMgr = new SnapshotManager(10);
    
    // Create
    mgr.create("v1", doc, "Description");
    
    // Check existence
    if (mgr.exists("v1")) {
        console.log("Snapshot exists");
    }
    
    // Get metadata
    const snap = mgr.get("v1");
    if (snap) {
        console.log(`Created: ${snap.createdAt}`);
        console.log(`Description: ${snap.description}`);
    }
    
    // List
    for (const s of mgr.list()) {
        console.log(`${s.name}: ${s.description}`);
    }
    
    // Restore
    const restoredDoc = mgr.restore("v1");
    
    // Delete
    mgr.delete("v1");
    
    // Count
    console.log(`Total snapshots: ${mgr.length}`);
    ```

## Automatic Eviction

When the snapshot limit is reached, the oldest snapshot is automatically evicted:

=== "Rust"
    ```rust
    let mut mgr = SnapshotManager::with_max_snapshots(3);

    mgr.create("v1", &doc, None).unwrap();
    mgr.create("v2", &doc, None).unwrap();
    mgr.create("v3", &doc, None).unwrap();

    // This will evict v1
    mgr.create("v4", &doc, None).unwrap();

    assert!(!mgr.exists("v1")); // Evicted
    assert!(mgr.exists("v2"));
    assert!(mgr.exists("v3"));
    assert!(mgr.exists("v4"));
    ```

## Snapshot Workflow

### Version Control Pattern

=== "Rust"
    ```rust
    use ucm_engine::Engine;
    use ucm_core::Document;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = Engine::new();
        let mut doc = Document::create();
        
        // Initial state
        engine.create_snapshot("initial", &doc, Some("Empty document".into()))?;
        
        // Add content
        // ... operations ...
        engine.create_snapshot("draft-1", &doc, Some("First draft".into()))?;
        
        // More changes
        // ... operations ...
        engine.create_snapshot("draft-2", &doc, Some("Second draft".into()))?;
        
        // Oops, draft-2 was bad, restore draft-1
        doc = engine.restore_snapshot("draft-1")?;
        
        // Continue from draft-1
        // ... operations ...
        engine.create_snapshot("final", &doc, Some("Final version".into()))?;
        
        Ok(())
    }
    ```

### Checkpoint Pattern

=== "Rust"
    ```rust
    // Before risky operation
    engine.create_snapshot("before-refactor", &doc, None)?;

    // Attempt refactor
    let result = perform_risky_refactor(&mut doc);

    if result.is_err() {
        // Restore on failure
        doc = engine.restore_snapshot("before-refactor")?;
        engine.delete_snapshot("before-refactor");
        return Err(result.unwrap_err());
    }

    // Success - clean up checkpoint
    engine.delete_snapshot("before-refactor");
    ```

### A/B Testing Pattern

=== "Rust"
    ```rust
    // Create base snapshot
    engine.create_snapshot("base", &doc, None)?;

    // Version A
    let mut doc_a = engine.restore_snapshot("base")?;
    apply_version_a(&mut doc_a);
    engine.create_snapshot("version-a", &doc_a, Some("Version A".into()))?;

    // Version B
    let mut doc_b = engine.restore_snapshot("base")?;
    apply_version_b(&mut doc_b);
    engine.create_snapshot("version-b", &doc_b, Some("Version B".into()))?;

    // Compare or choose
    let chosen = engine.restore_snapshot("version-a")?;
    ```

## Serialization Details

Snapshots serialize the complete document state:

=== "Rust"
    ```rust
    pub struct SerializedDocument {
        pub json: String,
    }
    ```

The serialized format includes:
- Document ID
- Root block ID
- Structure (adjacency map)
- All blocks (with content and metadata)
- Document metadata
- Document version

## Error Handling

### Snapshot Not Found

=== "Rust"
    ```rust
    let result = engine.restore_snapshot("nonexistent");
    assert!(result.is_err());
    ```

=== "Python"
    ```python
    try:
        manager.restore("nonexistent")
    except KeyError:
        print("Snapshot not found")
    ```

=== "JavaScript"
    ```javascript
    try {
        manager.restore("nonexistent");
    } catch (e) {
        console.log("Snapshot not found");
    }
    ```

### Serialization Errors

=== "Rust"
    ```rust
    // Rare, but possible with custom content
    let result = mgr.create("snapshot", &doc, None);
    if let Err(e) = result {
        eprintln!("Failed to create snapshot: {}", e);
    }
    ```

## Complete Example

=== "Rust"
    ```rust
    use ucm_engine::{Engine, Operation};
    use ucm_core::{Content, Document};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = Engine::new();
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Create initial snapshot
        engine.create_snapshot("empty", &doc, Some("Empty document".into()))?;
        
        // Add chapter 1
        engine.execute(&mut doc, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Chapter 1: Introduction"),
            label: Some("chapter-1".into()),
            tags: vec![],
            semantic_role: Some("heading1".into()),
            index: None,
        })?;
        
        engine.create_snapshot("chapter-1", &doc, Some("Added chapter 1".into()))?;
        
        // Add chapter 2
        engine.execute(&mut doc, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Chapter 2: Details"),
            label: Some("chapter-2".into()),
            tags: vec![],
            semantic_role: Some("heading1".into()),
            index: None,
        })?;
        
        engine.create_snapshot("chapter-2", &doc, Some("Added chapter 2".into()))?;
        
        // List snapshots
        println!("Snapshots:");
        for name in engine.list_snapshots() {
            println!("  - {}", name);
        }
        
        // Current state
        println!("\nCurrent: {} blocks", doc.block_count());
        
        // Restore to chapter-1
        doc = engine.restore_snapshot("chapter-1")?;
        println!("After restore to chapter-1: {} blocks", doc.block_count());
        
        // Restore to empty
        doc = engine.restore_snapshot("empty")?;
        println!("After restore to empty: {} blocks", doc.block_count());
        
        // Clean up
        engine.delete_snapshot("empty");
        engine.delete_snapshot("chapter-1");
        engine.delete_snapshot("chapter-2");
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, SnapshotManager
    
    doc = Document.create()
    root = doc.root_id
    manager = SnapshotManager()
    
    # Initial
    manager.create("empty", doc, "Empty document")
    
    # Add chapter 1
    doc.add_block(root, "Chapter 1: Introduction", role="heading1")
    manager.create("chapter-1", doc, "Added chapter 1")
    
    # Add chapter 2
    doc.add_block(root, "Chapter 2: Details", role="heading1")
    manager.create("chapter-2", doc, "Added chapter 2")
    
    # List
    print("Snapshots:")
    for s in manager.list():
        print(f"  - {s.name}")
        
    print(f"\nCurrent: {doc.block_count()} blocks")
    
    # Restore
    doc = manager.restore("chapter-1")
    print(f"After restore to chapter-1: {doc.block_count()} blocks")
    
    doc = manager.restore("empty")
    print(f"After restore to empty: {doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    import { Document, SnapshotManager } from 'ucp-content';
    
    const doc = Document.create();
    const root = doc.rootId;
    const manager = new SnapshotManager();
    
    // Initial
    manager.create("empty", doc, "Empty document");
    
    // Add chapter 1
    doc.addBlock(root, "Chapter 1: Introduction", "heading1");
    manager.create("chapter-1", doc, "Added chapter 1");
    
    // Add chapter 2
    doc.addBlock(root, "Chapter 2: Details", "heading1");
    manager.create("chapter-2", doc, "Added chapter 2");
    
    // List
    console.log("Snapshots:");
    for (const s of manager.list()) {
        console.log(`  - ${s.name}`);
    }
    
    console.log(`\nCurrent: ${doc.blockCount()} blocks`);
    
    // Restore
    let restored = manager.restore("chapter-1");
    console.log(`After restore to chapter-1: ${restored.blockCount()} blocks`);
    
    restored = manager.restore("empty");
    console.log(`After restore to empty: ${restored.blockCount()} blocks`);
    ```

## Best Practices

### 1. Use Descriptive Names

=== "Rust"
    ```rust
    // Good - descriptive
    engine.create_snapshot("before-migration-v2", &doc, None)?;
    engine.create_snapshot("after-review-alice", &doc, None)?;

    // Less ideal - generic
    engine.create_snapshot("v1", &doc, None)?;
    engine.create_snapshot("backup", &doc, None)?;
    ```

### 2. Add Descriptions

=== "Rust"
    ```rust
    engine.create_snapshot(
        "release-1.0",
        &doc,
        Some("Release 1.0 - reviewed and approved".into())
    )?;
    ```

### 3. Clean Up Old Snapshots

=== "Rust"
    ```rust
    // After successful operation
    engine.delete_snapshot("checkpoint");

    // Or use max_snapshots limit
    let mgr = SnapshotManager::with_max_snapshots(10);
    ```

### 4. Use Checkpoints for Risky Operations

=== "Rust"
    ```rust
    engine.create_snapshot("checkpoint", &doc, None)?;

    match risky_operation(&mut doc) {
        Ok(_) => {
            engine.delete_snapshot("checkpoint");
        }
        Err(e) => {
            doc = engine.restore_snapshot("checkpoint")?;
            engine.delete_snapshot("checkpoint");
            return Err(e);
        }
    }
    ```

### 5. Consider Snapshot Size

Snapshots store the full document. For large documents:
- Limit the number of snapshots
- Delete unnecessary snapshots promptly
- Consider external storage for long-term versioning

## See Also

- [Operations](./operations.md) - Document operations
- [Transactions](./transactions.md) - Atomic operations
- [UCL Commands](../ucl-parser/commands.md) - SNAPSHOT commands

