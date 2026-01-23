# Operations

Operations are the fundamental units of change in UCM. The engine executes operations to modify documents.

## Operation Types

=== "Rust"
    ```rust
    pub enum Operation {
        Edit { block_id, path, value, operator },
        Move { block_id, new_parent, index },
        Append { parent_id, content, label, tags, semantic_role, index },
        Delete { block_id, cascade, preserve_children },
        Prune { condition },
        Link { source, edge_type, target, metadata },
        Unlink { source, edge_type, target },
        CreateSnapshot { name, description },
        RestoreSnapshot { name },
        WriteSection { section_id, markdown, base_heading_level },
    }
    ```

=== "Python"
    *Operations are internal to the engine but are exposed through `Document` methods and UCL execution.*

=== "JavaScript"
    *Operations are internal to the engine but are exposed through `Document` methods and UCL execution.*

## Edit Operation

Modify block content or metadata.

### Structure

=== "Rust"
    ```rust
    Operation::Edit {
        block_id: BlockId,
        path: String,
        value: serde_json::Value,
        operator: EditOperator,
    }
    ```

### Edit Operators

| Operator | Operation | Description |
|-----------|-------------|-------------|
| `Edit` | Modify block content or metadata |
| `Move` | Move block to new parent |
| `Append` | Add new block |
| `Delete` | Remove block |
| `Prune` | Remove unreachable blocks |
| `Link` | Add edge between blocks |
| `Unlink` | Remove edge |
| `CreateSnapshot` | Create document snapshot |
| `RestoreSnapshot` | Restore from snapshot |
| `WriteSection` | Replace a section's children from Markdown with optional heading offset and undo |

### Examples

=== "Rust"
    ```rust
    use ucm_engine::{Engine, Operation, EditOperator};
    use ucm_core::{Document, Block, Content};

    let engine = Engine::new();
    let mut doc = Document::create();
    let root = doc.root.clone();

    // Add a block first
    let block = Block::new(Content::text("Original text"), None);
    let block_id = doc.add_block(block, &root).unwrap();

    // Edit content
    let result = engine.execute(&mut doc, Operation::Edit {
        block_id: block_id.clone(),
        path: "content.text".to_string(),
        value: serde_json::json!("Updated text"),
        operator: EditOperator::Set,
    }).unwrap();

    // Append to content
    let result = engine.execute(&mut doc, Operation::Edit {
        block_id: block_id.clone(),
        path: "content.text".to_string(),
        value: serde_json::json!(" - more text"),
        operator: EditOperator::Append,
    }).unwrap();

    // Edit metadata
    let result = engine.execute(&mut doc, Operation::Edit {
        block_id: block_id.clone(),
        path: "metadata.label".to_string(),
        value: serde_json::json!("my-label"),
        operator: EditOperator::Set,
    }).unwrap();

    // Add tags
    let result = engine.execute(&mut doc, Operation::Edit {
        block_id: block_id.clone(),
        path: "metadata.tags".to_string(),
        value: serde_json::json!(["important", "draft"]),
        operator: EditOperator::Append,
    }).unwrap();

    // Remove tag
    let result = engine.execute(&mut doc, Operation::Edit {
        block_id: block_id.clone(),
        path: "metadata.tags".to_string(),
        value: serde_json::json!("draft"),
        operator: EditOperator::Remove,
    }).unwrap();
    ```

=== "Python"
    ```python
    # Edit content
    doc.edit_block(block_id, "Updated text")
    
    # Edit metadata
    doc.set_label(block_id, "my-label")
    doc.add_tag(block_id, "important")
    doc.remove_tag(block_id, "draft")
    ```

=== "JavaScript"
    ```javascript
    // Edit content
    doc.editBlock(blockId, "Updated text");
    
    // Edit metadata
    doc.setLabel(blockId, "my-label");
    doc.addTag(blockId, "important");
    doc.removeTag(blockId, "draft");
    ```

## WriteSection Operation

Replace an entire section's children with parsed Markdown while capturing an undo payload.

### Structure

