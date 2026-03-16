# CodeGraph Schema Documentation

The CodeGraph is a structured representation of source code that captures the relationships between files, symbols, and their dependencies. It is stored as a UCM Document with a specific profile marker (`codegraph.v1`).

## Profile Information

- **Profile Name**: `codegraph`
- **Profile Version**: `v1`
- **Marker**: `codegraph.v1`
- **Extractor Version**: `ucp-codegraph-extractor.v1`

---

## Node Classes

Nodes represent entities in the codebase. Each node has a `node_class` metadata key that identifies its type.

### Repository

Represents the root of a codebase.

| Property | Value |
|----------|-------|
| `logical_key` prefix | `repository:` |
| Required Metadata | `logical_key`, `coderef` |

**Example**:
```json
{
  "logical_key": "repository:/home/user/my-project",
  "node_class": "repository",
  "coderef": { "path": "/home/user/my-project" }
}
```

### Directory

Represents a directory in the filesystem hierarchy.

| Property | Value |
|----------|-------|
| `logical_key` prefix | `directory:` |
| Required Metadata | `logical_key`, `coderef` |

**Example**:
```json
{
  "logical_key": "directory:/home/user/my-project/src",
  "node_class": "directory",
  "coderef": { "path": "/home/user/my-project/src" }
}
```

### File

Represents a source file.

| Property | Value |
|----------|-------|
| `logical_key` prefix | `file:` |
| Required Metadata | `logical_key`, `coderef`, `language` |

**Metadata**:
- `language`: The programming language of the file (`rust`, `python`, `typescript`, `javascript`)

**Example**:
```json
{
  "logical_key": "file:/home/user/my-project/src/main.rs",
  "node_class": "file",
  "coderef": { "path": "/home/user/my-project/src/main.rs" },
  "language": "rust"
}
```

### Symbol

Represents a named code entity (function, class, method, constant, etc.).

| Property | Value |
|----------|-------|
| `logical_key` prefix | `symbol:` |
| Required Metadata | `logical_key`, `coderef`, `language`, `symbol_kind`, `name`, `exported` |

**Metadata**:
- `language`: The programming language (`rust`, `python`, `typescript`, `javascript`)
- `symbol_kind`: The kind of symbol (e.g., `function`, `class`, `method`, `module`, `constant`)
- `name`: The simple name of the symbol
- `exported`: Boolean indicating if the symbol is exported (`true`/`false`)

**Additional Optional Metadata**:
- `description`: Documentation comment text
- `signature`: Function/method signature
- `modifiers`: Additional modifiers like `async`, `static`, visibility

**Example**:
```json
{
  "logical_key": "symbol:crates/ucp-codegraph/src/model.rs::CodeGraphStats",
  "node_class": "symbol",
  "coderef": {
    "path": "crates/ucp-codegraph/src/model.rs",
    "start_line": 86,
    "end_line": 98
  },
  "language": "rust",
  "symbol_kind": "struct",
  "name": "CodeGraphStats",
  "exported": true
}
```

---

## Edge Types

Edges represent relationships between nodes. The codegraph uses both standard UCM edge types and custom edge types.

### Standard Edge Types (UCM)

These are built-in edge types from the UCM core:

| Edge Type | Description |
|-----------|-------------|
| `references` | General reference relationship (function calls, variable usage) |
| `derived_from` | Content derived from another source |
| `supersedes` | Replaces/obsoletes another node |
| `transformed_from` | Transformed from another source |
| `cited_by` | Cited by another node |
| `links_to` | Hyperlink to another resource |
| `supports` | Provides support/evidence for |
| `contradicts` | Contradicts another node |
| `elaborates` | Provides additional detail |
| `summarizes` | Summarizes another node |
| `parent_of` | Hierarchical parent relationship |
| `child_of` | Hierarchical child relationship |
| `sibling_of` | Same parent relationship |
| `previous_sibling` | Previous in sibling order |
| `next_sibling` | Next in sibling order |
| `version_of` | Different version of same entity |
| `alternative_of` | Alternative to another node |
| `translation_of` | Translation of another node |

### Custom Edge Types (CodeGraph-Specific)

These are custom edge types added by the codegraph extractor:

| Edge Type | Description | Source | Target |
|-----------|-------------|--------|--------|
| `exports` | Symbol is exported from a file | File | Symbol |
| `imports_symbol` | File imports a symbol | File | Symbol |
| `uses_symbol` | Symbol uses/calls another symbol | Symbol | Symbol |
| `contains` | Container contains child | File/Directory | File/Symbol |
| `defines` | File defines a symbol | File | Symbol |
| `implements` | Symbol implements another | Symbol | Symbol |
| `extends` | Symbol extends another (inheritance) | Symbol | Symbol |

