# Semantic Roles

Semantic roles describe a block's function within a document's structure. This page provides comprehensive documentation for all semantic roles available in UCP.

## Overview

Semantic roles help:
- Organize documents hierarchically and semantically
- Enable intelligent content folding and summarization
- Support LLM context management
- Facilitate structured rendering and export

## Python SDK Usage

```python
from ucp import SemanticRole, Block, create

# Create blocks with semantic roles
intro = Block.text("Welcome to our guide", role=SemanticRole.INTRO)
warning = Block.text("Do not run in production", role=SemanticRole.WARNING)

# Use in documents
doc = create()
doc.add_block(doc.root_id, "Important note", role=SemanticRole.NOTE)
```

## Role Categories

### Headings

For document headings and titles at various levels.

| Role | Value | Description |
|------|-------|-------------|
| `HEADING1` | `heading1` | Top-level heading (H1) |
| `HEADING2` | `heading2` | Second-level heading (H2) |
| `HEADING3` | `heading3` | Third-level heading (H3) |
| `HEADING4` | `heading4` | Fourth-level heading (H4) |
| `HEADING5` | `heading5` | Fifth-level heading (H5) |
| `HEADING6` | `heading6` | Sixth-level heading (H6) |

```python
h1 = Block.text("Main Title", role=SemanticRole.HEADING1)
h2 = Block.text("Chapter 1", role=SemanticRole.HEADING2)
```

### Content Structure

Basic content organization roles.

| Role | Value | Description |
|------|-------|-------------|
| `PARAGRAPH` | `paragraph` | Standard text paragraph |
| `QUOTE` | `quote` | Block quote |
| `LIST` | `list` | List content (ordered or unordered) |

```python
para = Block.text("This is a paragraph.", role=SemanticRole.PARAGRAPH)
quote = Block.text("To be or not to be", role=SemanticRole.QUOTE)
```

### Technical Content

For code, math, and structured data.

| Role | Value | Description |
|------|-------|-------------|
| `CODE` | `code` | Code block |
| `TABLE` | `table` | Table data |
| `EQUATION` | `equation` | Mathematical equation |

```python
code = Block.code("print('Hello')", language="python")  # Automatically uses CODE role
equation = Block.text("E = mc^2", role=SemanticRole.EQUATION)
```

### Document Structure

High-level document organization.

| Role | Value | Description |
|------|-------|-------------|
| `TITLE` | `title` | Document title |
| `SUBTITLE` | `subtitle` | Document subtitle |
| `ABSTRACT` | `abstract` | Document abstract/summary |
| `SECTION` | `section` | Generic section container |

```python
title = Block.text("Research Paper Title", role=SemanticRole.TITLE)
abstract = Block.text("This paper explores...", role=SemanticRole.ABSTRACT)
```

### Narrative Structure

For essay/article organization.

| Role | Value | Description |
|------|-------|-------------|
| `INTRO` | `intro` | Introduction section |
| `BODY` | `body` | Main body content |
| `CONCLUSION` | `conclusion` | Conclusion section |

```python
intro = Block.text("In this article, we will...", role=SemanticRole.INTRO)
body = Block.text("The main argument is...", role=SemanticRole.BODY)
conclusion = Block.text("In summary...", role=SemanticRole.CONCLUSION)
```

### Callouts and Special Sections

For highlighted content and notifications.

| Role | Value | Description |
|------|-------|-------------|
| `NOTE` | `note` | Informational note |
| `WARNING` | `warning` | Warning message |
| `TIP` | `tip` | Helpful tip or hint |
| `SIDEBAR` | `sidebar` | Sidebar content |
| `CALLOUT` | `callout` | Generic callout box |

```python
note = Block.text("Remember to save your work", role=SemanticRole.NOTE)
warning = Block.text("This action cannot be undone", role=SemanticRole.WARNING)
tip = Block.text("Pro tip: Use keyboard shortcuts", role=SemanticRole.TIP)
```

### Meta Elements

