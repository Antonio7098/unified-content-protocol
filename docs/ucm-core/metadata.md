# Metadata

Block metadata provides rich information for search, display, and LLM optimization. This includes semantic roles, tags, labels, token estimates, and custom properties.

## Block Metadata Structure

```rust
pub struct BlockMetadata {
    /// Semantic role in document structure
    pub semantic_role: Option<SemanticRole>,
    
    /// Human-readable label (unique within document)
    pub label: Option<String>,
    
    /// Searchable tags
    pub tags: Vec<String>,
    
    /// Pre-computed summary for folding/context management
    pub summary: Option<String>,
    
    /// Estimated token count (computed lazily)
    pub token_estimate: Option<TokenEstimate>,
    
    /// Content hash for change detection
    pub content_hash: ContentHash,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    
    /// Custom key-value metadata
    pub custom: HashMap<String, serde_json::Value>,
}
```

## Creating Metadata

### Basic Creation

```rust
use ucm_core::metadata::BlockMetadata;
use ucm_core::id::compute_content_hash;
use ucm_core::Content;

let content = Content::text("Hello, world!");
let hash = compute_content_hash(&content);

let metadata = BlockMetadata::new(hash);
```

### Builder Pattern

```rust
use ucm_core::metadata::{BlockMetadata, SemanticRole, RoleCategory};
use ucm_core::id::compute_content_hash;

let hash = compute_content_hash(&content);

let metadata = BlockMetadata::new(hash)
    .with_role(SemanticRole::new(RoleCategory::Intro))
    .with_label("introduction")
    .with_tag("important")
    .with_tags(["draft", "review-needed"])
    .with_summary("Brief introduction to the topic")
    .with_custom("author", serde_json::json!("Alice"));
```

## Semantic Roles

Semantic roles describe a block's function in the document structure.

### Structure

```rust
pub struct SemanticRole {
    /// Primary category
    pub category: RoleCategory,
    
    /// Subcategory (optional)
    pub subcategory: Option<String>,
    
    /// Custom qualifier
    pub qualifier: Option<String>,
}
```

### Creating Semantic Roles

```rust
use ucm_core::metadata::{SemanticRole, RoleCategory};

// Simple role
let role = SemanticRole::new(RoleCategory::Intro);

// With subcategory
let role = SemanticRole::new(RoleCategory::Intro)
    .with_subcategory("hook");

// With qualifier
let role = SemanticRole::new(RoleCategory::Body)
    .with_subcategory("argument")
    .with_qualifier("v2");

// Parse from string
let role = SemanticRole::parse("intro.hook").unwrap();
let role = SemanticRole::parse("body.argument.v2").unwrap();

// Display
println!("{}", role); // "body.argument.v2"
```

### Role Categories

#### Document Structure

| Category | Description | String |
|----------|-------------|--------|
| `Title` | Document title | `title` |
| `Subtitle` | Document subtitle | `subtitle` |
| `Abstract` | Document abstract | `abstract` |
| `TableOfContents` | Table of contents | `toc` |

#### Headings

| Category | Description | String |
|----------|-------------|--------|
| `Heading1` | H1 heading | `heading1`, `h1` |
| `Heading2` | H2 heading | `heading2`, `h2` |
| `Heading3` | H3 heading | `heading3`, `h3` |
| `Heading4` | H4 heading | `heading4`, `h4` |
| `Heading5` | H5 heading | `heading5`, `h5` |
| `Heading6` | H6 heading | `heading6`, `h6` |

#### Content Structure

| Category | Description | String |
|----------|-------------|--------|
| `Paragraph` | Regular paragraph | `paragraph`, `para`, `p` |
| `List` | List content | `list`, `ul`, `ol` |

#### Introduction Elements

| Category | Description | String |
|----------|-------------|--------|
| `Intro` | Introduction section | `intro`, `introduction` |
| `IntroHook` | Opening hook | `intro_hook`, `hook` |
| `IntroContext` | Background context | `intro_context`, `context` |
| `IntroThesis` | Thesis statement | `intro_thesis`, `thesis` |

#### Body Elements

| Category | Description | String |
|----------|-------------|--------|
| `Body` | Body section | `body` |
| `BodyArgument` | Main argument | `body_argument`, `argument` |
| `BodyEvidence` | Supporting evidence | `body_evidence`, `evidence` |
| `BodyExample` | Example | `body_example`, `example` |
| `BodyCounterargument` | Counterargument | `body_counterargument` |
| `BodyTransition` | Transition | `body_transition`, `transition` |

#### Conclusion Elements