=== "Rust"
    ```rust
    Operation::WriteSection {
        section_id: BlockId,
        markdown: String,
        base_heading_level: Option<usize>,
    }
    ```

### Behavior

1. Calls `clear_section_content_with_undo` to remove the section's descendants and produce a `ClearSectionResult` containing `removed_ids` and a `DeletedContent` snapshot (blocks + structure + parent metadata).
2. Parses the supplied Markdown using `ucp-translator-markdown` and integrates it beneath `section_id`. When `base_heading_level` is set, each heading is re-based (e.g., `Some(3)` promotes the inserted top-level heading to `###`).
3. Returns an `OperationResult` whose `affected_blocks` include both deleted and newly added block IDs so downstream systems can update caches.

### Undo Workflow

Persist `DeletedContent` if you want a full rollback. Restoring first clears whatever content currently resides under the section (including manual edits after the write) and then reattaches the preserved subtree.

=== "Rust"
    ```rust
    use ucm_engine::section::{clear_section_content_with_undo, restore_deleted_content};

    let result = clear_section_content_with_undo(&mut doc, &section_id)?;
    let snapshot = result.deleted_content;

    // ... Write new markdown or perform edits ...

    let restored_ids = restore_deleted_content(&mut doc, &snapshot)?;
    assert_eq!(restored_ids.len(), result.removed_ids.len());
    ```

### SDK Support

- **Python**: `ucp.clear_section_with_undo(doc, section_id)` and `ucp.restore_deleted_section(doc, deleted_content)` mirror the Rust helpers. Restoration always removes the "replacement" subtree before reattaching the saved blocks.
- **JavaScript**: `clearSectionWithUndo(doc, sectionId)` returns `{ removedIds, deletedContent }` and `restoreDeletedSection(doc, deletedContent)` restores it.

### Convenience API

The `Document` class provides a direct method for writing sections:

=== "Python"
    ```python
    # Write markdown to a section
    result = doc.write_section(section_id, "## New Content\n\nThis replaces old content.")
    
    if result.success:
        print(f"Replaced {len(result.blocks_removed)} blocks with {len(result.blocks_added)} blocks")
    ```

=== "JavaScript"
    ```javascript
    // Write markdown to a section
    const result = doc.writeSection(sectionId, "## New Content\n\nThis replaces old content.");
    
    if (result.success) {
        console.log(`Replaced ${result.blocksRemoved.length} blocks with ${result.blocksAdded.length} blocks`);
    }
    ```

The deleted payload is pure JSON (blocks, structure, parent metadata), so you can persist it in durable storage to enable long-lived undo stacks.

## Move Operation

Move a block to a new parent.

### Structure

=== "Rust"
    ```rust
    Operation::Move {
        block_id: BlockId,
        new_parent: BlockId,
        index: Option<usize>,
    }
    ```

### Examples

=== "Rust"
    ```rust
    // Move to end of new parent's children
    let result = engine.execute(&mut doc, Operation::Move {
        block_id: child_id.clone(),
        new_parent: new_parent_id.clone(),
        index: None,
    }).unwrap();

    // Move to specific position
    let result = engine.execute(&mut doc, Operation::Move {
        block_id: child_id.clone(),
        new_parent: new_parent_id.clone(),
        index: Some(0), // First position
    }).unwrap();
    ```

=== "Python"
    ```python
    doc.move_block(child_id, new_parent_id)
    doc.move_block(child_id, new_parent_id, index=0)
    ```

=== "JavaScript"
    ```javascript
    doc.moveBlock(childId, newParentId);
    doc.moveBlock(childId, newParentId, 0);
    ```

### Cycle Detection

Moving a block to one of its descendants is prevented:

=== "Rust"
    ```rust
    // This will fail with CycleDetected error
    let result = engine.execute(&mut doc, Operation::Move {
        block_id: parent_id,
        new_parent: child_id, // child is descendant of parent
        index: None,
    });
    assert!(!result.unwrap().success);
    ```

## Append Operation

Add a new block to the document.

### Structure

