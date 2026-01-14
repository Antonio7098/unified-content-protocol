# UCP API

**ucp-api** provides a high-level API for working with UCP, combining all core crates into a convenient interface for application development.

## Overview

The UCP API is the recommended entry point for most applications. It provides:

- **UcpClient** - Main client for document manipulation
- **Unified interface** - Access to all UCP functionality
- **UCL integration** - Execute UCL commands directly
- **Convenience methods** - Common operations simplified

## Installation

```toml
[dependencies]
ucp-api = "0.1"
```

## Quick Start

```rust
use ucp_api::UcpClient;

fn main() {
    // Create client
    let client = UcpClient::new();
    
    // Create document
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Add content
    client.add_text(&mut doc, &root, "Hello, UCP!", Some("intro")).unwrap();
    
    // Execute UCL
    client.execute_ucl(&mut doc, r#"
        APPEND blk_root text :: "More content"
    "#).unwrap();
    
    // Serialize
    let json = client.to_json(&doc).unwrap();
    println!("{}", json);
}
```

## UcpClient

The main entry point for UCP operations.

### Creating a Client

```rust
use ucp_api::UcpClient;

// Default client
let client = UcpClient::new();
```

### Document Operations

```rust
// Create new document
let mut doc = client.create_document();

// Get document info
println!("Document ID: {}", doc.id);
println!("Root block: {}", doc.root);
println!("Block count: {}", doc.block_count());
```

### Adding Content

```rust
let root = doc.root.clone();

// Add text block
let text_id = client.add_text(
    &mut doc,
    &root,
    "Paragraph content",
    Some("paragraph")  // semantic role
).unwrap();

// Add code block
let code_id = client.add_code(
    &mut doc,
    &root,
    "rust",
    "fn main() {\n    println!(\"Hello!\");\n}"
).unwrap();
```

### Executing UCL

```rust
// Parse UCL (without executing)
let commands = client.parse_ucl(r#"
    EDIT blk_abc SET content.text = "Hello"
    APPEND blk_root text :: "New block"
"#).unwrap();

println!("Parsed {} commands", commands.len());

// Execute UCL commands
let results = client.execute_ucl(&mut doc, r#"
    APPEND blk_root text WITH label="intro" :: "Introduction"
    EDIT blk_intro SET metadata.tags += ["important"]
"#).unwrap();

for result in &results {
    if result.success {
        println!("Success: {:?}", result.affected_blocks);
    } else {
        println!("Failed: {:?}", result.error);
    }
}
```

### Serialization

```rust
// Serialize document to JSON
let json = client.to_json(&doc).unwrap();
println!("{}", json);

// Pretty-print if needed
let pretty: serde_json::Value = serde_json::from_str(&json).unwrap();
println!("{}", serde_json::to_string_pretty(&pretty).unwrap());
```

## Public API

```rust
pub use client::UcpClient;
pub use error::{Error, Result};
```

## Integration with Other Crates

UCP API re-exports types from underlying crates:

```rust
use ucp_api::UcpClient;

// From ucm-core
use ucm_core::{Block, Content, Document, Edge, EdgeType, BlockId};
use ucm_core::metadata::{SemanticRole, RoleCategory, TokenEstimate};

// From ucm-engine
use ucm_engine::{Engine, Operation, OperationResult};

// From ucl-parser
use ucl_parser::{parse, parse_commands, Command};
```

## See Also

- [Quick Start Guide](../../docs/getting-started/quick-start.md) - Getting started with UCP
- [UCL Commands](../../docs/ucl-parser/commands.md) - UCL command reference
- [UCM Core](../../docs/ucm-core/README.md) - Core types documentation
- [UCM Engine](../../docs/ucm-engine/README.md) - Engine documentation
