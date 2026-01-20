# Installation

This guide covers how to install and configure UCP for your Rust project.

## Requirements

- **Rust**: 1.70 or later
- **Cargo**: Latest stable version

## Adding UCP to Your Project

### Using the High-Level API (Recommended)

For most use cases, add `ucp-api` which re-exports everything you need:

```toml
[dependencies]
ucp-api = "0.1.4"
```

### Individual Crates

For fine-grained control, you can depend on specific crates:

```toml
[dependencies]
# Core types only
ucm-core = "0.1.4"

# Core + transformation engine
ucm-engine = "0.1.4"

# UCL command parsing
ucl-parser = "0.1.4"

# Markdown conversion
ucp-translator-markdown = "0.1.4"

# Observability utilities
ucp-observe = "0.1.4"
```

## Feature Flags

### ucm-core

| Feature | Description | Default |
|---------|-------------|---------|
| `serde` | Serialization support | ✓ |

### ucm-engine

| Feature | Description | Default |
|---------|-------------|---------|
| `tracing` | Structured logging | ✓ |

## Verifying Installation

Create a simple test program:

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content, Document};

fn main() {
    // Create a client
    let client = UcpClient::new();
    
    // Create a document
    let doc = client.create_document();
    
    println!("Document created with {} block(s)", doc.block_count());
    println!("Root block ID: {}", doc.root);
}
```

Run with:

```bash
cargo run
```

Expected output:

```
Document created with 1 block(s)
Root block ID: blk_ff00000000000000000000
```

## Development Setup

For contributing to UCP or running tests:

```bash
# Clone the repository
git clone https://github.com/your-org/unified-content-protocol.git
cd unified-content-protocol

# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p ucm-core

# Build documentation
cargo doc --workspace --no-deps --open
```

## Troubleshooting

### Common Issues

**Compilation Error: Missing Dependencies**

Ensure all workspace dependencies are available:

```bash
cargo update
cargo build --workspace
```

**Version Conflicts**

If you're using multiple UCP crates, ensure they're all from the same version:

```toml
[dependencies]
ucm-core = "0.1.4"
ucm-engine = "0.1.4"  # Must match ucm-core version
```

## Next Steps

- [Quick Start Guide](./quick-start.md) - Build your first UCP application
- [Core Concepts](./concepts.md) - Understand the UCP architecture
