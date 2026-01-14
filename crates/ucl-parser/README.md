# UCL Parser

**ucl-parser** provides parsing and execution support for the Unified Content Language (UCL) — a token-efficient command language for document manipulation.

## Overview

UCL Parser handles:

- **Parsing** - Convert UCL text into structured commands
- **Validation** - Check command syntax and semantics
- **Execution Support** - Convert commands to engine operations
- **Error Reporting** - Detailed error messages and locations

## Installation

```toml
[dependencies]
ucl-parser = "0.1"
```

## Quick Example

```rust
use ucl_parser::{parse, parse_commands};

fn main() {
    // Parse a single command
    let cmd = parse("EDIT blk_abc123 SET text = \"Hello\"").unwrap();
    println!("Command: {:?}", cmd);
    
    // Parse multiple commands
    let commands = parse_commands(r#"
        EDIT blk_abc SET text = "Updated"
        APPEND blk_root text :: "New content"
    "#).unwrap();
    
    println!("Parsed {} commands", commands.len());
}
```

## UCL Syntax

UCL commands follow a simple, token-efficient format:

```
EDIT <block_id> SET <path> = "<value>"
APPEND <parent_id> <type> :: <content>
MOVE <block_id> TO <parent_id>
DELETE <block_id> [CASCADE]
LINK <source> <edge_type> <target>
```

## Core Components

### Parser

Parse UCL text into structured commands:

```rust
use ucl_parser::{Parser, Command};

let parser = Parser::new();
let commands = parser.parse_multiple(ucl_text)?;
```

### Command Types

All UCL commands are represented as enums:

```rust
pub enum Command {
    Edit(EditCommand),
    Append(AppendCommand),
    Move(MoveCommand),
    Delete(DeleteCommand),
    Link(LinkCommand),
    Snapshot(SnapshotCommand),
    Prune(PruneCommand),
}
```

### Expressions

Support for path expressions and conditions:

```rust
use ucl_parser::{Expression, Path};

// Path expressions
let path = Path::from_str("content.text")?;
let expr = Expression::Literal("Hello".to_string());
```

## Public API

```rust
pub use parser::{Parser, parse, parse_commands};
pub use ast::{
    Command, EditCommand, AppendCommand, MoveCommand, 
    DeleteCommand, LinkCommand, SnapshotCommand, PruneCommand
};
pub use expression::{Expression, Path, Operator, Value};
pub use error::{ParseError, Result};
```

## See Also

- [Syntax Reference](../../docs/ucl-parser/syntax.md) - Complete syntax documentation
- [Command Reference](../../docs/ucl-parser/commands.md) - All UCL commands
- [Expressions](../../docs/ucl-parser/expressions.md) - Path and condition expressions
