# Metadata

Block metadata provides rich information for search, display, and LLM optimization. This includes semantic roles, tags, labels, token estimates, and custom properties.

## Block Metadata Structure

=== "Rust"
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

=== "Python"
    In Python, metadata is accessed via properties on the `Block` object or managed through `Document` methods.

    ```python
    block = doc.get_block(block_id)
    print(block.role)
    print(block.label)
    print(block.tags)
    ```

=== "JavaScript"
    In JavaScript, metadata fields are properties on the block object.

    ```javascript
    const block = doc.getBlock(blockId);
    console.log(block.role);
    console.log(block.label);
    console.log(block.tags);
    ```

## Creating Metadata

### Basic Creation

=== "Rust"
    ```rust
    use ucm_core::metadata::BlockMetadata;
    use ucm_core::id::compute_content_hash;
    use ucm_core::Content;

    let content = Content::text("Hello, world!");
    let hash = compute_content_hash(&content);

    let metadata = BlockMetadata::new(hash);
    ```

=== "Python"
    Metadata is typically created implicitly when adding blocks to a document.

    ```python
    doc.add_block(parent, "Hello, world!", role="intro")
    ```

=== "JavaScript"
    Metadata is typically created implicitly when adding blocks to a document.

    ```javascript
    doc.addBlock(parent, "Hello, world!", "intro");
    ```

### Builder Pattern

=== "Rust"
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

=== "Python"
    Use `Document` methods to set metadata after block creation.

    ```python
    block_id = doc.add_block(parent, content, role="intro")
    doc.set_label(block_id, "introduction")
    doc.add_tag(block_id, "important")
    doc.add_tag(block_id, "draft")
    ```

=== "JavaScript"
    Use `Document` methods to set metadata after block creation.

    ```javascript
    const blockId = doc.addBlock(parent, content, "intro");
    doc.setLabel(blockId, "introduction");
    doc.addTag(blockId, "important");
    doc.addTag(blockId, "draft");
    ```

## Semantic Roles

Semantic roles describe a block's function in the document structure.

### Structure

=== "Rust"
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

=== "Rust"
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

=== "Python"
    Roles are passed as dot-separated strings.

    ```python
    # Simple role
    doc.add_block(parent, "...", role="intro")

    # With subcategory
    doc.add_block(parent, "...", role="intro.hook")

    # With qualifier
    doc.add_block(parent, "...", role="body.argument.v2")
    ```

=== "JavaScript"
    Roles are passed as dot-separated strings.

    ```javascript
    // Simple role
    doc.addBlock(parent, "...", "intro");

    // With subcategory
    doc.addBlock(parent, "...", "intro.hook");

    // With qualifier
    doc.addBlock(parent, "...", "body.argument.v2");
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

=== "Rust"
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

=== "Rust"
    ```rust
    let block = Block::new(Content::text("Important"), None)
        .with_label("main-warning");

    // Find by label
    if let Some(id) = doc.indices.find_by_label("main-warning") {
        let block = doc.get_block(&id).unwrap();
    }
    ```

=== "Python"
    ```python
    doc.add_block(parent, "Important", label="main-warning")
    
    # Find by label
    id = doc.find_by_label("main-warning")
    ```

=== "JavaScript"
    ```javascript
    doc.addBlock(parent, "Important", null, "main-warning");
    
    // Find by label
    const id = doc.findByLabel("main-warning");
    ```

### Label Best Practices

=== "Rust"
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

=== "Rust"
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

=== "Python"
    ```python
    doc.add_tag(block_id, "draft")
    doc.add_tag(block_id, "needs-review")
    
    # Find by tag
    drafts = doc.find_by_tag("draft")
    ```

=== "JavaScript"
    ```javascript
    doc.addTag(blockId, "draft");
    doc.addTag(blockId, "needs-review");
    
    // Find by tag
    const drafts = doc.findByTag("draft");
    ```

### Tag Best Practices

=== "Rust"
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

=== "Rust"
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

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import IdMapper
    
    mapper = IdMapper()
    # Returns (original, shortened, savings)
    stats = mapper.estimate_token_savings("Hello, world!")
    print(f"Generic tokens: {stats[0]}")
    ```

