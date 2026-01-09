# UCL Syntax Reference

This document provides a complete reference for the Unified Content Language (UCL) syntax.

## Lexical Elements

### Comments

```ucl
// Single-line comment
EDIT blk_abc SET text = "value"  // Inline comment
```

### Identifiers

```ucl
// Block IDs: blk_ prefix + 24 hex characters
blk_a1b2c3d4e5f6a1b2c3d4e5f6

// Property names
label
content
metadata
```

### Literals

#### Strings

```ucl
// Double-quoted
"Hello, world!"

// Single-quoted
'Hello, world!'

// Escape sequences
"Line 1\nLine 2"
"Tab:\tvalue"
"Quote: \"quoted\""
```

#### Numbers

```ucl
// Integers
42
-17
0

// Floats
3.14
-2.5
0.001
```

#### Booleans

```ucl
true
false
```

#### Null

```ucl
null
```

#### Arrays

```ucl
[1, 2, 3]
["a", "b", "c"]
[true, false, null]
[]
```

#### Objects

```ucl
{"key": "value"}
{"name": "Alice", "age": 30}
{}
```

#### Block References

```ucl
@blk_a1b2c3d4e5f6a1b2c3d4e5f6
```

### Content Types

```ucl
text      // Plain text
table     // Tabular data
code      // Source code
math      // Mathematical expressions
media     // Images, audio, video
json      // JSON data
binary    // Binary data
composite // Container for other blocks
```

### Operators

#### Assignment Operators

| Operator | Name | Description |
|----------|------|-------------|
| `=` | Set | Replace value |
| `+=` | Append | Append to string/array |
| `-=` | Remove | Remove from string/array |
| `++` | Increment | Increment number |
| `--` | Decrement | Decrement number |

#### Comparison Operators

| Operator | Name | Description |
|----------|------|-------------|
| `=` | Equal | Equality check |
| `!=` | Not Equal | Inequality check |
| `>` | Greater Than | Greater than |
| `>=` | Greater or Equal | Greater than or equal |
| `<` | Less Than | Less than |
| `<=` | Less or Equal | Less than or equal |

#### Logical Operators

| Operator | Description |
|----------|-------------|
| `AND` | Logical AND |
| `OR` | Logical OR |
| `NOT` | Logical NOT |

#### String Operators

| Operator | Description |
|----------|-------------|
| `CONTAINS` | String contains |
| `STARTS_WITH` | String starts with |
| `ENDS_WITH` | String ends with |
| `MATCHES` | Regex match |

#### Existence Operators

| Operator | Description |
|----------|-------------|
| `EXISTS` | Path exists |
| `IS_NULL` | Value is null |
| `IS_NOT_NULL` | Value is not null |
| `IS_EMPTY` | Value is empty |

## Document Structure

### STRUCTURE Section

Declares the document hierarchy:

```ucl
STRUCTURE
blk_root: [blk_child1, blk_child2, blk_child3]
blk_child1: [blk_grandchild1, blk_grandchild2]
blk_child2: []
```

### BLOCKS Section

Defines block content:

```ucl
BLOCKS
// Text block
text #blk_intro label="Introduction" tags=["important"] :: "Welcome to UCP!"

// Code block
code #blk_example lang="rust" :: "fn main() {
    println!(\"Hello!\");
}"

// Table block
table #blk_data :: |Name|Age|City|
                   |Alice|30|NYC|
                   |Bob|25|LA|

// JSON block
json #blk_config :: {"debug": true, "level": 5}
```

#### Block Definition Syntax

```
<content_type> #<block_id> [properties...] :: <content>
```

Properties:
- `label="value"` - Block label
- `tags=["tag1", "tag2"]` - Block tags
- `role="semantic_role"` - Semantic role
- Custom properties as key=value

### COMMANDS Section

Contains transformation commands:

```ucl
COMMANDS
EDIT blk_intro SET content.text = "Updated"
APPEND blk_root text :: "New paragraph"
MOVE blk_child TO blk_newparent
DELETE blk_old CASCADE
```

## Path Expressions

Paths navigate block structure:

```ucl
// Simple property
content

// Nested property
content.text

// Array index
rows[0]

// Array slice
rows[0:5]
rows[:5]
rows[5:]

// JSON path
$data.users[0].name
```

### Path Segments

| Segment | Example | Description |
|---------|---------|-------------|
| Property | `content` | Access property |
| Index | `[0]` | Array index |
| Slice | `[0:5]` | Array slice |
| JSONPath | `$path` | JSON path expression |

## Conditions

Conditions filter operations:

```ucl
// Simple comparison
WHERE content.text = "Hello"

// Numeric comparison
WHERE metadata.priority > 5

// String operations
WHERE content.text CONTAINS "important"
WHERE label STARTS_WITH "chapter"
WHERE content.text MATCHES "^[A-Z].*"

// Existence checks
WHERE summary EXISTS
WHERE deprecated IS_NULL

// Logical combinations
WHERE priority > 5 AND status = "active"
WHERE type = "warning" OR type = "error"
WHERE NOT archived

// Parentheses for grouping
WHERE (priority > 5 OR urgent) AND NOT archived
```

## Commands

### EDIT

Modify block content or metadata:

```ucl
// Set content
EDIT blk_abc SET content.text = "New text"

// Append to content
EDIT blk_abc SET content.text += " more text"

// Set metadata
EDIT blk_abc SET metadata.label = "new-label"

// Add tags
EDIT blk_abc SET metadata.tags += ["new-tag"]

// Remove tags
EDIT blk_abc SET metadata.tags -= ["old-tag"]

// Conditional edit
EDIT blk_abc SET content.text = "Updated" WHERE status = "draft"
```

### MOVE

Move block to new location:

```ucl
// Move to new parent
MOVE blk_child TO blk_newparent

// Move to specific position
MOVE blk_child TO blk_parent AT 0

// Move before sibling
MOVE blk_child BEFORE blk_sibling

// Move after sibling
MOVE blk_child AFTER blk_sibling
```

### APPEND

Add new block:

```ucl
// Basic append
APPEND blk_parent text :: "New paragraph"

// With properties
APPEND blk_parent text WITH label="intro" tags=["important"] :: "Content"

// At specific position
APPEND blk_parent text AT 0 :: "First child"

// Code block
APPEND blk_parent code WITH lang="python" :: "def hello(): pass"
```

### DELETE

Remove block:

```ucl
// Delete single block
DELETE blk_abc

// Delete with descendants
DELETE blk_abc CASCADE

// Delete but keep children
DELETE blk_abc PRESERVE_CHILDREN

// Conditional delete
DELETE WHERE tags CONTAINS "deprecated"
```

### PRUNE

Remove unreachable or matching blocks:

```ucl
// Prune unreachable blocks
PRUNE UNREACHABLE

// Prune by condition
PRUNE WHERE tags CONTAINS "temp"

// Dry run (report without deleting)
PRUNE UNREACHABLE DRY_RUN
```

### FOLD

Collapse content for context management:

```ucl
// Fold to depth
FOLD blk_section DEPTH 2

// Fold by token limit
FOLD blk_section MAX_TOKENS 1000

// Preserve specific tags
FOLD blk_section DEPTH 1 PRESERVE_TAGS ["important", "summary"]
```

### LINK

Add relationship edge:

```ucl
// Basic link
LINK blk_source references blk_target

// With metadata
LINK blk_evidence supports blk_claim WITH confidence=0.95

// Custom edge type
LINK blk_impl implements blk_interface
```

### UNLINK

Remove relationship edge:

```ucl
UNLINK blk_source references blk_target
```

### SNAPSHOT

Version management:

```ucl
// Create snapshot
SNAPSHOT CREATE "v1.0"
SNAPSHOT CREATE "draft" WITH description="First draft"

// Restore snapshot
SNAPSHOT RESTORE "v1.0"

// List snapshots
SNAPSHOT LIST

// Delete snapshot
SNAPSHOT DELETE "old-version"

// Compare snapshots
SNAPSHOT DIFF "v1.0" "v2.0"
```

### Transaction Commands

```ucl
// Begin transaction
BEGIN TRANSACTION
BEGIN TRANSACTION "import-data"

// Commit transaction
COMMIT
COMMIT "import-data"

// Rollback transaction
ROLLBACK
ROLLBACK "import-data"
```

### ATOMIC

Group commands for atomic execution:

```ucl
ATOMIC {
    APPEND blk_root text :: "Block 1"
    APPEND blk_root text :: "Block 2"
    LINK blk_1 references blk_2
}
```

## Complete Example

```ucl
// Define document structure
STRUCTURE
blk_root: [blk_title, blk_intro, blk_chapter1]
blk_chapter1: [blk_section1, blk_section2]

// Define blocks
BLOCKS
text #blk_title role="title" :: "My Document"
text #blk_intro role="intro" label="introduction" :: "Welcome to this guide."
text #blk_chapter1 role="heading1" :: "Chapter 1: Getting Started"
text #blk_section1 role="heading2" :: "Installation"
text #blk_section2 role="heading2" :: "Configuration"

// Commands to modify
COMMANDS
// Update introduction
EDIT blk_intro SET content.text = "Welcome to this comprehensive guide."

// Add code example under section 1
APPEND blk_section1 code WITH label="install-cmd" lang="bash" :: "cargo add ucp-api"

// Add reference
LINK blk_intro references blk_section1

// Tag for review
EDIT blk_intro SET metadata.tags += ["needs-review"]

// Create snapshot
SNAPSHOT CREATE "initial" WITH description="Initial document structure"
```

## Grammar Summary

```ebnf
document     = [structure] [blocks] [commands]

structure    = "STRUCTURE" {structure_entry}
structure_entry = block_id ":" "[" [block_id {"," block_id}] "]"

blocks       = "BLOCKS" {block_def}
block_def    = content_type "#" block_id {property} "::" content_literal

commands     = "COMMANDS" {command}
command      = edit | move | append | delete | prune | fold 
             | link | unlink | snapshot | transaction | atomic

edit         = "EDIT" block_id "SET" path operator value [condition]
move         = "MOVE" block_id move_target
append       = "APPEND" block_id content_type ["AT" int] ["WITH" properties] "::" content
delete       = "DELETE" [block_id] ["CASCADE"] ["PRESERVE_CHILDREN"] [condition]
prune        = "PRUNE" prune_target ["DRY_RUN"]
fold         = "FOLD" block_id {fold_option}
link         = "LINK" block_id edge_type block_id ["WITH" properties]
unlink       = "UNLINK" block_id edge_type block_id
snapshot     = "SNAPSHOT" snapshot_action
transaction  = "BEGIN" "TRANSACTION" [string] | "COMMIT" [string] | "ROLLBACK" [string]
atomic       = "ATOMIC" "{" {command} "}"

path         = path_segment {"." path_segment}
path_segment = identifier | "[" int "]" | "[" [int] ":" [int] "]"
condition    = "WHERE" expression
expression   = and_expr {"OR" and_expr}
and_expr     = unary_expr {"AND" unary_expr}
unary_expr   = ["NOT"] primary_expr
primary_expr = comparison | "(" expression ")"
comparison   = path comp_op value | path string_op string | path exist_op
```

## See Also

- [Commands Reference](./commands.md) - Detailed command documentation
- [Expressions](./expressions.md) - Path and condition expressions
- [UCM Engine Operations](../ucm-engine/operations.md) - How commands map to operations
