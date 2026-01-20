# Master Implementation Plan: UCM Advanced Features

## Overview

This document consolidates all major feature development tasks for the Unified Content Model (UCM) system. It provides evidence-based analysis of the current codebase architecture and outlines comprehensive implementation strategies for each feature, with special attention to performance, robustness, error handling, and edge cases.

## Table of Contents

1. [Section-Based Markdown Writing](#section-based-markdown-writing)
2. [HTML to UCM Translation](#html-to-ucm-translation) 
3. [Graph Traversal and Navigation](#graph-traversal-and-navigation)
4. [Context Management Infrastructure](#context-management-infrastructure)
5. [Cross-Cutting Concerns](#cross-cutting-concerns)
6. [Implementation Priorities](#implementation-priorities)

---

## Section-Based Markdown Writing

### Current State Analysis

**Evidence from codebase:**

The current system supports individual block editing through:

1. **Block-level operations** (`crates/ucm-engine/src/engine.rs`):
   - `Operation::Edit` - edits specific block content at paths like `content.text`
   - `Operation::Append` - adds new blocks to parents
   - Individual block manipulation through the engine

2. **Markdown translation** (`crates/translators/markdown/`):
   - `from_markdown.rs` - parses markdown into hierarchical block structures
   - `to_markdown.rs` - renders block documents back to markdown
   - Supports heading hierarchy (H1-H6) with proper parent-child relationships

3. **Document structure** uses hierarchical blocks where headings create sections:
   - Headings become section containers
   - Content under headings becomes child blocks
   - Maintains relative section structure

### Required Implementation

#### 1. New Operation Type: WriteSection

**Location:** `crates/ucm-engine/src/operation.rs`

```rust
/// Replace section contents with markdown
WriteSection {
    section_id: BlockId,  // Target section (heading block)
    markdown: String,      // New markdown content
    preserve_structure: bool,  // Keep existing subsection structure
},
```

#### 2. Engine Implementation

**Location:** `crates/ucm-engine/src/engine.rs`

```rust
fn execute_write_section(
    &self,
    doc: &mut Document,
    section_id: &BlockId,
    markdown: &str,
    preserve_structure: bool,
) -> Result<OperationResult> {
    // 1. Parse markdown into temporary document
    let temp_doc = MarkdownParser::new().parse(markdown)?;
    
    // 2. Remove existing children of section (except subsections if preserve_structure)
    self.clear_section_content(doc, section_id, preserve_structure)?;
    
    // 3. Integrate new blocks from temp_doc
    self.integrate_section_blocks(doc, section_id, &temp_doc)?;
    
    Ok(OperationResult::success(vec![*section_id]))
}
```

#### 3. Section Management Utilities

**New module:** `crates/ucm-engine/src/section.rs`

```rust
/// Clear section content before inserting new blocks
pub fn clear_section_content(
    doc: &mut Document,
    section_id: &BlockId,
) -> Result<()>

/// Integrate blocks from temp document into target section
pub fn integrate_section_blocks(
    doc: &mut Document,
    target_section: &BlockId,
    source_doc: &Document,
) -> Result<()>

/// Find section by path (e.g., "Section 1 > Subsection 2")
pub fn find_section_by_path(doc: &Document, path: &str) -> Option<BlockId>
```

#### 4. UCL Command Integration

**Location:** `crates/ucl-parser/src/ast.rs`

```rust
/// WRITE_SECTION command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteSectionCommand {
    pub section_id: String,
    pub markdown: String,
    pub base_heading_level: Option<usize>,
}
```

### Performance Considerations

1. **Markdown Parsing Efficiency**: 
   - Use streaming parser for large markdown content
   - Implement parsing limits to prevent DoS attacks
   - Cache parsed AST for repeated operations

2. **Block Integration**:
   - Batch block additions to minimize document re-indexing
   - Use efficient HashMap operations for block lookups
   - Implement incremental validation

3. **Memory Management**:
   - Clear temporary document after integration
   - Use references instead of copies where possible
   - Implement size limits for markdown input

### Error Handling

1. **Invalid Markdown**: Graceful parsing errors with location information
2. **Section Not Found**: Clear error messages with section path suggestions
3. **Circular References**: Detect and prevent during integration
4. **Permission Errors**: Access control for section modifications
5. **Size Limits**: Enforce reasonable limits on markdown content

### Edge Cases

1. **Empty Sections**: Handle gracefully without breaking structure
2. **Malformed Markdown**: Recover from parsing errors
3. **Deep Nesting**: Prevent excessive heading levels (>6)
4. **Mixed Content**: Handle text, code, tables in same section
5. **Concurrent Modifications**: Handle race conditions in multi-user scenarios

---

## HTML to UCM Translation

### Current State Analysis

**Evidence from codebase:**

The system currently has no HTML translation capabilities. However, it has:

1. **Content Types** (`crates/ucm-core/src/content.rs`):
   - `Content::Media` with `MediaSource` enum supporting URLs, Base64, References, External
   - `MediaSource::Url(String)` - for direct URL references
   - `MediaSource::External(ExternalRef)` - for cloud storage references

2. **Translation Infrastructure** (`crates/translators/markdown/`):
   - Established pattern for source-to-UCM translation
   - Parser/renderer architecture that can be extended
   - Content type mapping and semantic role assignment

3. **Edge System** (`crates/ucm-core/src/edge.rs`):
   - `EdgeType::LinksTo` for hyperlink relationships
   - `EdgeType::References` for cross-references
   - Edge metadata for storing additional link information

### Required Implementation

#### 1. HTML Translator Crate

**New crate:** `crates/translators/html/Cargo.toml`

```toml
[package]
name = "ucp-translator-html"
description = "HTML to UCM document translator"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
ucm-core = { workspace = true }
selectolax = "0.6"  # High-performance HTML parser
serde = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
```

#### 2. HTML Parser Implementation

**Location:** `crates/translators/html/src/lib.rs`

```rust
//! HTML to UCM document translator using selectolax
use selectolax::Node;
use ucm_core::{Block, Content, Document, Result, MediaSource};

#[derive(Debug, Error)]
pub enum HtmlError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Invalid HTML structure: {0}")]
    InvalidStructure(String),
}

/// HTML parser configuration
pub struct HtmlParserConfig {
    pub preserve_whitespace: bool,
    pub extract_images: bool,
    pub extract_links: bool,
    pub heading_strategy: HeadingStrategy,
}

pub enum HeadingStrategy {
    AsIs,           // Use actual heading levels (h1-h6)
    Flatten,        // Flatten all headings to h3
    InferFromNesting, // Infer hierarchy from nesting
}
```

#### 3. Content Extraction Logic

```rust
impl HtmlParser {
    pub fn parse(&self, html: &str) -> Result<Document> {
        let mut doc = Document::create();
        let html_tree = selectolax::parse_html().utf8().select(html)?;
        
        let body = html_tree.select_first("body").unwrap_or_else(|_| html_tree);
        self.process_node(&mut doc, &doc.root, &body, 0)?;
        
        Ok(doc)
    }
    
    fn process_node(&self, doc: &mut Document, parent_id: &BlockId, node: &Node, depth: usize) -> Result<BlockId> {
        match node.tag_name().as_deref() {
            Some("h1" | "h2" | "h3" | "h4" | "h5" | "h6") => {
                self.process_heading(doc, parent_id, node, depth)
            }
            Some("p") => self.process_paragraph(doc, parent_id, node),
            Some("img") => self.process_image(doc, parent_id, node),
            Some("a") => self.process_link(doc, parent_id, node),
            // ... other element types
        }
    }
}
```

#### 4. Media and Link Handling

```rust
fn process_image(&self, doc: &mut Document, parent_id: &BlockId, node: &Node) -> Result<BlockId> {
    if !self.config.extract_images {
        return Ok(*parent_id);
    }
    
    let src = node.get_attribute("src").unwrap_or_default();
    let alt = node.get_attribute("alt").unwrap_or_default();
    
    // Create media block with proper source handling
    let media_source = if src.starts_with("data:") {
        MediaSource::Base64(src.split(',').nth(1).unwrap_or_default().to_string())
    } else if src.starts_with("http://") || src.starts_with("https://") {
        MediaSource::Url(src.to_string())
    } else {
        MediaSource::Url(format!("https://{}", src)) // Handle relative URLs
    };
    
    let media_content = Content::media(&alt, media_source);
    let block = Block::new(media_content, Some("image"));
    doc.add_block(block, parent_id)
}

fn process_link(&self, doc: &mut Document, parent_id: &BlockId, node: &Node) -> Result<BlockId> {
    let href = node.get_attribute("href").unwrap_or_default();
    let text = node.text();
    
    // Create link block
    let block = Block::new(Content::text(&text), Some("link"));
    let block_id = doc.add_block(block, parent_id)?;
    
    // Add edge representing the link
    if let Ok(edge) = ucm_core::Edge::new(
        ucm_core::EdgeType::LinksTo,
        ucm_core::BlockId::from_bytes([0; 12]), // Placeholder for external ref
    ) {
        // Store href in edge metadata
        let mut edge_with_meta = edge;
        edge_with_meta.metadata.custom.insert("href".to_string(), serde_json::json!(href));
        
        if let Some(block_mut) = doc.get_block_mut(&block_id) {
            block_mut.add_edge(edge_with_meta);
        }
    }
    
    Ok(block_id)
}
```

### Performance Considerations

1. **HTML Parsing**: 
   - Use selectolax's streaming capabilities for large documents
   - Implement memory-efficient DOM traversal
   - Cache parsing results for repeated content

2. **Media Processing**:
   - Lazy loading of external resources
   - Size limits for images and embedded content
   - Async processing for network resources

3. **Document Construction**:
   - Batch block creation operations
   - Efficient parent-child relationship building
   - Minimal copying of content data

### Error Handling

1. **Malformed HTML**: Graceful recovery with partial parsing
2. **Invalid URLs**: Validation and sanitization of links
3. **Resource Limits**: Size and time limits for processing
4. **Network Errors**: Timeout and retry logic for external resources
5. **Encoding Issues**: Handle various character encodings properly

### Edge Cases

1. **Nested Tables**: Complex table structures with colspan/rowspan
2. **Script Content**: Handling or ignoring JavaScript/CSS
3. **Frames/iframes**: Proper handling of nested documents
4. **Invalid Markup**: Recovery from broken HTML
5. **Large Documents**: Memory-efficient processing of huge HTML files

---

## Graph Traversal and Navigation

### Current State Analysis

**Evidence from codebase:**

The current system has basic traversal capabilities:

1. **Document Structure** (`crates/ucm-core/src/document.rs`):
   - `doc.structure` HashMap for parent-child relationships
   - `doc.children()` and `doc.parent()` methods for navigation
   - BFS traversal in `crates/ucp-llm/src/id_mapper.rs` for LLM prompts

2. **Path Expressions** (`crates/ucl-parser/src/ast.rs`):
   - `Path` struct with segments for navigation
   - `PathSegment` enum supporting properties, indices, slices
   - Path-based editing operations

3. **Edge System** (`crates/ucm-core/src/edge.rs`):
   - Various edge types for relationships (References, LinksTo, etc.)
   - Edge metadata for additional relationship data
   - EdgeIndex for efficient edge lookups

4. **Validation** (`crates/ucm-engine/src/validate.rs`):
   - Cycle detection using DFS traversal
   - Maximum depth enforcement
   - Resource limits for large documents

### Required Implementation

#### 1. Traversal Engine

**New module:** `crates/ucm-engine/src/traversal.rs`

```rust
//! Graph traversal operations for UCM documents
use ucm_core::{Block, BlockId, Document};
use std::collections::{HashMap, HashSet, VecDeque};

/// Traversal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalResult {
    pub nodes: Vec<TraversalNode>,
    pub edges: Vec<TraversalEdge>,
    pub paths: Vec<Vec<BlockId>>,
    pub summary: TraversalSummary,
    pub metadata: TraversalMetadata,
}

/// Graph traversal engine
pub struct TraversalEngine {
    config: TraversalConfig,
}

#[derive(Debug, Clone)]
pub struct TraversalConfig {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub default_preview_length: usize,
    pub include_orphans: bool,
    pub cache_enabled: bool,
}
```

#### 2. Traversal Operations

```rust
impl TraversalEngine {
    pub fn navigate(
        &self,
        doc: &Document,
        start_id: Option<BlockId>,
        direction: NavigateDirection,
        depth: Option<usize>,
        filter: Option<TraversalFilter>,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        // Implementation with multiple traversal strategies
    }
    
    fn traverse_bfs(&self, doc: &Document, start: BlockId, max_depth: usize) -> Result<Vec<TraversalNode>> {
        let mut nodes = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, None, 0));
        
        while let Some((node_id, parent_id, depth)) = queue.pop_front() {
            if depth > max_depth || nodes.len() >= self.config.max_nodes {
                break;
            }
            // ... BFS traversal logic
        }
        Ok(nodes)
    }
}
```

#### 3. UCL Commands

**Location:** `crates/ucl-parser/src/ast.rs`

```rust
/// Navigation commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateCommand {
    pub start_id: Option<String>,
    pub direction: NavigateDirection,
    pub depth: Option<usize>,
    pub filter: Option<TraversalFilter>,
    pub output: TraversalOutput,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigateDirection {
    Down,           // Children only
    Up,             // Parent only  
    Both,           // Both directions
    Siblings,       // Same level
    BreadthFirst,   // BFS traversal
    DepthFirst,     // DFS traversal
}
```

### Performance Considerations

1. **Traversal Algorithms**:
   - Use iterative BFS/DFS to avoid stack overflow
   - Implement early termination conditions
   - Cache traversal results for repeated queries

2. **Memory Management**:
   - Stream large traversals to avoid memory spikes
   - Use weak references for cached data
   - Implement size limits for result sets

3. **Index Optimization**:
   - Pre-compute common traversal paths
   - Use adjacency lists for efficient graph operations
   - Implement incremental updates for dynamic graphs

### Error Handling

1. **Invalid Paths**: Clear error messages with path suggestions
2. **Circular References**: Detect and handle gracefully
3. **Permission Errors**: Access control for sensitive areas
4. **Resource Exhaustion**: Graceful degradation under load
5. **Invalid Filters**: Validation of traversal parameters

### Edge Cases

1. **Self-Loops**: Handle blocks that reference themselves
2. **Disconnected Graphs**: Handle orphaned subgraphs
3. **Very Deep Graphs**: Prevent stack overflow in recursive traversals
4. **Large Result Sets**: Pagination for traversal results
5. **Concurrent Modifications**: Handle graph changes during traversal

---

## Context Management Infrastructure

This section specifies platform capabilities rather than an autonomous agent. The goal is to expose UCM-native APIs that let higher-level orchestration layers load documents, traverse the knowledge graph, and curate context windows (adding or trimming sections) while preserving UCM invariants. External orchestration layers or services can then orchestrate these primitives to implement their own policies.

### Current State Analysis

**Evidence from codebase:**

The current system exposes foundational features that external orchestration layers could build upon:

1. **Prompt Building** (`crates/ucp-llm/src/prompt_builder.rs`):
   - `system_context` and `task_context` fields
   - `with_system_context()` and `with_task_context()` methods
   - Basic prompt construction for LLM interactions

2. **ID Mapping** (`crates/ucp-llm/src/id_mapper.rs`):
   - Token-efficient ID mapping for LLM prompts
   - `document_to_prompt()` method for normalized format
   - BFS traversal for context ordering

3. **Content Types** (`crates/ucm-core/src/content.rs`):
   - Various content types with size estimation
   - `Content::Media` with multiple source types
   - Token estimation capabilities

4. **Document Structure** (`crates/ucm-core/src/document.rs`):
   - Hierarchical block organization
   - Parent-child relationships for context building
   - Metadata for context relevance

### Required Implementation

#### 1. Context Management Crate

**New crate:** `crates/ucp-context/src/lib.rs`

```rust
//! Intelligent context management for UCM documents
use ucm_core::{Block, BlockId, Document};
use std::collections::{HashMap, HashSet};

/// Context window with intelligent management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub id: String,
    pub blocks: HashMap<BlockId, ContextBlock>,
    pub relationships: Vec<ContextRelation>,
    pub metadata: ContextMetadata,
    pub constraints: ContextConstraints,
}

/// Context Management Infrastructure
pub struct ContextManager {
    window: ContextWindow,
    document: Document,
    analyzer: ContextAnalyzer,
    strategy: ContextStrategy,
    cache: ContextCache,
}
```

#### 2. Context Operations

```rust
impl ContextManager {
    pub fn initialize_focus(&mut self, focus_id: BlockId, task_description: &str) -> Result<()> {
        // Add focus block and immediate context
        self.add_block_to_context(focus_id, InclusionReason::DirectReference, task_description)?;
        self.add_structural_context(focus_id, 2)?;
        self.add_semantic_context(focus_id, task_description)?;
        Ok(())
    }
    
    pub fn navigate_to(&mut self, target_id: BlockId, task_description: &str) -> Result<ContextUpdateResult> {
        // Navigate to new focus area with context updates
        self.add_block_to_context(target_id, InclusionReason::NavigationPath, task_description)?;
        self.add_structural_context(target_id, 2)?;
        self.add_semantic_context(target_id, task_description)?;
        self.prune_if_needed()?;
        Ok(ContextUpdateResult::default())
    }
    
    pub fn expand_context(&mut self, direction: ExpandDirection, depth: usize) -> Result<ContextUpdateResult> {
        // Expand context in specified direction
        let focus_id = self.window.metadata.focus_area.ok_or(Error::NoFocusArea)?;
        
        match direction {
            ExpandDirection::Down => self.expand_downward(focus_id, depth)?,
            ExpandDirection::Up => self.expand_upward(focus_id, depth)?,
            ExpandDirection::Both => {
                self.expand_downward(focus_id, depth)?;
                self.expand_upward(focus_id, depth)?;
            }
            ExpandDirection::Semantic => self.expand_semantic(focus_id, depth)?,
        }
        
        self.prune_if_needed()?;
        Ok(ContextUpdateResult::default())
    }
}
```

#### 3. Context Constraints and Policies

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConstraints {
    pub max_tokens: usize,
    pub max_blocks: usize,
    pub max_depth: usize,
    pub min_relevance: f32,
    pub required_roles: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub preserve_structure: bool,
    pub allow_compression: bool,
}

#[derive(Debug, Clone)]
pub enum ExpansionPolicy {
    Conservative,      // Only add highly relevant blocks
    Balanced,         // Balance relevance and diversity
    Aggressive,        // Add potentially useful blocks
    Adaptive,         // Adapt based on task complexity
}

#[derive(Debug, Clone)]
pub enum PruningPolicy {
    RelevanceFirst,    // Remove lowest relevance first
    RecencyFirst,      // Remove least recently accessed
    RedundancyFirst,  // Remove redundant content
    Strategic,        // Strategic removal based on structure
}
```

#### 4. UCL Commands

**Location:** `crates/ucl-parser/src/ast.rs`

```rust
/// Context management commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCommand {
    pub action: ContextAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextAction {
    Initialize {
        focus_id: String,
        task_description: String,
        constraints: Option<ContextConstraints>,
    },
    Navigate {
        target_id: String,
        task_description: String,
    },
    Expand {
        direction: String,  // "up", "down", "both", "semantic"
        depth: usize,
    },
    Add {
        block_id: String,
        reason: String,
    },
    Remove {
        block_id: String,
        rationale: String,
    },
    Search {
        query: String,
        max_results: usize,
    },
    Compress {
        method: String,  // "summarize", "truncate", "hybrid"
    },
}
```

### Performance Considerations

1. **Context Window Management**:
   - Efficient token counting and estimation
   - Lazy loading of block content
   - Incremental context updates

2. **Relevance Scoring**:
   - Cached relevance calculations
   - Efficient semantic similarity algorithms
   - Vector-based similarity for large documents

3. **Memory Optimization**:
   - Weak references for cached data
   - Streaming context for large windows
   - Garbage collection of unused context

### Error Handling

1. **Context Overflow**: Graceful degradation when limits exceeded
2. **Invalid Focus**: Clear error messages for missing blocks
3. **Semantic Analysis**: Handle failures in relevance calculation
4. **Compression Errors**: Fallback strategies for failed compression
5. **Navigation Errors**: Recovery from invalid navigation attempts

### Edge Cases

1. **Empty Documents**: Handle gracefully with default context
2. **Circular References**: Detect and handle in context building
3. **Very Large Documents**: Efficient processing of massive content
4. **Conflicting Constraints**: Resolve constraint conflicts intelligently
5. **Rapid Context Changes**: Handle frequent navigation efficiently

---

## Cross-Cutting Concerns

### Performance Optimization

#### 1. Memory Management

**Evidence from codebase:**
- `ResourceLimits` in `crates/ucm-engine/src/validate.rs` with size limits
- Efficient HashMap usage in document structure
- Streaming operations in markdown translation

**Implementation Strategy:**
```rust
pub struct PerformanceConfig {
    pub max_memory_mb: usize,
    pub max_operations_per_batch: usize,
    pub cache_size_mb: usize,
    pub enable_streaming: bool,
}

impl PerformanceConfig {
    pub fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_operations_per_batch: 1000,
            cache_size_mb: 64,
            enable_streaming: true,
        }
    }
}
```

#### 2. Caching Strategy

**Evidence from codebase:**
- `ContextCache` in context management design
- ID mapping cache in `crates/ucp-llm/src/id_mapper.rs`

**Implementation Strategy:**
```rust
pub struct CacheManager {
    pub block_cache: LruCache<BlockId, CachedBlock>,
    pub traversal_cache: HashMap<(BlockId, TraversalType), TraversalResult>,
    pub relevance_cache: HashMap<BlockId, f32>,
    pub semantic_cache: HashMap<String, Vec<BlockId>>,
}
```

#### 3. Async Operations

**Implementation Strategy:**
```rust
pub struct AsyncExecutor {
    pub thread_pool: ThreadPool,
    pub task_queue: VecDeque<AsyncTask>,
    pub max_concurrent: usize,
}

pub enum AsyncTask {
    ParseMarkdown(String),
    TraverseGraph(TraversalRequest),
    ComputeRelevance(RelevanceRequest),
    CompressContext(ContextCompressionRequest),
}
```

### Robustness and Error Handling

#### 1. Error Taxonomy

**Evidence from codebase:**
- Comprehensive error codes in `crates/ucm-core/src/error.rs`
- `Result<T>` types throughout the system
- Validation pipeline with detailed error reporting

**Enhanced Error Strategy:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum UcpError {
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Context constraint violation: {0}")]
    ContextFull(String),
    
    #[error("Traversal failed: {0}")]
    TraversalError(String),
    
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
    
    #[error("Performance limit exceeded: {limit}")]
    PerformanceLimit { limit: String },
    
    #[error("Concurrent modification: {0}")]
    ConcurrentModification(String),
}
```

#### 2. Validation Pipeline

**Evidence from codebase:**
- `ValidationPipeline` in `crates/ucm-engine/src/validate.rs`
- Resource limits and schema validation
- Cycle detection and structural validation

**Enhanced Validation:**
```rust
pub struct EnhancedValidationPipeline {
    pub limits: ResourceLimits,
    pub schema_validator: SchemaValidator,
    pub security_validator: SecurityValidator,
    pub performance_validator: PerformanceValidator,
}

impl EnhancedValidationPipeline {
    pub fn validate_document(&self, doc: &Document) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Existing validations
        issues.extend(self.validate_structure(doc));
        issues.extend(self.validate_resources(doc));
        
        // Enhanced validations
        issues.extend(self.security_validator.validate(doc));
        issues.extend(self.performance_validator.validate(doc));
        issues.extend(self.schema_validator.validate(doc));
        
        ValidationResult::new(issues)
    }
}
```

#### 3. Security Considerations

**Evidence from codebase:**
- `ErrorCode::E500PathTraversal` for path security
- `ErrorCode::E501DisallowedScheme` for URL validation
- Security error categories in error handling

**Security Strategy:**
```rust
pub struct SecurityValidator {
    pub allowed_schemes: HashSet<String>,
    pub max_url_length: usize,
    pub block_external_resources: bool,
    pub sanitize_content: bool,
}

impl SecurityValidator {
    pub fn validate(&self, doc: &Document) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Validate all URLs in media content
        for block in doc.blocks.values() {
            if let Content::Media(media) = &block.content {
                if let MediaSource::Url(url) = &media.source {
                    self.validate_url(url, &mut issues);
                }
            }
        }
        
        // Check for path traversal attempts
        self.validate_paths(doc, &mut issues);
        
        // Validate content size limits
        self.validate_content_sizes(doc, &mut issues);
        
        issues
    }
}
```

### Testing Strategy

#### 1. Unit Testing

**Evidence from codebase:**
- Comprehensive test suites in all modules
- Property-based testing with proptest
- Golden file testing for markdown translation

**Testing Strategy:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_section_writing_basic() {
        let mut doc = Document::create();
        let section = doc.add_block(
            Block::new(Content::text("Section Title"), Some("heading1")),
            &doc.root
        ).unwrap();
        
        let markdown = r#"New content
With multiple lines
- List item 1
- List item 2"#;
        
        let result = write_section(&mut doc, section, markdown, false);
        assert!(result.is_success());
    }
    
    proptest! {
        fn test_section_writing_properties(
            markdown in "\\PC*",
            preserve_structure in any::<bool>(),
        ) {
            // Property-based test for section writing
        }
    }
}
```

#### 2. Integration Testing

**Integration Test Strategy:**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_html_to_ucm_roundtrip() {
        let html = r#"<html><body>
            <h1>Title</h1>
            <p>Paragraph with <a href="https://example.com">link</a></p>
            <img src="test.jpg" alt="Test image">
        </body></html>"#;
        
        let parser = HtmlParser::new();
        let doc = parser.parse(html).unwrap();
        
        // Convert back to HTML (if implemented)
        let reconstructed = render_html(&doc).unwrap();
        
        // Validate key elements are preserved
        assert!(reconstructed.contains("Title"));
        assert!(reconstructed.contains("link"));
        assert!(reconstructed.contains("test.jpg"));
    }
    
    #[test]
    fn test_context_management_workflow() {
        let mut doc = create_test_document();
        let mut context = ContextManager::new(doc, default_constraints());
        
        // Initialize context
        context.initialize_focus(focus_block, "Test task").unwrap();
        
        // Navigate and expand
        context.navigate_to(target_block, "New task").unwrap();
        context.expand_context(ExpandDirection::Down, 2).unwrap();
        
        // Verify context constraints
        let stats = context.get_statistics();
        assert!(stats.total_tokens <= 4000);
        assert!(stats.total_blocks <= 50);
    }
}
```

#### 3. Performance Testing

**Performance Test Strategy:**
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_large_document_traversal() {
        let large_doc = create_large_document(10000); // 10k blocks
        let engine = TraversalEngine::default();
        
        let start = Instant::now();
        let result = engine.navigate(
            &large_doc,
            None,
            NavigateDirection::BreadthFirst,
            Some(5),
            None,
            TraversalOutput::StructureAndBlocks
        ).unwrap();
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000); // Should complete in < 1s
        assert!(result.nodes.len() <= 1000); // Respect limits
    }
    
    #[test]
    fn test_context_management_performance() {
        let mut context = ContextManager::new(
            create_test_document(),
            ContextConstraints {
                max_tokens: 4000,
                max_blocks: 100,
                ..Default::default()
            }
        );
        
        let start = Instant::now();
        
        // Simulate rapid context changes
        for i in 0..100 {
            let block_id = format!("blk_{:04}", i);
            context.add_block(
                BlockId::from_str(&block_id).unwrap(),
                InclusionReason::ExternalDecision,
                format!("Test iteration {}", i)
            ).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500); // Should handle 100 ops in < 500ms
    }
}
```

---

## Implementation Priorities

### Phase 1: Core Infrastructure (Weeks 1-2)

1. **Section Writing Foundation**
   - Implement `WriteSection` operation
   - Basic markdown parsing integration
   - Section management utilities
   - UCL command support

2. **HTML Translation Basics**
   - HTML parser crate setup
   - Basic element parsing (headings, paragraphs, links)
   - Media content extraction
   - Simple document construction

### Phase 2: Advanced Features (Weeks 3-4)

1. **Enhanced Section Writing**
   - Preserve subsection structure option
   - Heading level adjustment
   - Error handling and validation
   - Performance optimization

2. **Complete HTML Translation**
   - Full HTML element support
   - Table and form handling
   - CSS and script processing options
   - Link relationship management

### Phase 3: Graph Navigation (Weeks 5-6)

1. **Traversal Engine**
   - Multiple traversal algorithms
   - Filtering and search capabilities
   - Path generation and analysis
   - Performance optimization

2. **Navigation Commands**
   - UCL command integration
   - Interactive navigation
   - Visualization support
   - Edge case handling

### Phase 4: Context Management (Weeks 7-8)

1. **Context Management APIs**
   - Intelligent context building primitives
   - Relevance scoring services
   - Constraint management utilities
   - Compression strategies

2. **Integration Layer**
   - LLM workflow integration
   - Context-aware prompting
   - Dynamic context adjustment
   - Performance monitoring

### Phase 5: Polish and Optimization (Weeks 9-10)

1. **Performance Optimization**
   - Caching strategies
   - Memory management
   - Async operations
   - Resource limits

2. **Robustness and Testing**
   - Comprehensive test suites
   - Error handling refinement
   - Security validation
   - Documentation completion

---

## Conclusion

This master implementation plan provides a comprehensive roadmap for extending the UCM system with advanced features while maintaining the existing architecture's strengths. The plan emphasizes:

1. **Performance**: Efficient algorithms, caching, and resource management
2. **Robustness**: Comprehensive error handling and validation
3. **Extensibility**: Modular design that supports future enhancements
4. **Integration**: Seamless compatibility with existing UCL and SDK interfaces
5. **Testing**: Thorough testing strategy including unit, integration, and performance tests

Each feature builds upon the existing codebase strengths while addressing the identified limitations. The implementation timeline allows for incremental development with regular validation and testing at each phase.

The success of this plan depends on maintaining the existing code quality standards and ensuring that all new features integrate seamlessly with the current UCM architecture.