=== "JavaScript"
    ```javascript
    import { IdMapper } from 'ucp-content';
    
    const mapper = new IdMapper();
    // Returns { originalTokens, shortenedTokens, savings }
    const stats = mapper.estimateTokenSavings("Hello, world!");
    console.log(`Generic tokens: ${stats.originalTokens}`);
    ```

### Token Models

=== "Rust"
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

=== "Rust"
    ```rust
    // CJK text has different tokenization
    let cjk_estimate = TokenEstimate::compute(&Content::text("你好世界"));

    // Code has language-specific adjustments
    let rust_estimate = TokenEstimate::compute(&Content::code("rust", "fn main() {}"));
    let python_estimate = TokenEstimate::compute(&Content::code("python", "def main(): pass"));
    ```

### Document-Level Tokens

=== "Rust"
    ```rust
    use ucm_core::metadata::TokenModel;

    let total_gpt4 = doc.total_tokens(TokenModel::GPT4);
    let total_claude = doc.total_tokens(TokenModel::Claude);

    println!("Document has {} GPT-4 tokens", total_gpt4);
    ```

## Summaries

Summaries enable content folding for context management.

=== "Rust"
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

=== "Rust"
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

=== "Rust"
    ```rust
    // Creation time is set automatically
    let metadata = BlockMetadata::new(hash);
    println!("Created: {}", metadata.created_at);

    // Update modification time
    metadata.touch();
    println!("Modified: {}", metadata.modified_at);
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    print(f"Created: {block.created_at}")
    print(f"Modified: {block.modified_at}")
    ```

=== "JavaScript"
    ```javascript
    // Timestamp properties are ISO strings on document metadata
    // but individual block timestamps may not be directly exposed in JS v0.1.6
    ```

## Custom Metadata

Store arbitrary key-value pairs:

=== "Rust"
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

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document
    
    doc = Document.create()
    root = doc.root_id
    
    # Create block
    block_id = doc.add_block(
        root, 
        "This chapter introduces the core concepts of memory safety in Rust.",
        role="intro.overview",
        label="chapter-1-intro",
        tags=["chapter-1", "memory-safety", "introduction"]
    )
    
    # Query
    intros = doc.find_by_tag("introduction")
    chap1 = doc.find_by_tag("chapter-1")
    intro_block = doc.find_by_label("chapter-1-intro")
    
    # Check block properties
    if intro_block:
        block = doc.get_block(intro_block)
        print(f"Role: {block.role}")
        print(f"Tags: {block.tags}")
    ```

## Best Practices

### 1. Use Semantic Roles Consistently

=== "Rust"
    ```rust
    // Good - clear document structure
    Block::new(content, Some("heading1"))
    Block::new(content, Some("intro.hook"))
    Block::new(content, Some("body.evidence"))

    // Less ideal - no semantic information
    Block::new(content, None)
    ```

### 2. Use Labels for Key Blocks

=== "Rust"
    ```rust
    // Blocks you'll need to reference later
    .with_label("main-thesis")
    .with_label("conclusion-summary")
    .with_label("api-example-1")
    ```

### 3. Use Tags for Categories

=== "Rust"
    ```rust
    // Consistent tag naming
    .with_tag("status:draft")
    .with_tag("type:example")
    .with_tag("topic:memory-safety")
    ```

### 4. Provide Summaries for Long Content

=== "Rust"
    ```rust
    // For blocks that might be folded
    .with_summary("Detailed analysis of O(n log n) sorting algorithms")
    ```

### 5. Store Domain-Specific Data in Custom

=== "Rust"
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
