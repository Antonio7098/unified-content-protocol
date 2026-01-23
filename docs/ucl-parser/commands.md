# UCL Commands Reference

This document provides detailed documentation for each UCL command.

## EDIT Command

Modify block content or metadata.

### Syntax

```ucl
EDIT <block_id> SET <path> <operator> <value> [WHERE <condition>]
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `block_id` | Target block ID |
| `path` | Property path to modify |
| `operator` | Assignment operator (`=`, `+=`, `-=`, `++`, `--`) |
| `value` | New value |
| `condition` | Optional filter condition |

### Examples

=== "UCL"
    ```ucl
    // Set text content
    EDIT blk_abc123def456 SET content.text = "New content"

    // Append to text
    EDIT blk_abc123def456 SET content.text += " additional text"

    // Remove from text
    EDIT blk_abc123def456 SET content.text -= "remove this"

    // Set label
    EDIT blk_abc123def456 SET metadata.label = "new-label"

    // Add tags
    EDIT blk_abc123def456 SET metadata.tags += ["tag1", "tag2"]

    // Remove tag
    EDIT blk_abc123def456 SET metadata.tags -= ["old-tag"]

    // Set summary
    EDIT blk_abc123def456 SET metadata.summary = "Brief description"

    // Custom metadata
    EDIT blk_abc123def456 SET metadata.author = "Alice"

    // Conditional edit
    EDIT blk_abc123def456 SET content.text = "Updated" WHERE status = "draft"
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, r#"
        EDIT blk_abc123def456 SET content.text = "New content"
    "#)?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, """
        EDIT blk_abc123def456 SET content.text = "New content"
    """)
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, `
        EDIT blk_abc123def456 SET content.text = "New content"
    `);
    ```

### Supported Paths

| Path | Description |
|------|-------------|
| `content.text` or `text` | Text content |
| `metadata.label` | Block label |
| `metadata.tags` | Block tags |
| `metadata.summary` | Block summary |
| `metadata.<key>` | Custom metadata |

---

## MOVE Command

Move a block to a new location in the document structure.

### Syntax

```ucl
MOVE <block_id> TO <parent_id> [AT <index>]
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `block_id` | Block to move |
| `parent_id` | New parent block |
| `sibling_id` | Reference sibling |
| `index` | Position in parent's children (0-indexed) |

### Examples

=== "UCL"
    ```ucl
    // Move to new parent (appends to end)
    MOVE blk_child TO blk_newparent

    // Move to specific position
    MOVE blk_child TO blk_parent AT 0    // First position
    MOVE blk_child TO blk_parent AT 2    // Third position

    // Move relative to sibling
    MOVE blk_child BEFORE blk_sibling
    MOVE blk_child AFTER blk_sibling
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, r#"
        MOVE blk_child TO blk_newparent
    "#)?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, "MOVE blk_child TO blk_newparent")
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, "MOVE blk_child TO blk_newparent");
    ```

### Notes

- Moving a block to one of its descendants is prevented (cycle detection)
- Moving to the same parent at a different position is allowed

---

## APPEND Command

Add a new block to the document.

### Syntax

```ucl
APPEND <parent_id> <content_type> [AT <index>] [WITH <properties>] :: <content>
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `parent_id` | Parent block ID |
| `content_type` | Type: `text`, `code`, `table`, `math`, `media`, `json`, `binary`, `composite` |
| `index` | Optional position (0-indexed) |
| `properties` | Optional key=value pairs |
| `content` | Content literal |

### Properties

| Property | Description |
|----------|-------------|
| `label` | Block label |
| `tags` | Block tags (array) |
| `role` | Semantic role |

### Examples

=== "UCL"
    ```ucl
    // Simple text
    APPEND blk_parent text :: "New paragraph"

    // With label and tags
    APPEND blk_parent text WITH label="intro" tags=["important"] :: "Introduction"

    // At specific position
    APPEND blk_parent text AT 0 :: "First child"

    // Code block
    APPEND blk_parent code :: "fn main() {
        println!(\"Hello!\");
    }"

    // With semantic role
    APPEND blk_parent text WITH role="heading2" :: "Section Title"

    // JSON content
    APPEND blk_parent json :: {"key": "value", "count": 42}

    // Table (pipe-delimited)
    APPEND blk_parent table :: |Name|Age|
                               |Alice|30|
                               |Bob|25|
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, r#"
        APPEND blk_parent text :: "New paragraph"
    "#)?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, """
        APPEND blk_parent text :: "New paragraph"
    """)
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, `
        APPEND blk_parent text :: "New paragraph"
    `);
    ```

---

## DELETE Command

Remove a block from the document.

### Syntax

```ucl
DELETE <block_id> [CASCADE] [PRESERVE_CHILDREN]
DELETE WHERE <condition>
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `block_id` | Block to delete |
| `CASCADE` | Also delete all descendants |
| `PRESERVE_CHILDREN` | Reparent children to grandparent before deleting |
| `condition` | Delete blocks matching condition |

### Examples

=== "UCL"
    ```ucl
    // Delete single block (children become orphaned)
    DELETE blk_abc123def456

    // Delete with all descendants
    DELETE blk_abc123def456 CASCADE

    // Delete but keep children (move to grandparent)
    DELETE blk_abc123def456 PRESERVE_CHILDREN

    // Delete by condition
    DELETE WHERE tags CONTAINS "deprecated"
    DELETE WHERE metadata.status = "archived"
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, "DELETE blk_abc123def456 CASCADE")?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, "DELETE blk_abc123def456 CASCADE")
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, "DELETE blk_abc123def456 CASCADE");
    ```

### Notes

- Without `CASCADE`, children become orphaned (unreachable)
- `PRESERVE_CHILDREN` moves children to the deleted block's parent
- Conditional delete affects all matching blocks

---

## PRUNE Command

Remove unreachable blocks or blocks matching a condition.

### Syntax

```ucl
PRUNE UNREACHABLE [DRY_RUN]
PRUNE WHERE <condition> [DRY_RUN]
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `UNREACHABLE` | Remove blocks not reachable from root |
| `condition` | Remove blocks matching condition |
| `DRY_RUN` | Report without actually deleting |

### Examples

=== "UCL"
    ```ucl
    // Remove all orphaned blocks
    PRUNE UNREACHABLE

    // Preview what would be pruned
    PRUNE UNREACHABLE DRY_RUN

    // Prune by tag
    PRUNE WHERE tags CONTAINS "temporary"

    // Prune old blocks
    PRUNE WHERE metadata.created_at < "2024-01-01"
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, "PRUNE UNREACHABLE")?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, "PRUNE UNREACHABLE")
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, "PRUNE UNREACHABLE");
    ```

---

## FOLD Command

Collapse content for context management (LLM optimization).

### Syntax

```ucl
FOLD <block_id> [DEPTH <n>] [MAX_TOKENS <n>] [PRESERVE_TAGS <tags>]
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `block_id` | Block to fold |
| `DEPTH` | Maximum depth to preserve |
| `MAX_TOKENS` | Maximum tokens to preserve |
| `PRESERVE_TAGS` | Tags to always preserve |

### Examples

=== "UCL"
    ```ucl
    // Fold to depth 2
    FOLD blk_section DEPTH 2

    // Fold by token limit
    FOLD blk_document MAX_TOKENS 4000

    // Preserve important content
    FOLD blk_section DEPTH 1 PRESERVE_TAGS ["important", "summary"]

    // Combine options
    FOLD blk_chapter DEPTH 3 MAX_TOKENS 2000 PRESERVE_TAGS ["key-point"]
    ```

### Notes

- Folded content is replaced with summaries (if available)
- Blocks with preserved tags are not folded
- Useful for fitting documents into LLM context windows

---

## LINK Command

Add a relationship edge between blocks.

### Syntax

```ucl
LINK <source_id> <edge_type> <target_id> [WITH <properties>]
```

### Parameters

| Parameter | Description |
|-----------|-------------|
| `source_id` | Source block ID |
| `edge_type` | Relationship type |
| `target_id` | Target block ID |
| `properties` | Optional edge metadata |

### Edge Types

| Type | Description |
|------|-------------|
| `references` | General reference |
| `derived_from` | Content derived from |
| `supersedes` | Replaces another block |
| `supports` | Provides evidence for |
| `contradicts` | Contradicts (symmetric) |
| `elaborates` | Expands on |
| `summarizes` | Summarizes |
| `version_of` | Different version |
| `translation_of` | Translation |
| Custom | Any custom type |

### Examples

=== "UCL"
    ```ucl
    // Basic reference
    LINK blk_paragraph references blk_source

    // Evidence supports claim
    LINK blk_evidence supports blk_claim WITH confidence=0.95

    // Derivation
    LINK blk_summary derived_from blk_original

    // Contradiction
    LINK blk_counter contradicts blk_argument

    // Custom relationship
    LINK blk_impl implements blk_interface

    // With description
    LINK blk_a references blk_b WITH description="See also"
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, "LINK blk_a references blk_b")?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, "LINK blk_a references blk_b")
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, "LINK blk_a references blk_b");
    ```

---

## UNLINK Command

Remove a relationship edge between blocks.

### Syntax

```ucl
UNLINK <source_id> <edge_type> <target_id>
```

### Examples

=== "UCL"
    ```ucl
    UNLINK blk_paragraph references blk_source
    UNLINK blk_evidence supports blk_claim
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, "UNLINK blk_a references blk_b")?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, "UNLINK blk_a references blk_b")
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, "UNLINK blk_a references blk_b");
    ```

---

## SNAPSHOT Commands

Manage document versions.

### Create Snapshot

```ucl
SNAPSHOT CREATE "<name>"
SNAPSHOT CREATE "<name>" WITH description="<description>"
```

### Restore Snapshot

```ucl
SNAPSHOT RESTORE "<name>"
```

### List Snapshots

```ucl
SNAPSHOT LIST
```

### Delete Snapshot

```ucl
SNAPSHOT DELETE "<name>"
```

### Compare Snapshots

```ucl
SNAPSHOT DIFF "<name1>" "<name2>"
```

### Examples

=== "UCL"
    ```ucl
    // Create versioned snapshot
    SNAPSHOT CREATE "v1.0" WITH description="Initial release"

    // Create checkpoint
    SNAPSHOT CREATE "before-refactor"

    // Restore previous version
    SNAPSHOT RESTORE "v1.0"

    // Clean up old snapshots
    SNAPSHOT DELETE "draft-1"
    SNAPSHOT DELETE "draft-2"
    ```

=== "Rust (via Client)"
    ```rust
    client.execute_ucl(&mut doc, r#"SNAPSHOT CREATE "v1.0""#)?;
    ```

=== "Python"
    ```python
    ucp_content.execute_ucl(doc, 'SNAPSHOT CREATE "v1.0"')
    ```

=== "JavaScript"
    ```javascript
    executeUcl(doc, 'SNAPSHOT CREATE "v1.0"');
    ```

---

## Transaction Commands

Group operations for atomic execution.

### BEGIN TRANSACTION

```ucl
BEGIN TRANSACTION
BEGIN TRANSACTION "<name>"
```

### COMMIT

```ucl
COMMIT
COMMIT "<name>"
```

### ROLLBACK

```ucl
ROLLBACK
ROLLBACK "<name>"
```

### Examples

=== "UCL"
    ```ucl
    // Anonymous transaction
    BEGIN TRANSACTION
    APPEND blk_root text :: "Block 1"
    APPEND blk_root text :: "Block 2"
    COMMIT

    // Named transaction
    BEGIN TRANSACTION "import-chapter"
    APPEND blk_root text WITH role="heading1" :: "Chapter 3"
    APPEND blk_chapter3 text :: "Content..."
    COMMIT "import-chapter"

    // Rollback on error
    BEGIN TRANSACTION "risky-operation"
    DELETE blk_important CASCADE
    // Oops, wrong block!
    ROLLBACK "risky-operation"
    ```

---

## ATOMIC Command

Execute multiple commands atomically.

### Syntax

```ucl
ATOMIC {
    <command>
    <command>
    ...
}
```

### Examples

=== "UCL"
    ```ucl
    // Atomic block creation with linking
    ATOMIC {
        APPEND blk_root text WITH label="claim" :: "Main argument"
        APPEND blk_root text WITH label="evidence" :: "Supporting evidence"
        LINK blk_evidence supports blk_claim
    }

    // Atomic restructure
    ATOMIC {
        MOVE blk_section1 TO blk_chapter2
        MOVE blk_section2 TO blk_chapter2
        DELETE blk_chapter1 CASCADE
    }
    ```

### Notes

- All commands succeed or none are applied
- Equivalent to a transaction with immediate commit
- Useful for operations that must be atomic

---

## Command Chaining

Commands can be written on separate lines or chained:

```ucl
// Separate lines
EDIT blk_a SET text = "Hello"
EDIT blk_b SET text = "World"

// Multiple operations
APPEND blk_root text :: "Para 1"
APPEND blk_root text :: "Para 2"
LINK blk_para1 references blk_para2
```

## See Also

- [Syntax Reference](./syntax.md) - Complete syntax documentation
- [Expressions](./expressions.md) - Path and condition expressions
- [UCM Engine Operations](../ucm-engine/operations.md) - How commands execute