=== "Rust"
    ```rust
    Operation::Append {
        parent_id: BlockId,
        content: Content,
        label: Option<String>,
        tags: Vec<String>,
        semantic_role: Option<String>,
        index: Option<usize>,
    }
    ```

### Examples

=== "Rust"
    ```rust
    use ucm_core::Content;

    // Append text block
    let result = engine.execute(&mut doc, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("New paragraph"),
        label: Some("intro-para".into()),
        tags: vec!["introduction".into()],
        semantic_role: Some("paragraph".into()),
        index: None,
    }).unwrap();

    let new_block_id = &result.affected_blocks[0];

    // Append code block
    let result = engine.execute(&mut doc, Operation::Append {
        parent_id: root.clone(),
        content: Content::code("rust", "fn main() {}"),
        label: Some("example-1".into()),
        tags: vec!["example".into(), "rust".into()],
        semantic_role: Some("code".into()),
        index: None,
    }).unwrap();

    // Append at specific position
    let result = engine.execute(&mut doc, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("First!"),
        label: None,
        tags: vec![],
        semantic_role: None,
        index: Some(0), // Insert at beginning
    }).unwrap();
    ```

=== "Python"
    ```python
    # Append text block
    block_id = doc.add_block(
        root_id, 
        "New paragraph", 
        label="intro-para", 
        tags=["introduction"],
        role="paragraph"
    )
    
    # Append code block
    code_id = doc.add_code(
        root_id,
        "rust",
        "fn main() {}",
        label="example-1"
    )
    
    # Append at position
    doc.add_block_with_content(
        root_id, 
        Content.text("First!"), 
        index=0
    )
    ```

=== "JavaScript"
    ```javascript
    // Append text block
    const blockId = doc.addBlock(
        rootId, 
        "New paragraph", 
        "paragraph",
        "intro-para"
    );
    doc.addTag(blockId, "introduction");
    
    // Append code block
    const codeId = doc.addCode(
        rootId,
        "rust",
        "fn main() {}",
        "example-1"
    );
    
    // Append at position
    doc.addBlockAt(rootId, "First!", 0);
    ```

## Delete Operation

Remove a block from the document.

### Structure

=== "Rust"
    ```rust
    Operation::Delete {
        block_id: BlockId,
        cascade: bool,
        preserve_children: bool,
    }
    ```

### Options

| Option | Description |
|--------|-------------|
| `cascade: false` | Delete only the specified block |
| `cascade: true` | Delete block and all descendants |
| `preserve_children: true` | Reparent children to grandparent before delete |

### Examples

=== "Rust"
    ```rust
    // Delete single block (children become orphaned)
    let result = engine.execute(&mut doc, Operation::Delete {
        block_id: block_id.clone(),
        cascade: false,
        preserve_children: false,
    }).unwrap();

    // Delete with all descendants
    let result = engine.execute(&mut doc, Operation::Delete {
        block_id: block_id.clone(),
        cascade: true,
        preserve_children: false,
    }).unwrap();

    // Delete but keep children (reparent to grandparent)
    let result = engine.execute(&mut doc, Operation::Delete {
        block_id: block_id.clone(),
        cascade: false,
        preserve_children: true,
    }).unwrap();
    ```

=== "Python"
    ```python
    # Delete single block
    doc.delete_block(block_id)
    
    # Delete with cascade
    doc.delete_block(block_id, cascade=True)
    ```

=== "JavaScript"
    ```javascript
    // Delete single block
    doc.deleteBlock(blockId);
    
    // Delete with cascade
    doc.deleteBlock(blockId, true);
    ```

## Prune Operation

Remove unreachable blocks or blocks matching a condition.

### Structure

=== "Rust"
    ```rust
    Operation::Prune {
        condition: Option<PruneCondition>,
    }

    pub enum PruneCondition {
        Unreachable,
        TagContains(String),
        Custom(String),
    }
    ```

### Examples

