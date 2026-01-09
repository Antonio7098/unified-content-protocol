# Validation

The validation pipeline ensures document integrity by checking for structural issues, resource limits, and consistency.

## Validation Result

```rust
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub code: ErrorCode,
    pub message: String,
    pub location: Option<Location>,
    pub suggestion: Option<String>,
}

pub enum ValidationSeverity {
    Error,   // Document is invalid
    Warning, // Potential issue
    Info,    // Informational
}
```

## Basic Usage

```rust
use ucm_engine::Engine;
use ucm_core::Document;

let engine = Engine::new();
let doc = Document::create();

let result = engine.validate(&doc);

if result.valid {
    println!("Document is valid!");
} else {
    println!("Document has issues:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }
}
```

## Validation Pipeline

```rust
use ucm_engine::validate::{ValidationPipeline, ResourceLimits};

// Default pipeline
let validator = ValidationPipeline::new();

// With custom limits
let limits = ResourceLimits {
    max_document_size: 10 * 1024 * 1024,  // 10MB
    max_block_count: 50_000,
    max_block_size: 1 * 1024 * 1024,      // 1MB
    max_nesting_depth: 20,
    max_edges_per_block: 500,
};
let validator = ValidationPipeline::with_limits(limits);

let result = validator.validate_document(&doc);
```

## Resource Limits

```rust
pub struct ResourceLimits {
    /// Maximum total document size (default: 50MB)
    pub max_document_size: usize,
    
    /// Maximum number of blocks (default: 100,000)
    pub max_block_count: usize,
    
    /// Maximum size of a single block (default: 5MB)
    pub max_block_size: usize,
    
    /// Maximum nesting depth (default: 50)
    pub max_nesting_depth: usize,
    
    /// Maximum edges per block (default: 1,000)
    pub max_edges_per_block: usize,
}
```

## Validation Checks

### Structure Validation

**Cycle Detection**
```rust
// Cycles in document structure are errors
// E201: Cycle detected in structure
```

**Nesting Depth**
```rust
// Deep nesting beyond limit
// E403: Nesting depth limit exceeded
```

**Missing References**
```rust
// Structure references non-existent blocks
// E001: Block not found
```

### Block Validation

**Block Size**
```rust
// Block content exceeds size limit
// E402: Block size limit exceeded
```

**Edge Count**
```rust
// Too many edges on a single block
// E404: Edge count limit exceeded
```

**Dangling Edge Targets**
```rust
// Edge points to non-existent block
// E001: Block not found
```

### Document Validation

**Block Count**
```rust
// Too many blocks in document
// E400: Document size limit exceeded
```

**Orphaned Blocks**
```rust
// Blocks unreachable from root (warning)
// E203: Orphaned block detected
```

## Working with Results

### Filter by Severity

```rust
let result = engine.validate(&doc);

// Get only errors
let errors = result.errors();
for error in errors {
    eprintln!("ERROR [{}]: {}", error.code, error.message);
}

// Get only warnings
let warnings = result.warnings();
for warning in warnings {
    println!("WARNING [{}]: {}", warning.code, warning.message);
}
```

### Check Validity

```rust
let result = engine.validate(&doc);

// Document is valid if no errors (warnings are OK)
if result.valid {
    println!("Ready to proceed");
} else {
    println!("Fix {} errors before continuing", result.errors().len());
}
```

### Merge Results

```rust
let mut result1 = validator.validate_document(&doc);
let result2 = validate_custom_rules(&doc);

result1.merge(result2);

// result1 now contains all issues from both validations
```

## Validating Block IDs

```rust
let validator = ValidationPipeline::new();

// Validate block ID format
match validator.validate_block_id("blk_a1b2c3d4e5f6a1b2c3d4e5f6") {
    Ok(id) => println!("Valid ID: {}", id),
    Err(e) => eprintln!("Invalid ID: {}", e),
}

// Invalid formats
assert!(validator.validate_block_id("invalid").is_err());
assert!(validator.validate_block_id("blk_xyz").is_err());
```

## Error Codes

| Code | Description |
|------|-------------|
| E001 | Block not found |
| E002 | Invalid block ID format |
| E201 | Cycle detected in structure |
| E203 | Orphaned block (warning) |
| E400 | Document size exceeded |
| E402 | Block size exceeded |
| E403 | Nesting depth exceeded |
| E404 | Edge count exceeded |

## Validation on Operations

The engine can validate after each operation:

```rust
use ucm_engine::{Engine, EngineConfig};

let config = EngineConfig {
    validate_on_operation: true,  // Enable (default)
    ..Default::default()
};

let engine = Engine::with_config(config);

// Each operation triggers validation
let result = engine.execute(&mut doc, operation)?;

if !result.success {
    // Operation failed validation
    println!("Failed: {:?}", result.error);
}
```

