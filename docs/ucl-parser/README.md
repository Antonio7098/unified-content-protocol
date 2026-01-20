# UCL Parser

**ucl-parser** provides the parser for the Unified Content Language (UCL) — a token-efficient command language for manipulating UCM documents.

## Overview

UCL is designed for:
- **Token efficiency** - Minimal tokens for LLM interactions
- **Expressiveness** - Rich operations for document manipulation
- **Safety** - Structured commands prevent injection attacks
- **Readability** - Human-readable syntax

## Installation

```toml
[dependencies]
ucl-parser = "0.1.4"
```

## Quick Example

```rust
use ucl_parser::{parse, parse_commands};

// Parse a full UCL document
let ucl = r#"
STRUCTURE
blk_root: [blk_intro, blk_body]

BLOCKS
text #blk_intro label="Introduction" :: "Welcome to UCP!"

COMMANDS
EDIT blk_intro SET content.text = "Updated introduction"
"#;

let doc = parse(ucl).unwrap();
println!("Structure entries: {}", doc.structure.len());
println!("Block definitions: {}", doc.blocks.len());
println!("Commands: {}", doc.commands.len());

// Parse commands only
let commands = parse_commands(r#"
    EDIT blk_abc SET content.text = "Hello"
    APPEND blk_root text :: "New block"
"#).unwrap();

println!("Parsed {} commands", commands.len());
```

## Module Overview

| Module | Description |
|--------|-------------|
| [`ast`](./syntax.md) | Abstract Syntax Tree types |
| [`lexer`](./syntax.md) | Tokenizer using Logos |
| [`parser`](./syntax.md) | Recursive descent parser |

## Public API

### Functions

```rust
/// Parse a full UCL document
pub fn parse(input: &str) -> ParseResult<UclDocument>;

/// Parse UCL commands only (without STRUCTURE/BLOCKS sections)
pub fn parse_commands(input: &str) -> ParseResult<Vec<Command>>;
```

### Re-exports

```rust
pub use ast::*;
pub use lexer::{Token, TokenKind};
pub use parser::{ParseError, ParseResult, Parser};
```

## UCL Document Structure

A UCL document has three optional sections:

```
STRUCTURE
<adjacency declarations>

BLOCKS
<block definitions>

COMMANDS
<transformation commands>
```

### STRUCTURE Section

Declares parent-child relationships:

```
STRUCTURE
blk_root: [blk_child1, blk_child2]
blk_child1: [blk_grandchild]
```

### BLOCKS Section

Defines block content:

```
BLOCKS
text #blk_intro label="Introduction" :: "Welcome!"
code #blk_example lang="rust" :: "fn main() {}"
```

### COMMANDS Section

Specifies operations:

```
COMMANDS
EDIT blk_intro SET content.text = "Updated"
APPEND blk_root text :: "New paragraph"
DELETE blk_old CASCADE
```

## Commands

| Command | Description |
|---------|-------------|
| `EDIT` | Modify block content or metadata |
| `MOVE` | Move block to new parent |
| `APPEND` | Add new block |
| `DELETE` | Remove block |
| `PRUNE` | Remove unreachable blocks |
| `FOLD` | Collapse content for context management |
| `LINK` | Add relationship edge |
| `UNLINK` | Remove relationship edge |
| `SNAPSHOT` | Version management |
| `BEGIN/COMMIT/ROLLBACK` | Transaction control |
| `ATOMIC` | Atomic operation group |

## Error Handling

```rust
use ucl_parser::{parse_commands, ParseError};

let result = parse_commands("INVALID SYNTAX");

match result {
    Ok(commands) => println!("Parsed {} commands", commands.len()),
    Err(ParseError::UnexpectedToken { expected, found, line, column }) => {
        eprintln!("Error at {}:{}: expected {}, found {}", line, column, expected, found);
    }
    Err(ParseError::UnexpectedEof) => {
        eprintln!("Unexpected end of input");
    }
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## See Also

- [Syntax Reference](./syntax.md) - Complete syntax documentation
- [Commands Reference](./commands.md) - Detailed command documentation
- [Expressions](./expressions.md) - Path and condition expressions
