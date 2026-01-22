# UCP Observe

**ucp-observe** provides observability utilities for UCP applications, including tracing initialization, audit logging, and metrics collection.

## Overview

UCP Observe helps you:

- **Initialize tracing** - Set up structured logging
- **Audit operations** - Track document changes
- **Collect metrics** - Monitor performance

## Installation

=== "Rust"
    ```toml
    [dependencies]
    ucp-observe = "0.1.7"
    ```

=== "Python"
    ```bash
    pip install ucp-content
    ```

=== "JavaScript"
    ```bash
    npm install ucp-content
    ```

## Quick Start

=== "Rust"
    ```rust
    use ucp_observe::{init_tracing, AuditEntry, MetricsRecorder};

    fn main() {
        init_tracing();
        
        let mut metrics = MetricsRecorder::new();
        metrics.record_operation(true);
        metrics.record_block_created();
        
        let audit = AuditEntry::new("create_document", "doc-123")
            .with_user("alice")
            .with_duration(42);
    }
    ```

=== "Python"
    ```python
    import ucp

    # Create metrics recorder
    metrics = ucp.MetricsRecorder()
    metrics.record_operation(True)
    metrics.record_block_created()

    print(f"Total ops: {metrics.operations_total}")
    print(f"Blocks created: {metrics.blocks_created}")

    # Create audit entry
    audit = ucp.AuditEntry("CREATE", "doc-123")
    audit = audit.with_user("alice").with_duration(42)

    print(audit.to_dict())
    ```

=== "JavaScript"
    ```javascript
    import { WasmMetricsRecorder, WasmAuditEntry } from 'ucp-content';

    // Create metrics recorder
    const metrics = new WasmMetricsRecorder();
    metrics.recordOperation(true);
    metrics.recordBlockCreated();

    console.log(`Total ops: ${metrics.operationsTotal}`);
    console.log(`Blocks created: ${metrics.blocksCreated}`);

    // Create audit entry
    const audit = new WasmAuditEntry('CREATE', 'doc-123')
        .withUser('alice')
        .withDuration(42);

    console.log(audit.toJson());
    ```

## Tracing (Rust Only)

=== "Rust"
    ```rust
    use ucp_observe::{init_tracing, AuditEntry, MetricsRecorder};

    fn main() {
        // Initialize tracing
        init_tracing();
        
        // Create metrics recorder
        let mut metrics = MetricsRecorder::new();
        
        // Record operations
        metrics.record("document_created", 1);
        metrics.record("blocks_added", 5);
        
        // Create audit entry
        let audit = AuditEntry::new("create_document")
            .with_user("alice")
            .with_detail("document_id", "doc-123");
        
        println!("{}", audit);
    }
    ```

## Tracing

### Initialize Tracing

=== "Rust"
    ```rust
    use ucp_observe::init_tracing;

    fn main() {
        // Initialize with default settings
        init_tracing();
        
        // Now tracing macros work
        tracing::info!("Application started");
        tracing::debug!("Debug information");
    }
    ```

### Tracing in UCP Operations

The UCM Engine uses tracing for operation logging:

=== "Rust"
    ```rust
    use ucm_engine::Engine;
    use tracing::info;

    let engine = Engine::new();

    // Operations are automatically traced
    let result = engine.execute(&mut doc, operation)?;

    // Add your own traces
    info!(
        operation = "custom_operation",
        block_count = doc.block_count(),
        "Operation completed"
    );
    ```

### Log Levels

| Level | Usage |
|-------|-------|
| `error` | Errors that need attention |
| `warn` | Warnings about potential issues |
| `info` | General information |
| `debug` | Detailed debugging information |
| `trace` | Very detailed tracing |

=== "Rust"
    ```rust
    use tracing::{error, warn, info, debug, trace};

    error!("Operation failed: {}", error_message);
    warn!("Deprecated feature used");
    info!("Document saved");
    debug!("Block ID: {}", block_id);
    trace!("Entering function");
    ```

## Audit Logging

### AuditEntry

Track operations for compliance and debugging:

=== "Rust"
    ```rust
    use ucp_observe::AuditEntry;

    // Create audit entry
    let entry = AuditEntry::new("edit_block")
        .with_user("alice@example.com")
        .with_detail("block_id", "blk_abc123")
        .with_detail("operation", "update_content")
        .with_detail("old_value", "Hello")
        .with_detail("new_value", "Hello, World!");

    // Log the entry
    println!("{}", entry);

    // Access fields
    println!("Action: {}", entry.action);
    println!("User: {:?}", entry.user);
    println!("Timestamp: {}", entry.timestamp);
    ```