| Category | Description | String |
|----------|-------------|--------|
| `Conclusion` | Conclusion section | `conclusion` |
| `ConclusionSummary` | Summary | `conclusion_summary`, `summary` |
| `ConclusionImplication` | Implications | `conclusion_implication` |
| `ConclusionCallToAction` | Call to action | `conclusion_cta`, `cta` |

#### Special Sections

| Category | Description | String |
|----------|-------------|--------|
| `Sidebar` | Sidebar content | `sidebar` |
| `Callout` | Callout box | `callout` |
| `Warning` | Warning message | `warning` |
| `Note` | Note | `note` |
| `Quote` | Block quote | `quote`, `blockquote` |

#### Technical Elements

| Category | Description | String |
|----------|-------------|--------|
| `Definition` | Definition | `definition` |
| `Theorem` | Theorem | `theorem` |
| `Proof` | Proof | `proof` |
| `Algorithm` | Algorithm | `algorithm` |
| `Code` | Code block | `code` |

#### Meta Elements

| Category | Description | String |
|----------|-------------|--------|
| `Metadata` | Metadata block | `metadata`, `meta` |
| `Citation` | Citation | `citation`, `cite` |
| `Footnote` | Footnote | `footnote` |
| `Appendix` | Appendix | `appendix` |
| `Reference` | Reference | `reference`, `ref` |

### Using Roles in Blocks

```rust
use ucm_core::{Block, Content};

// Via Block::new
let block = Block::new(Content::text("Introduction"), Some("intro.hook"));

// The role is automatically parsed
let role = block.metadata.semantic_role.as_ref().unwrap();
assert_eq!(role.category, RoleCategory::Intro);
```

## Labels

Labels provide unique, human-readable identifiers for blocks.

```rust
let block = Block::new(Content::text("Important"), None)
    .with_label("main-warning");

// Find by label
if let Some(id) = doc.indices.find_by_label("main-warning") {
    let block = doc.get_block(&id).unwrap();
}
```

### Label Best Practices

```rust
// Good - descriptive, unique
.with_label("chapter-1-intro")
.with_label("api-overview")
.with_label("warning-deprecated")

// Less ideal - generic, may conflict
.with_label("intro")
.with_label("section1")
```

## Tags

Tags enable categorization and filtering.

```rust
let block = Block::new(Content::text("Draft content"), None)
    .with_tag("draft")
    .with_tag("needs-review")
    .with_tag("chapter-1");

// Check for tag
assert!(block.has_tag("draft"));

// Find by tag
let drafts = doc.indices.find_by_tag("draft");
```

### Tag Best Practices

```rust
// Good - consistent naming
.with_tag("status:draft")
.with_tag("status:final")
.with_tag("priority:high")
.with_tag("author:alice")

// Useful categories
.with_tag("needs-review")
.with_tag("deprecated")
.with_tag("experimental")
.with_tag("public-api")
```

## Token Estimation

Token estimates help with LLM context management.

### Structure

```rust
pub struct TokenEstimate {
    /// Estimated tokens for GPT-4 tokenizer
    pub gpt4: u32,
    
    /// Estimated tokens for Claude tokenizer
    pub claude: u32,
    
    /// Estimated tokens for Llama tokenizer
    pub llama: u32,
    
    /// Generic estimate (average)
    pub generic: u32,
}
```

### Computing Estimates

```rust
use ucm_core::metadata::{TokenEstimate, TokenModel};
use ucm_core::Content;

let content = Content::text("Hello, world! This is a test.");
let estimate = TokenEstimate::compute(&content);

println!("GPT-4: {} tokens", estimate.gpt4);
println!("Claude: {} tokens", estimate.claude);
println!("Llama: {} tokens", estimate.llama);
println!("Generic: {} tokens", estimate.generic);

// Get for specific model
let tokens = estimate.for_model(TokenModel::GPT4);
```

### Token Models

```rust
pub enum TokenModel {
    GPT4,
    Claude,
    Llama,
    Generic,
}
```

### Estimation Details

Token estimation considers:

- **Text**: Word count, character count, CJK character ratio
- **Code**: Line count, character count, language-specific adjustments
- **Tables**: Cell count, header tokens, structure tokens
- **JSON**: Serialized length

```rust
// CJK text has different tokenization
let cjk_estimate = TokenEstimate::compute(&Content::text("你好世界"));

// Code has language-specific adjustments
let rust_estimate = TokenEstimate::compute(&Content::code("rust", "fn main() {}"));
let python_estimate = TokenEstimate::compute(&Content::code("python", "def main(): pass"));
```

### Document-Level Tokens

