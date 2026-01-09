# UCL Expressions

This document covers path expressions and condition expressions used in UCL commands.

## Path Expressions

Paths navigate through block structure to access properties.

### Simple Paths

```ucl
// Single property
content
metadata
label

// Nested property
content.text
metadata.tags
metadata.semantic_role.category
```

### Array Access

```ucl
// Index (0-based)
rows[0]
cells[5]
tags[0]

// Negative index (from end)
rows[-1]    // Last element
tags[-2]    // Second to last
```

### Array Slices

```ucl
// Range [start:end) - end is exclusive
rows[0:5]   // First 5 elements
rows[2:7]   // Elements 2-6

// Open-ended slices
rows[:5]    // First 5 elements
rows[5:]    // From index 5 to end
rows[:]     // All elements (copy)
```

### JSON Path

For navigating JSON content:

```ucl
// JSON path prefix with $
$data.users[0].name
$config.settings.debug
$response.items[*].id
```

### Combined Paths

```ucl
// Complex navigation
content.table.rows[0].cells[2]
metadata.custom.nested.value
$data.users[0].addresses[0].city
```

## Path Segments

| Segment Type | Syntax | Example |
|--------------|--------|---------|
| Property | `name` | `content` |
| Index | `[n]` | `[0]`, `[-1]` |
| Slice | `[start:end]` | `[0:5]`, `[:3]` |
| JSON Path | `$path` | `$data.key` |

## Condition Expressions

Conditions filter operations based on block properties.

### Basic Syntax

```ucl
WHERE <expression>
```

### Comparison Operations

```ucl
// Equality
WHERE content.text = "Hello"
WHERE metadata.label = "intro"

// Inequality
WHERE status != "archived"
WHERE priority != 0

// Numeric comparisons
WHERE priority > 5
WHERE count >= 10
WHERE value < 100
WHERE score <= 0.5
```

### String Operations

```ucl
// Contains substring
WHERE content.text CONTAINS "important"
WHERE tags CONTAINS "draft"

// Prefix match
WHERE label STARTS_WITH "chapter"
WHERE content.text STARTS_WITH "Note:"

// Suffix match
WHERE filename ENDS_WITH ".md"
WHERE label ENDS_WITH "-v2"

// Regex match
WHERE content.text MATCHES "^[A-Z].*\\.$"
WHERE label MATCHES "section-[0-9]+"
```

### Existence Checks

```ucl
// Property exists
WHERE summary EXISTS
WHERE metadata.custom.author EXISTS

// Null checks
WHERE deprecated IS_NULL
WHERE label IS_NOT_NULL

// Empty checks
WHERE content.text IS_EMPTY
WHERE tags IS_EMPTY
```

### Logical Operators

```ucl
// AND - both conditions must be true
WHERE priority > 5 AND status = "active"
WHERE type = "code" AND lang = "rust"

// OR - either condition must be true
WHERE type = "warning" OR type = "error"
WHERE status = "draft" OR status = "review"

// NOT - negation
WHERE NOT archived
WHERE NOT tags CONTAINS "deprecated"

// Combined
WHERE (priority > 5 OR urgent) AND NOT archived
WHERE type = "code" AND (lang = "rust" OR lang = "python")
```

### Operator Precedence

From highest to lowest:
1. `NOT`
2. `AND`
3. `OR`

Use parentheses to override:

```ucl
// Without parentheses: NOT binds tightest
WHERE NOT a AND b    // (NOT a) AND b

// With parentheses
WHERE NOT (a AND b)  // NOT (a AND b)

// OR has lowest precedence
WHERE a AND b OR c   // (a AND b) OR c
WHERE a AND (b OR c) // a AND (b OR c)
```

## Condition Examples

### By Content Type

```ucl
// Find text blocks
WHERE content.type = "text"

// Find code blocks
WHERE content.type = "code"

// Find code in specific language
WHERE content.type = "code" AND content.lang = "rust"
```

### By Metadata