## Custom Validation

Extend validation with custom rules:

```rust
use ucm_engine::validate::ValidationResult;
use ucm_core::{ValidationIssue, ValidationSeverity, ErrorCode, Document};

fn validate_custom_rules(doc: &Document) -> ValidationResult {
    let mut issues = Vec::new();
    
    // Example: Require all blocks to have labels
    for block in doc.blocks.values() {
        if block.metadata.label.is_none() && !block.is_root() {
            issues.push(ValidationIssue::warning(
                ErrorCode::E200SchemaViolation,
                format!("Block {} has no label", block.id),
            ));
        }
    }
    
    // Example: Require at least one heading
    let has_heading = doc.blocks.values().any(|b| {
        b.metadata.semantic_role
            .as_ref()
            .map(|r| r.category.as_str().starts_with("heading"))
            .unwrap_or(false)
    });
    
    if !has_heading {
        issues.push(ValidationIssue::warning(
            ErrorCode::E202InvalidStructure,
            "Document has no headings".to_string(),
        ));
    }
    
    ValidationResult::invalid(issues)
}

// Use with standard validation
let mut result = engine.validate(&doc);
result.merge(validate_custom_rules(&doc));
```

## Complete Example

```rust
use ucm_engine::{Engine, Operation};
use ucm_engine::validate::{ValidationPipeline, ResourceLimits};
use ucm_core::{Content, Document, Block};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create validator with strict limits
    let limits = ResourceLimits {
        max_block_count: 100,
        max_nesting_depth: 5,
        max_block_size: 10_000,
        max_edges_per_block: 10,
        ..Default::default()
    };
    let validator = ValidationPipeline::with_limits(limits);
    
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Add some blocks
    for i in 0..10 {
        let block = Block::new(
            Content::text(&format!("Block {}", i)),
            Some("paragraph")
        );
        doc.add_block(block, &root)?;
    }
    
    // Validate
    let result = validator.validate_document(&doc);
    
    println!("Validation Result:");
    println!("  Valid: {}", result.valid);
    println!("  Errors: {}", result.errors().len());
    println!("  Warnings: {}", result.warnings().len());
    
    if !result.valid {
        println!("\nErrors:");
        for error in result.errors() {
            println!("  [{}] {}", error.code, error.message);
            if let Some(suggestion) = &error.suggestion {
                println!("      Suggestion: {}", suggestion);
            }
        }
    }
    
    if !result.warnings().is_empty() {
        println!("\nWarnings:");
        for warning in result.warnings() {
            println!("  [{}] {}", warning.code, warning.message);
        }
    }
    
    // Create orphan to demonstrate warning
    let orphan = Block::new(Content::text("Orphan"), None);
    let orphan_id = doc.add_block(orphan, &root)?;
    doc.remove_from_structure(&orphan_id);
    
    let result = validator.validate_document(&doc);
    println!("\nAfter creating orphan:");
    println!("  Valid: {}", result.valid); // Still valid (orphans are warnings)
    println!("  Warnings: {}", result.warnings().len());
    
    Ok(())
}
```

## Best Practices

### 1. Validate Before Serialization

```rust
let result = engine.validate(&doc);
if !result.valid {
    return Err("Cannot save invalid document".into());
}
save_document(&doc)?;
```

### 2. Use Appropriate Limits

```rust
// For user-generated content
let limits = ResourceLimits {
    max_block_count: 1_000,
    max_block_size: 100_000,
    ..Default::default()
};

// For machine-generated content
let limits = ResourceLimits {
    max_block_count: 100_000,
    max_block_size: 5_000_000,
    ..Default::default()
};
```

### 3. Handle Warnings Appropriately

```rust
let result = engine.validate(&doc);

// Errors are blockers
if !result.valid {
    return Err("Document has errors".into());
}

// Warnings may need attention
if !result.warnings().is_empty() {
    log::warn!("Document has {} warnings", result.warnings().len());
    for w in result.warnings() {
        log::warn!("  {}", w.message);
    }
}
```

### 4. Validate After Bulk Operations

```rust
// After import or bulk changes
doc.rebuild_indices();
let result = engine.validate(&doc);
```

### 5. Provide Suggestions

```rust
let issue = ValidationIssue::error(
    ErrorCode::E001BlockNotFound,
    format!("Block {} not found", block_id),
).with_suggestion("Check if the block was deleted or if the ID is correct");
```

## See Also

- [Operations](./operations.md) - Document operations
- [Documents](../ucm-core/documents.md) - Document structure
- [Error Handling](../ucm-core/README.md) - Error codes
