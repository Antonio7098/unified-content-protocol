# Installation

This guide covers how to install and configure UCP for your project.

## Requirements

=== "Rust"
    - **Rust**: 1.70 or later
    - **Cargo**: Latest stable version

=== "Python"
    - **Python**: 3.8 or later
    - **pip**: Latest stable version

=== "JavaScript"
    - **Node.js**: 16 or later
    - **npm**: Latest stable version

## Adding UCP to Your Project

=== "Rust"
    ### Using the High-Level API (Recommended)

    For most use cases, add `ucp-api` which re-exports everything you need:

    ```toml
    [dependencies]
    ucp-api = "0.1.8"
    ```

    ### Individual Crates

    For fine-grained control, you can depend on specific crates:

    ```toml
    [dependencies]
    # Core types only
    ucm-core = "0.1.8"

    # Core + transformation engine
    ucm-engine = "0.1.8"

    # UCL command parsing
    ucl-parser = "0.1.8"

    # Markdown conversion
    ucp-translator-markdown = "0.1.8"

    # Observability utilities
    ucp-observe = "0.1.8"
    ```

=== "Python"
    Install the `ucp-content` package from PyPI:

    ```bash
    pip install ucp-content
    ```

=== "JavaScript"
    Install the `ucp-content` package from npm:

    ```bash
    npm install ucp-content
    ```

## Feature Flags (Rust only)

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

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document

    # Create a document
    doc = Document.create()

    print(f"Document created with {doc.block_count()} block(s)")
    print(f"Root block ID: {doc.root_id}")
    ```

    Run with:

    ```bash
    python test_ucp.py
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    // Create a document
    const doc = Document.create();

    console.log(`Document created with ${doc.blockCount()} block(s)`);
    console.log(`Root block ID: ${doc.rootId}`);
    ```

    Run with:

    ```bash
    node test_ucp.js
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
ucm-core = "0.1.8"
ucm-engine = "0.1.8"  # Must match ucm-core version
```

## Next Steps

- [Quick Start Guide](./quick-start.md) - Build your first UCP application
- [Core Concepts](./concepts.md) - Understand the UCP architecture