```ucl
// By label
WHERE metadata.label = "introduction"

// By tag
WHERE metadata.tags CONTAINS "important"

// By role
WHERE metadata.semantic_role.category = "heading1"

// By multiple tags
WHERE metadata.tags CONTAINS "draft" AND metadata.tags CONTAINS "review"
```

### By Custom Properties

```ucl
// Custom metadata
WHERE metadata.custom.author = "Alice"
WHERE metadata.custom.version > 2
WHERE metadata.custom.approved = true
```

### Complex Conditions

```ucl
// Find draft code blocks needing review
WHERE content.type = "code" 
  AND metadata.tags CONTAINS "draft"
  AND metadata.tags CONTAINS "needs-review"

// Find important content that's not archived
WHERE (metadata.tags CONTAINS "important" OR priority > 8)
  AND NOT metadata.tags CONTAINS "archived"

// Find headings in first two levels
WHERE (metadata.semantic_role.category = "heading1"
    OR metadata.semantic_role.category = "heading2")
  AND content.text IS_NOT_NULL
```

## Using Conditions in Commands

### EDIT with Condition

```ucl
// Update all draft blocks
EDIT blk_section SET metadata.tags += ["reviewed"] WHERE status = "draft"

// Clear deprecated content
EDIT blk_doc SET content.text = "[DEPRECATED]" WHERE deprecated = true
```

### DELETE with Condition

```ucl
// Delete all deprecated blocks
DELETE WHERE metadata.tags CONTAINS "deprecated"

// Delete old temporary blocks
DELETE WHERE metadata.tags CONTAINS "temp" AND created_at < "2024-01-01"
```

### PRUNE with Condition

```ucl
// Prune by tag
PRUNE WHERE metadata.tags CONTAINS "temporary"

// Prune empty blocks
PRUNE WHERE content.text IS_EMPTY
```

## Value Types in Conditions

### Strings

```ucl
WHERE label = "intro"
WHERE label = 'intro'  // Single quotes also work
```

### Numbers

```ucl
WHERE priority = 5
WHERE score = 3.14
WHERE count = -1
```

### Booleans

```ucl
WHERE active = true
WHERE deprecated = false
```

### Null

```ucl
WHERE value = null
WHERE optional IS_NULL
```

### Arrays

```ucl
WHERE tags = ["a", "b", "c"]
WHERE metadata.tags CONTAINS "important"
```

### Block References

```ucl
WHERE parent = @blk_abc123def456
WHERE references CONTAINS @blk_target
```

## AST Representation

Conditions are represented in the AST as:

```rust
pub enum Condition {
    Comparison { path: Path, op: ComparisonOp, value: Value },
    Contains { path: Path, value: Value },
    StartsWith { path: Path, prefix: String },
    EndsWith { path: Path, suffix: String },
    Matches { path: Path, regex: String },
    Exists { path: Path },
    IsNull { path: Path },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

pub enum ComparisonOp {
    Eq,  // =
    Ne,  // !=
    Gt,  // >
    Ge,  // >=
    Lt,  // <
    Le,  // <=
}
```

## Best Practices

### 1. Use Specific Paths

```ucl
// Good - specific path
WHERE metadata.tags CONTAINS "draft"

// Less ideal - ambiguous
WHERE tags CONTAINS "draft"
```

### 2. Combine Conditions Logically

```ucl
// Good - clear grouping
WHERE (type = "warning" OR type = "error") AND NOT resolved

// Confusing - relies on precedence
WHERE type = "warning" OR type = "error" AND NOT resolved
```

### 3. Use Appropriate Operators

```ucl
// For substring search
WHERE content.text CONTAINS "keyword"

// For exact match
WHERE label = "exact-value"

// For pattern matching
WHERE label MATCHES "chapter-[0-9]+"
```

### 4. Check Existence Before Comparison

```ucl
// If property might not exist
WHERE metadata.custom.author EXISTS AND metadata.custom.author = "Alice"
```

## See Also

- [Syntax Reference](./syntax.md) - Complete syntax documentation
- [Commands Reference](./commands.md) - Command documentation