For references and supplementary content.

| Role | Value | Description |
|------|-------|-------------|
| `METADATA` | `metadata` | Metadata block |
| `CITATION` | `citation` | Citation or reference |
| `FOOTNOTE` | `footnote` | Footnote content |

```python
citation = Block.text("[1] Author, Title, 2024", role=SemanticRole.CITATION)
footnote = Block.text("See appendix for details", role=SemanticRole.FOOTNOTE)
```

## Complete Role List

| Role | Value | Category |
|------|-------|----------|
| `HEADING1` | `heading1` | Headings |
| `HEADING2` | `heading2` | Headings |
| `HEADING3` | `heading3` | Headings |
| `HEADING4` | `heading4` | Headings |
| `HEADING5` | `heading5` | Headings |
| `HEADING6` | `heading6` | Headings |
| `PARAGRAPH` | `paragraph` | Content |
| `QUOTE` | `quote` | Content |
| `LIST` | `list` | Content |
| `CODE` | `code` | Technical |
| `TABLE` | `table` | Technical |
| `EQUATION` | `equation` | Technical |
| `TITLE` | `title` | Document |
| `SUBTITLE` | `subtitle` | Document |
| `ABSTRACT` | `abstract` | Document |
| `SECTION` | `section` | Document |
| `INTRO` | `intro` | Narrative |
| `BODY` | `body` | Narrative |
| `CONCLUSION` | `conclusion` | Narrative |
| `NOTE` | `note` | Callouts |
| `WARNING` | `warning` | Callouts |
| `TIP` | `tip` | Callouts |
| `SIDEBAR` | `sidebar` | Callouts |
| `CALLOUT` | `callout` | Callouts |
| `METADATA` | `metadata` | Meta |
| `CITATION` | `citation` | Meta |
| `FOOTNOTE` | `footnote` | Meta |

## Rust SDK Roles

The Rust SDK provides additional granularity through subcategories. See [Metadata](./metadata.md#semantic-roles) for the complete Rust API.

Additional Rust-only role categories include:
- Introduction elements: `IntroHook`, `IntroContext`, `IntroThesis`
- Body elements: `BodyArgument`, `BodyEvidence`, `BodyExample`, `BodyCounterargument`, `BodyTransition`
- Conclusion elements: `ConclusionSummary`, `ConclusionImplication`, `ConclusionCallToAction`
- Technical elements: `Definition`, `Theorem`, `Proof`, `Algorithm`

## Best Practices

### 1. Use Roles Consistently

```python
# Good - clear semantic structure
doc.add_block(root, "Title", role=SemanticRole.TITLE)
doc.add_block(root, "Introduction", role=SemanticRole.INTRO)
doc.add_block(root, "Main content", role=SemanticRole.BODY)

# Less ideal - no semantic information
doc.add_block(root, "Title")  # What kind of block is this?
```

### 2. Choose Appropriate Roles

```python
# Use WARNING for important cautions
doc.add_block(root, "Data loss may occur!", role=SemanticRole.WARNING)

# Use TIP for helpful hints
doc.add_block(root, "Try using Ctrl+S", role=SemanticRole.TIP)

# Use NOTE for general information
doc.add_block(root, "See also: Related topic", role=SemanticRole.NOTE)
```

### 3. Leverage Roles for LLM Context

```python
from ucp import IdMapper, PromptBuilder

# Build context-aware prompts
doc = parse(markdown)
mapper = IdMapper(doc)

# IdMapper.describe() uses normalized format with structure and blocks
print(mapper.describe(doc))
# Output shows:
# Document structure:
# 1: 2 3
# 2:
# 3:
#
# Blocks:
# 1 type=text content=""
# 2 type=text content="Introduction"
# 3 type=text content="Content..."
```

## See Also

- [Metadata](./metadata.md) - Full metadata documentation
- [Blocks](./blocks.md) - Block structure
- [LLM Utilities](../ucp-llm/README.md) - Using roles with LLMs
