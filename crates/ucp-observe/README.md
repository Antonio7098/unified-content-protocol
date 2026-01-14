# UCP Observe

**ucp-observe** provides observability utilities for UCP applications, including tracing, metrics, and audit logging for monitoring document operations.

## Overview

UCP Observe offers:

- **Tracing** - Distributed tracing for operation flows
- **Metrics** - Performance and usage metrics
- **Audit Logging** - Immutable audit trail of changes
- **Event Bus** - Publish/subscribe pattern for events

## Installation

```toml
[dependencies]
ucp-observe = "0.1"
```

## Quick Example

```rust
use ucp_observe::{Tracer, Metrics, EventBus, DocumentEvent};

fn main() {
    // Initialize tracing
    let tracer = Tracer::new("ucp-app");
    
    // Start a span
    let _span = tracer.start_span("document_operation");
    
    // Record metrics
    let metrics = Metrics::new();
    metrics.counter("documents_created").increment();
    metrics.histogram("operation_duration").record(42.5);
    
    // Emit events
    let event_bus = EventBus::new();
    event_bus.publish(DocumentEvent::Created {
        document_id: "doc_123".to_string(),
        timestamp: std::time::SystemTime::now(),
    });
}
```

## Core Components

### Tracing

Distributed tracing for operation flows:

```rust
use ucp_observe::{Tracer, Span};

let tracer = Tracer::new("my-app");
let span = tracer.start_span("operation");

// Add context
span.set_attribute("document_id", "doc_123");
span.set_attribute("operation", "edit");

// Finish span
span.finish();
```

### Metrics

Performance and usage metrics:

```rust
use ucp_observe::Metrics;

let metrics = Metrics::new();

// Counters
metrics.counter("operations_total").increment();
metrics.counter("operations_total").increment_by(5);

// Histograms
metrics.histogram("duration_ms").record(42.5);
metrics.histogram("duration_ms").record_multiple(&[10.0, 20.0, 30.0]);

// Gauges
metrics.gauge("active_documents").set(42);
```

### Event Bus

Publish/subscribe pattern for events:

```rust
use ucp_observe::{EventBus, DocumentEvent, EventHandler};

let event_bus = EventBus::new();

// Subscribe to events
event_bus.subscribe(Box::new(|event: &DocumentEvent| {
    match event {
        DocumentEvent::Created { document_id, .. } => {
            println!("Document created: {}", document_id);
        }
        DocumentEvent::Modified { document_id, .. } => {
            println!("Document modified: {}", document_id);
        }
    }
}));

// Publish events
event_bus.publish(DocumentEvent::Created {
    document_id: "doc_123".to_string(),
    timestamp: std::time::SystemTime::now(),
});
```

### Audit Logging

Immutable audit trail:

```rust
use ucp_observe::{AuditLogger, AuditEvent};

let audit = AuditLogger::new("audit.log");

// Log operations
audit.log(AuditEvent::Operation {
    operation_id: "op_456".to_string(),
    user_id: "user_789".to_string(),
    operation: "EDIT".to_string(),
    target: "blk_abc".to_string(),
    timestamp: std::time::SystemTime::now(),
    metadata: serde_json::json!({"changes": ["content"]}),
});
```

## Public API

```rust
pub use tracing::{Tracer, Span, SpanContext};
pub use metrics::{Metrics, Counter, Histogram, Gauge};
pub use events::{EventBus, EventHandler, Event};
pub use audit::{AuditLogger, AuditEvent};
```

## Event Types

UCP Observe defines standard event types:

```rust
pub enum DocumentEvent {
    Created { document_id: String, timestamp: SystemTime },
    Modified { document_id: String, timestamp: SystemTime },
    Deleted { document_id: String, timestamp: SystemTime },
    SnapshotCreated { document_id: String, snapshot_id: String, timestamp: SystemTime },
}
```

## Configuration

Configure observability components:

```rust
use ucp_observe::{ObservabilityConfig, init_observability};

let config = ObservabilityConfig {
    tracing_enabled: true,
    metrics_enabled: true,
    audit_enabled: true,
    sampling_rate: 0.1, // 10% sampling for traces
};

let obs = init_observability(config);
```

## See Also

- [Observability Guide](../../docs/ucp-observe/README.md) - Detailed observability documentation
- [Tracing Concepts](../../docs/ucp-observe/tracing.md) - Distributed tracing
- [Metrics Reference](../../docs/ucp-observe/metrics.md) - Available metrics
- [Audit Trail](../../docs/ucp-observe/audit.md) - Audit logging