```rust
use ucm_core::metadata::TokenModel;

let total_gpt4 = doc.total_tokens(TokenModel::GPT4);
let total_claude = doc.total_tokens(TokenModel::Claude);

println!("Document has {} GPT-4 tokens", total_gpt4);
```

## Summaries

Summaries enable content folding for context management.

```rust
let metadata = BlockMetadata::new(hash)
    .with_summary("Brief overview of the algorithm's complexity analysis");

// Use summary when folding content
if let Some(summary) = &block.metadata.summary {
    println!("Folded: {}", summary);
}
```

## Content Hash

The content hash enables change detection.

```rust
use ucm_core::id::{compute_content_hash, ContentHash};

let content = Content::text("Hello");
let hash = compute_content_hash(&content);

// Hash is stored in metadata
let metadata = BlockMetadata::new(hash);

// Compare hashes to detect changes
let new_hash = compute_content_hash(&new_content);
if metadata.content_hash != new_hash {
    println!("Content has changed!");
}
```

## Timestamps

```rust
// Creation time is set automatically
let metadata = BlockMetadata::new(hash);
println!("Created: {}", metadata.created_at);

// Update modification time
metadata.touch();
println!("Modified: {}", metadata.modified_at);
```

## Custom Metadata

Store arbitrary key-value pairs:

```rust
let metadata = BlockMetadata::new(hash)
    .with_custom("author", serde_json::json!("Alice"))
    .with_custom("version", serde_json::json!(2))
    .with_custom("config", serde_json::json!({
        "highlight": true,
        "language": "en"
    }));

// Access custom metadata
if let Some(author) = metadata.custom.get("author") {
    println!("Author: {}", author);
}
```

## Complete Example

```rust
use ucm_core::{Block, Content, Document};
use ucm_core::metadata::{BlockMetadata, SemanticRole, RoleCategory, TokenEstimate, TokenModel};
use ucm_core::id::compute_content_hash;

fn main() {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create block with rich metadata
    let content = Content::text(
        "This chapter introduces the core concepts of memory safety in Rust."
    );
    let hash = compute_content_hash(&content);
    
    let metadata = BlockMetadata::new(hash)
        .with_role(SemanticRole::new(RoleCategory::Intro)
            .with_subcategory("overview"))
        .with_label("chapter-1-intro")
        .with_tags(["chapter-1", "memory-safety", "introduction"])
        .with_summary("Introduction to Rust memory safety concepts")
        .with_custom("difficulty", serde_json::json!("beginner"))
        .with_custom("estimated_read_time_minutes", serde_json::json!(5));
    
    let block = Block::new(content, Some("intro.overview"))
        .with_metadata(metadata);
    
    let block_id = doc.add_block(block, &root).unwrap();
    
    // Query by various criteria
    let intros = doc.indices.find_by_tag("introduction");
    let chapter1 = doc.indices.find_by_tag("chapter-1");
    let by_label = doc.indices.find_by_label("chapter-1-intro");
    
    // Get token estimate
    let block = doc.get_block(&block_id).unwrap();
    let tokens = block.token_estimate();
    println!("GPT-4 tokens: {}", tokens.for_model(TokenModel::GPT4));
    
    // Check semantic role
    if let Some(role) = &block.metadata.semantic_role {
        println!("Role: {}", role); // "intro.overview"
    }
}
```

## Best Practices

### 1. Use Semantic Roles Consistently

```rust
// Good - clear document structure
Block::new(content, Some("heading1"))
Block::new(content, Some("intro.hook"))
Block::new(content, Some("body.evidence"))

// Less ideal - no semantic information
Block::new(content, None)
```

### 2. Use Labels for Key Blocks

```rust
// Blocks you'll need to reference later
.with_label("main-thesis")
.with_label("conclusion-summary")
.with_label("api-example-1")
```

### 3. Use Tags for Categories

```rust
// Consistent tag naming
.with_tag("status:draft")
.with_tag("type:example")
.with_tag("topic:memory-safety")
```

### 4. Provide Summaries for Long Content

```rust
// For blocks that might be folded
.with_summary("Detailed analysis of O(n log n) sorting algorithms")
```

### 5. Store Domain-Specific Data in Custom

```rust
// Application-specific metadata
.with_custom("review_status", serde_json::json!("approved"))
.with_custom("last_reviewer", serde_json::json!("bob@example.com"))
```

## See Also

- [Semantic Roles](./semantic-roles.md) - Complete semantic role reference
- [Blocks](./blocks.md) - Block structure
- [ID Generation](./id-generation.md) - Content hashing
- [Documents](./documents.md) - Document indices