=== "Rust"
    ```rust
    // Prune all unreachable blocks
    let result = engine.execute(&mut doc, Operation::Prune {
        condition: None, // Defaults to Unreachable
    }).unwrap();

    // Explicitly prune unreachable
    let result = engine.execute(&mut doc, Operation::Prune {
        condition: Some(PruneCondition::Unreachable),
    }).unwrap();

    // Prune blocks with specific tag
    let result = engine.execute(&mut doc, Operation::Prune {
        condition: Some(PruneCondition::TagContains("deprecated".into())),
    }).unwrap();

    println!("Pruned {} blocks", result.affected_blocks.len());
    ```

=== "Python"
    ```python
    pruned = doc.prune_unreachable()
    print(f"Pruned {len(pruned)} blocks")
    ```

=== "JavaScript"
    ```javascript
    const pruned = doc.pruneUnreachable();
    console.log(`Pruned ${pruned.length} blocks`);
    ```

## Link Operation

Add an edge between blocks.

### Structure

=== "Rust"
    ```rust
    Operation::Link {
        source: BlockId,
        edge_type: EdgeType,
        target: BlockId,
        metadata: Option<serde_json::Value>,
    }
    ```

### Examples

=== "Rust"
    ```rust
    use ucm_core::EdgeType;

    // Add reference
    let result = engine.execute(&mut doc, Operation::Link {
        source: source_id.clone(),
        edge_type: EdgeType::References,
        target: target_id.clone(),
        metadata: None,
    }).unwrap();

    // Add with metadata
    let result = engine.execute(&mut doc, Operation::Link {
        source: source_id.clone(),
        edge_type: EdgeType::Supports,
        target: claim_id.clone(),
        metadata: Some(serde_json::json!({
            "confidence": 0.95,
            "reason": "Direct evidence"
        })),
    }).unwrap();

    // Custom edge type
    let result = engine.execute(&mut doc, Operation::Link {
        source: impl_id.clone(),
        edge_type: EdgeType::Custom("implements".into()),
        target: interface_id.clone(),
        metadata: None,
    }).unwrap();
    ```

=== "Python"
    ```python
    from ucp_content import EdgeType
    
    doc.add_edge(source_id, EdgeType.References, target_id)
    ```

=== "JavaScript"
    ```javascript
    import { EdgeType } from 'ucp-content';
    
    doc.addEdge(sourceId, EdgeType.References, targetId);
    ```

## Unlink Operation

Remove an edge between blocks.

### Structure

=== "Rust"
    ```rust
    Operation::Unlink {
        source: BlockId,
        edge_type: EdgeType,
        target: BlockId,
    }
    ```

### Examples

=== "Rust"
    ```rust
    let result = engine.execute(&mut doc, Operation::Unlink {
        source: source_id.clone(),
        edge_type: EdgeType::References,
        target: target_id.clone(),
    }).unwrap();

    if result.success {
        println!("Edge removed");
    } else {
        println!("Edge not found");
    }
    ```

=== "Python"
    ```python
    doc.remove_edge(source_id, EdgeType.References, target_id)
    ```

=== "JavaScript"
    ```javascript
    doc.removeEdge(sourceId, EdgeType.References, targetId);
    ```

## Operation Results

=== "Rust"
    ```rust
    pub struct OperationResult {
        /// Whether the operation succeeded
        pub success: bool,
        
        /// Affected block IDs
        pub affected_blocks: Vec<BlockId>,
        
        /// Any warnings generated
        pub warnings: Vec<String>,
        
        /// Error message if failed
        pub error: Option<String>,
    }
    ```

### Handling Results

=== "Rust"
    ```rust
    let result = engine.execute(&mut doc, operation).unwrap();

    if result.success {
        println!("Success! Affected blocks:");
        for block_id in &result.affected_blocks {
            println!("  - {}", block_id);
        }
        
        for warning in &result.warnings {
            println!("Warning: {}", warning);
        }
    } else {
        eprintln!("Failed: {}", result.error.unwrap_or_default());
    }
    ```

## Batch Execution

Execute multiple operations:

=== "Rust"
    ```rust
    let ops = vec![
        Operation::Append { ... },
        Operation::Edit { ... },
        Operation::Link { ... },
    ];

    let results = engine.execute_batch(&mut doc, ops).unwrap();

    for (i, result) in results.iter().enumerate() {
        if result.success {
            println!("Operation {} succeeded", i);
        } else {
            println!("Operation {} failed: {:?}", i, result.error);
            break; // Batch stops on first failure
        }
    }
    ```

=== "Python"
    Batch execution is supported via UCL commands:
    
    ```python
    from ucp_content import execute_ucl
    
    execute_ucl(doc, """
        APPEND root "New block"
        EDIT blk_123 SET content.text = "Updated"
    """)
    ```

=== "JavaScript"
    Batch execution is supported via UCL commands:
    
    ```javascript
    import { executeUcl } from 'ucp-content';
    
    executeUcl(doc, `
        APPEND root "New block"
        EDIT blk_123 SET content.text = "Updated"
    `);
    ```

### Batch Limits

=== "Rust"
    ```rust
    let config = EngineConfig {
        max_batch_size: 1000, // Limit batch size
        ..Default::default()
    };

    let engine = Engine::with_config(config);

    // This will fail if ops.len() > 1000
    let results = engine.execute_batch(&mut doc, ops);
    ```

## Operation Descriptions

Operations provide descriptions for logging:

=== "Rust"
    ```rust
    let op = Operation::Edit {
        block_id: id.clone(),
        path: "content.text".into(),
        value: serde_json::json!("new"),
        operator: EditOperator::Set,
    };

    println!("{}", op.description()); // "EDIT blk_... SET content.text"
    ```

## Complete Example

=== "Rust"
    ```rust
    use ucm_engine::{Engine, Operation, EditOperator, PruneCondition};
    use ucm_core::{Content, Document, EdgeType};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let engine = Engine::new();
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Build document structure
        let result = engine.execute(&mut doc, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Introduction"),
            label: Some("intro".into()),
            tags: vec!["section".into()],
            semantic_role: Some("heading1".into()),
            index: None,
        })?;
        let intro_id = result.affected_blocks[0].clone();
        
        let result = engine.execute(&mut doc, Operation::Append {
            parent_id: intro_id.clone(),
            content: Content::text("Welcome to the guide."),
            label: None,
            tags: vec![],
            semantic_role: Some("paragraph".into()),
            index: None,
        })?;
        let para_id = result.affected_blocks[0].clone();
        
        let result = engine.execute(&mut doc, Operation::Append {
            parent_id: root.clone(),
            content: Content::code("rust", "fn main() {\n    println!(\"Hello!\");\n}"),
            label: Some("example-1".into()),
            tags: vec!["example".into()],
            semantic_role: Some("code".into()),
            index: None,
        })?;
        let code_id = result.affected_blocks[0].clone();
        
        // Add reference from paragraph to code
        engine.execute(&mut doc, Operation::Link {
            source: para_id.clone(),
            edge_type: EdgeType::References,
            target: code_id.clone(),
            metadata: None,
        })?;
        
        // Edit the paragraph
        engine.execute(&mut doc, Operation::Edit {
            block_id: para_id.clone(),
            path: "content.text".into(),
            value: serde_json::json!("Welcome to the guide. See the example below."),
            operator: EditOperator::Set,
        })?;
        
        // Add tags
        engine.execute(&mut doc, Operation::Edit {
            block_id: para_id.clone(),
            path: "metadata.tags".into(),
            value: serde_json::json!(["updated"]),
            operator: EditOperator::Append,
        })?;
        
        // Validate
        let validation = engine.validate(&doc);
        println!("Valid: {}", validation.valid);
        
        // Print structure
        println!("Document has {} blocks", doc.block_count());
        
        Ok(())
    }
    ```

## See Also

- [Transactions](./transactions.md) - Atomic operation groups
- [Snapshots](./snapshots.md) - Document versioning
- [Validation](./validation.md) - Document validation
- [UCL Commands](../ucl-parser/commands.md) - UCL command syntax