### AuditEntry Structure

=== "Rust"
    ```rust
    pub struct AuditEntry {
        /// Unique entry ID
        pub id: String,
        
        /// Action performed
        pub action: String,
        
        /// User who performed the action
        pub user: Option<String>,
        
        /// When the action occurred
        pub timestamp: DateTime<Utc>,
        
        /// Additional details
        pub details: HashMap<String, String>,
    }
    ```

### Audit Trail Example

=== "Rust"
    ```rust
    use ucp_observe::AuditEntry;
    use std::collections::VecDeque;

    struct AuditTrail {
        entries: VecDeque<AuditEntry>,
        max_entries: usize,
    }

    impl AuditTrail {
        fn new(max_entries: usize) -> Self {
            Self {
                entries: VecDeque::new(),
                max_entries,
            }
        }
        
        fn log(&mut self, entry: AuditEntry) {
            if self.entries.len() >= self.max_entries {
                self.entries.pop_front();
            }
            self.entries.push_back(entry);
        }
        
        fn recent(&self, n: usize) -> Vec<&AuditEntry> {
            self.entries.iter().rev().take(n).collect()
        }
    }

    // Usage
    let mut trail = AuditTrail::new(1000);

    trail.log(AuditEntry::new("create_document")
        .with_user("alice")
        .with_detail("doc_id", "doc-1"));

    trail.log(AuditEntry::new("add_block")
        .with_user("alice")
        .with_detail("doc_id", "doc-1")
        .with_detail("block_id", "blk_abc"));

    // Get recent entries
    for entry in trail.recent(10) {
        println!("{}: {} by {:?}", entry.timestamp, entry.action, entry.user);
    }
    ```

## Metrics Collection

### MetricsRecorder

Simple metrics collection:

=== "Rust"
    ```rust
    use ucp_observe::MetricsRecorder;

    let mut metrics = MetricsRecorder::new();

    // Record counts
    metrics.record("documents_created", 1);
    metrics.record("blocks_added", 10);
    metrics.record("operations_executed", 5);

    // Increment existing metric
    metrics.record("operations_executed", 3);  // Now 8

    // Get metric value
    if let Some(count) = metrics.get("documents_created") {
        println!("Documents created: {}", count);
    }

    // Get all metrics
    for (name, value) in metrics.all() {
        println!("{}: {}", name, value);
    }
    ```

### MetricsRecorder Structure

=== "Rust"
    ```rust
    pub struct MetricsRecorder {
        counters: HashMap<String, u64>,
    }

    impl MetricsRecorder {
        pub fn new() -> Self;
        pub fn record(&mut self, name: &str, value: u64);
        pub fn get(&self, name: &str) -> Option<u64>;
        pub fn all(&self) -> impl Iterator<Item = (&String, &u64)>;
        pub fn reset(&mut self);
    }
    ```

### Metrics Example

=== "Rust"
    ```rust
    use ucp_observe::MetricsRecorder;
    use ucm_engine::Engine;
    use ucm_core::Document;

    struct InstrumentedEngine {
        engine: Engine,
        metrics: MetricsRecorder,
    }

    impl InstrumentedEngine {
        fn new() -> Self {
            Self {
                engine: Engine::new(),
                metrics: MetricsRecorder::new(),
            }
        }
        
        fn execute(&mut self, doc: &mut Document, op: Operation) -> Result<OperationResult> {
            self.metrics.record("operations_total", 1);
            
            let result = self.engine.execute(doc, op)?;
            
            if result.success {
                self.metrics.record("operations_success", 1);
                self.metrics.record("blocks_affected", result.affected_blocks.len() as u64);
            } else {
                self.metrics.record("operations_failed", 1);
            }
            
            Ok(result)
        }
        
        fn report(&self) {
            println!("Metrics Report:");
            for (name, value) in self.metrics.all() {
                println!("  {}: {}", name, value);
            }
        }
    }
    ```

## Complete Example