---

## Graph Structure

The graph is stored as a UCM Document with hierarchical structure and edges.

### JSON Schema

```json
{
  "profile": "codegraph",
  "profile_version": "v1",
  "nodes": [
    {
      "logical_key": "string",
      "node_class": "repository|directory|file|symbol",
      "semantic_role": "string|null",
      "content_type": "string",
      "content": "string",
      "metadata": {
        "label": "string|null",
        "semantic_role": "string|null",
        "tags": [],
        "summary": "string|null",
        "custom": {
          "logical_key": "string",
          "node_class": "string",
          "coderef": {
            "path": "string",
            "start_line": "number",
            "end_line": "number",
            "start_col": "number|null",
            "end_col": "number|null"
          },
          "language": "string",
          "symbol_kind": "string",
          "name": "string",
          "exported": "boolean"
        }
      }
    }
  ],
  "structure": [
    {
      "parent": "block_id",
      "children": ["block_id", "block_id"]
    }
  ],
  "edges": [
    {
      "source": "block_id",
      "edge_type": "references|exports|imports_symbol|...",
      "target": "block_id",
      "metadata": {
        "confidence": "number|null",
        "description": "string|null",
        "custom": {}
      }
    }
  ],
  "document_metadata": {
    "title": "string|null",
    "description": "string|null",
    "authors": [],
    "language": "string|null",
    "custom": {}
  }
}
```

---

## Metadata Keys

### Required Metadata by Node Class

| Node Class | Required Keys |
|------------|---------------|
| `repository` | `logical_key`, `coderef` |
| `directory` | `logical_key`, `coderef` |
| `file` | `logical_key`, `coderef`, `language` |
| `symbol` | `logical_key`, `coderef`, `language`, `symbol_kind`, `name`, `exported` |

### Common Metadata Keys

| Key | Type | Description |
|-----|------|-------------|
| `logical_key` | string | Unique identifier with prefix |
| `node_class` | string | Type of node |
| `coderef` | object | Code reference information |
| `language` | string | Programming language |
| `symbol_kind` | string | Kind of symbol (function, class, etc.) |
| `name` | string | Display name |
| `exported` | boolean | Whether symbol is exported |
| `description` | string | Documentation |
| `signature` | string | Function signature |
| `modifiers` | object | Additional modifiers |

### Coderef Structure

```json
{
  "path": "relative/or/absolute/path/to/file.rs",
  "start_line": 10,
  "end_line": 25,
  "start_col": 0,
  "end_col": 15
}
```

---

## Supported Languages

| Language | Extensions | Symbol Kinds |
|----------|------------|--------------|
| Rust | `.rs` | function, struct, enum, trait, impl, module, const, macro |
| Python | `.py` | function, class, module, async_function |
| TypeScript | `.ts`, `.tsx` | function, class, interface, type_alias, method |
| JavaScript | `.js`, `.jsx` | function, class, variable_declaration |

---

## Stats

The codegraph build produces statistics:

```json
{
  "total_nodes": 150,
  "repository_nodes": 1,
  "directory_nodes": 12,
  "file_nodes": 45,
  "symbol_nodes": 92,
  "total_edges": 234,
  "reference_edges": 180,
  "export_edges": 54,
  "languages": {
    "rust": 50,
    "typescript": 30,
    "python": 12
  }
}
```

---

## Extraction Configuration

The extractor can be configured with:

```json
{
  "include_extensions": ["rs", "py", "ts", "tsx", "js", "jsx"],
  "exclude_dirs": [".git", "target", "node_modules", "dist", "build"],
  "continue_on_parse_error": true,
  "include_hidden": false,
  "max_file_bytes": 2097152,
  "emit_export_edges": true
}
```

---

## Validation

The codegraph is validated on build with these error codes:

| Code | Description |
|------|-------------|
| CG1017 | Invalid node_class |
| CG1018 | Missing required metadata key |
| CG1019 | logical_key must start with expected prefix |

---

## Related Files

- Schema definition: `crates/ucp-codegraph/src/model.rs`
- Canonicalization: `crates/ucp-codegraph/src/legacy/canonical.rs`
- Extraction: `crates/ucp-codegraph/src/legacy/extract.rs`
- Build: `crates/ucp-codegraph/src/legacy/build.rs`
- Validation: `crates/ucp-codegraph/src/legacy/validate.rs`
