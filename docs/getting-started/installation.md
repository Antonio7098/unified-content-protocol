# Installation

This guide covers how to install and configure UCP for your project.

## Requirements

=== "Rust (Library)"
    - **Rust**: 1.75 or later
    - **Cargo**: Latest stable version

=== "Rust (CLI)"
    - **Rust**: 1.75 or later
    - **Cargo**: Latest stable version

=== "Python"
    - **Python**: 3.8 or later
    - **pip**: Latest stable version

=== "JavaScript"
    - **Node.js**: 16 or later
    - **npm**: Latest stable version

## Installing the CLI

The **ucp-cli** is the fastest way to get started with UCP. It provides a command-line interface for all UCP operations.

### Option 1: Install from crates.io (Recommended)

```bash
# Install the latest version from crates.io
cargo install ucp-cli

# Verify installation
ucp --version

# Get help
ucp --help
```

### Option 2: Install from Source

```bash
# Clone the repository
git clone https://github.com/Antonio7098/unified-content-protocol.git
cd unified-content-protocol

# Install from local source
cargo install --path crates/ucp-cli

# Or run without installing
cargo run -p ucp-cli -- --help
```

### Option 3: Build for Distribution

```bash
# Build release binary
cargo build --release -p ucp-cli

# Binary located at: target/release/ucp
./target/release/ucp --version
```

## Adding UCP to Your Project

=== "Rust"

    For most use cases, add `ucp-api` which re-exports everything you need:

    ```toml
    [dependencies]
    ucp-api = "0.1.11"
    ```

    [View ucp-api on crates.io](https://crates.io/crates/ucp-api)

    For fine-grained control, you can depend on specific crates:

    ```toml
    [dependencies]
    # Core types only
    ucm-core = "0.1.11"

    # Core + transformation engine
    ucm-engine = "0.1.11"

    # UCL command parsing
    ucl-parser = "0.1.11"

    # Markdown conversion
    ucp-translator-markdown = "0.1.11"

    # HTML conversion
    ucp-translator-html = "0.1.11"

    # Observability utilities
    ucp-observe = "0.1.11"

    # LLM utilities
    ucp-llm = "0.1.11"

    # Agent graph traversal
    ucp-agent = "0.1.11"
    ```

    **All crates on crates.io:**

    | Crate | Description | crates.io Link |
    |-------|-------------|----------------|
    | ucm-core | Core types | [Link](https://crates.io/crates/ucm-core) |
    | ucm-engine | Transformation engine | [Link](https://crates.io/crates/ucm-engine) |
    | ucl-parser | UCL parser | [Link](https://crates.io/crates/ucl-parser) |
    | ucp-api | High-level API | [Link](https://crates.io/crates/ucp-api) |
    | ucp-cli | CLI tool | [Link](https://crates.io/crates/ucp-cli) |
    | ucp-translator-markdown | Markdown translator | [Link](https://crates.io/crates/ucp-translator-markdown) |
    | ucp-translator-html | HTML translator | [Link](https://crates.io/crates/ucp-translator-html) |
    | ucp-llm | LLM utilities | [Link](https://crates.io/crates/ucp-llm) |
    | ucp-observe | Observability | [Link](https://crates.io/crates/ucp-observe) |
    | ucp-agent | Agent traversal | [Link](https://crates.io/crates/ucp-agent) |

=== "Python"

    Install the `ucp-content` package from PyPI:

    ```bash
    pip install ucp-content
    ```

    Add to your requirements.txt:

    ```txt
    ucp-content>=0.1.0
    ```

    For development with the latest features:

    ```bash
    pip install git+https://github.com/Antonio7098/unified-content-protocol.git#subdirectory=packages/ucp-python
    ```

=== "JavaScript"

    Install the `ucp-content` package from npm:

    ```bash
    npm install ucp-content
    ```

    Add to your package.json:

    ```json
    {
      "dependencies": {
        "ucp-content": "^0.1.0"
      }
    }
    ```

    For development with the latest features:

    ```bash
    npm install https://github.com/Antonio7098/unified-content-protocol.git#main --packages=packages/ucp-js
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
ucm-core = "0.1.11"
ucm-engine = "0.1.9"  # Must match ucm-core version
```

## Next Steps

- [Quick Start Guide](./quick-start.md) - Build your first UCP application
- [Core Concepts](./concepts.md) - Understand the UCP architecture