=== "Rust"
    ```rust
    use ucp_observe::{init_tracing, AuditEntry, MetricsRecorder};
    use ucm_engine::{Engine, Operation};
    use ucm_core::{Content, Document};
    use tracing::{info, warn, error, instrument};

    struct UcpApplication {
        engine: Engine,
        metrics: MetricsRecorder,
        current_user: Option<String>,
    }

    impl UcpApplication {
        fn new() -> Self {
            init_tracing();
            
            Self {
                engine: Engine::new(),
                metrics: MetricsRecorder::new(),
                current_user: None,
            }
        }
        
        fn set_user(&mut self, user: &str) {
            self.current_user = Some(user.to_string());
            info!(user = user, "User set");
        }
        
        #[instrument(skip(self, doc))]
        fn create_document(&mut self) -> Document {
            let doc = Document::create();
            
            self.metrics.record("documents_created", 1);
            
            let audit = AuditEntry::new("create_document")
                .with_user(self.current_user.as_deref().unwrap_or("anonymous"))
                .with_detail("document_id", &doc.id.0);
            
            info!(%audit, "Document created");
            
            doc
        }
        
        #[instrument(skip(self, doc, op), fields(op_type = %op.description()))]
        fn execute(&mut self, doc: &mut Document, op: Operation) -> Result<(), String> {
            self.metrics.record("operations_total", 1);
            
            let result = self.engine.execute(doc, op.clone())
                .map_err(|e| e.to_string())?;
            
            if result.success {
                self.metrics.record("operations_success", 1);
                
                let audit = AuditEntry::new(&op.description())
                    .with_user(self.current_user.as_deref().unwrap_or("anonymous"))
                    .with_detail("affected_blocks", &format!("{:?}", result.affected_blocks));
                
                info!(%audit, "Operation succeeded");
            } else {
                self.metrics.record("operations_failed", 1);
                
                warn!(
                    error = ?result.error,
                    "Operation failed"
                );
                
                return Err(result.error.unwrap_or_default());
            }
            
            Ok(())
        }
        
        fn report_metrics(&self) {
            info!("=== Metrics Report ===");
            for (name, value) in self.metrics.all() {
                info!(metric = name, value = value, "Metric");
            }
        }
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = UcpApplication::new();
        
        app.set_user("alice@example.com");
        
        let mut doc = app.create_document();
        let root = doc.root.clone();
        
        // Execute operations
        app.execute(&mut doc, Operation::Append {
            parent_id: root.clone(),
            content: Content::text("Hello, World!"),
            label: Some("greeting".into()),
            tags: vec![],
            semantic_role: Some("intro".into()),
            index: None,
        })?;
        
        app.execute(&mut doc, Operation::Append {
            parent_id: root,
            content: Content::code("rust", "fn main() {}"),
            label: Some("example".into()),
            tags: vec!["code".into()],
            semantic_role: Some("code".into()),
            index: None,
        })?;
        
        // Report
        app.report_metrics();
        
        Ok(())
    }
    ```

## Integration with External Systems

### Prometheus-style Metrics

=== "Rust"
    ```rust
    use ucp_observe::MetricsRecorder;

    impl MetricsRecorder {
        fn to_prometheus(&self) -> String {
            let mut output = String::new();
            for (name, value) in self.all() {
                output.push_str(&format!(
                    "ucp_{} {}\n",
                    name.replace("-", "_"),
                    value
                ));
            }
            output
        }
    }

    // Expose via HTTP endpoint
    // GET /metrics -> metrics.to_prometheus()
    ```

### JSON Audit Export

=== "Rust"
    ```rust
    use ucp_observe::AuditEntry;
    use serde_json;

    impl AuditEntry {
        fn to_json(&self) -> String {
            serde_json::json!({
                "id": self.id,
                "action": self.action,
                "user": self.user,
                "timestamp": self.timestamp.to_rfc3339(),
                "details": self.details,
            }).to_string()
        }
    }

    // Send to logging service
    // audit_service.log(entry.to_json());
    ```

## Best Practices

### 1. Initialize Tracing Early

=== "Rust"
    ```rust
    fn main() {
        // First thing in main
        init_tracing();
        
        // Rest of application...
    }
    ```

### 2. Use Structured Logging

=== "Rust"
    ```rust
    // Good - structured fields
    info!(
        document_id = %doc.id,
        block_count = doc.block_count(),
        "Document loaded"
    );

    // Less ideal - string interpolation
    info!("Document {} loaded with {} blocks", doc.id, doc.block_count());
    ```

### 3. Include Context in Audit Entries

=== "Rust"
    ```rust
    let audit = AuditEntry::new("delete_block")
        .with_user(&user_id)
        .with_detail("block_id", &block_id.to_string())
        .with_detail("reason", "User requested deletion")
        .with_detail("cascade", &cascade.to_string());
    ```

### 4. Reset Metrics Periodically

=== "Rust"
    ```rust
    // For time-windowed metrics
    metrics.reset();

    // Or use separate recorders for different windows
    let hourly_metrics = MetricsRecorder::new();
    let daily_metrics = MetricsRecorder::new();
    ```

## See Also

- [UCM Engine](../ucm-engine/README.md) - Engine with tracing support
- [UCP API](../ucp-api/README.md) - High-level API

