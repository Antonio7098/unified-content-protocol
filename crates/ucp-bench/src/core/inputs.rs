//! Core test input registry.
//!
//! Defines sample inputs for each benchmark category with varying complexity levels.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Input category for core benchmarks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputCategory {
    Markdown,
    MarkdownRender,
    Ucl,
    Document,
    Normalization,
    Json,
    Table,
    CodeBlock,
}

impl InputCategory {
    pub fn all() -> &'static [InputCategory] {
        &[
            InputCategory::Markdown,
            InputCategory::MarkdownRender,
            InputCategory::Ucl,
            InputCategory::Document,
            InputCategory::Normalization,
            InputCategory::Json,
            InputCategory::Table,
            InputCategory::CodeBlock,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            InputCategory::Markdown => "Markdown",
            InputCategory::MarkdownRender => "UCM to Markdown",
            InputCategory::Ucl => "UCL Parsing",
            InputCategory::Document => "Document Operations",
            InputCategory::Normalization => "Normalization",
            InputCategory::Json => "JSON Content",
            InputCategory::Table => "Table Operations",
            InputCategory::CodeBlock => "Code Block",
        }
    }
}

/// Complexity level for test inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexityLevel {
    Simple,
    Medium,
    Complex,
    Stress,
}

/// A single test input for core benchmarks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTestInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: InputCategory,
    pub complexity: ComplexityLevel,
    pub input: String,
    pub expected_structure: Option<ExpectedStructure>,
}

/// Expected structure for validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedStructure {
    pub block_count: Option<usize>,
    pub content_types: Vec<String>,
    pub has_headings: bool,
    pub has_code: bool,
    pub has_tables: bool,
    pub has_lists: bool,
}

/// Registry of all core test inputs.
#[derive(Clone)]
pub struct CoreTestInputRegistry {
    inputs: HashMap<String, CoreTestInput>,
    by_category: HashMap<InputCategory, Vec<String>>,
}

pub static CORE_INPUTS: Lazy<CoreTestInputRegistry> = Lazy::new(CoreTestInputRegistry::new);

impl CoreTestInputRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            inputs: HashMap::new(),
            by_category: HashMap::new(),
        };

        for input in Self::builtin_inputs() {
            let category = input.category;
            let id = input.id.clone();
            registry.inputs.insert(id.clone(), input);
            registry.by_category.entry(category).or_default().push(id);
        }

        registry
    }

    pub fn get(&self, id: &str) -> Option<&CoreTestInput> {
        self.inputs.get(id)
    }

    pub fn list(&self) -> Vec<&CoreTestInput> {
        self.inputs.values().collect()
    }

    pub fn by_category(&self, category: InputCategory) -> Vec<&CoreTestInput> {
        self.by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.inputs.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn categories(&self) -> Vec<InputCategory> {
        InputCategory::all().to_vec()
    }

    fn builtin_inputs() -> Vec<CoreTestInput> {
        let mut inputs = Vec::new();

        // Markdown inputs
        inputs.extend(Self::markdown_inputs());
        inputs.extend(Self::markdown_render_inputs());
        inputs.extend(Self::ucl_inputs());
        inputs.extend(Self::document_inputs());
        inputs.extend(Self::normalization_inputs());
        inputs.extend(Self::json_inputs());
        inputs.extend(Self::table_inputs());
        inputs.extend(Self::code_block_inputs());

        inputs
    }
    
    fn markdown_render_inputs() -> Vec<CoreTestInput> {
        // These use the same markdown content but test rendering UCM back to markdown
        vec![
            CoreTestInput {
                id: "render_simple".into(),
                name: "Render Simple Document".into(),
                description: "Render a simple UCM document to markdown".into(),
                category: InputCategory::MarkdownRender,
                complexity: ComplexityLevel::Simple,
                input: "# Hello World\n\nThis is a paragraph.\n".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "render_hierarchical".into(),
                name: "Render Hierarchical Document".into(),
                description: "Render a hierarchical UCM document with nested headings".into(),
                category: InputCategory::MarkdownRender,
                complexity: ComplexityLevel::Medium,
                input: r#"# Title

## Section 1

Content for section 1.

### Subsection 1.1

More detailed content.

## Section 2

Another section.
"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "render_with_code".into(),
                name: "Render Document with Code".into(),
                description: "Render UCM document containing code blocks".into(),
                category: InputCategory::MarkdownRender,
                complexity: ComplexityLevel::Medium,
                input: r#"# Code Example

```python
def hello():
    print("Hello")
```

Some text after code.
"#.into(),
                expected_structure: None,
            },
        ]
    }

    fn markdown_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "md_simple_heading".into(),
                name: "Simple Heading".into(),
                description: "Basic markdown with a single heading".into(),
                category: InputCategory::Markdown,
                complexity: ComplexityLevel::Simple,
                input: "# Hello World\n\nThis is a paragraph.\n".into(),
                expected_structure: Some(ExpectedStructure {
                    block_count: Some(2),
                    content_types: vec!["text".into()],
                    has_headings: true,
                    has_code: false,
                    has_tables: false,
                    has_lists: false,
                }),
            },
            CoreTestInput {
                id: "md_nested_headings".into(),
                name: "Nested Headings".into(),
                description: "Markdown with multiple heading levels".into(),
                category: InputCategory::Markdown,
                complexity: ComplexityLevel::Medium,
                input: r#"# Title

## Section 1

Content for section 1.

### Subsection 1.1

More detailed content.

## Section 2

Another section.
"#.into(),
                expected_structure: Some(ExpectedStructure {
                    block_count: Some(6),
                    content_types: vec!["text".into()],
                    has_headings: true,
                    has_code: false,
                    has_tables: false,
                    has_lists: false,
                }),
            },
            CoreTestInput {
                id: "md_code_blocks".into(),
                name: "Code Blocks".into(),
                description: "Markdown with fenced code blocks".into(),
                category: InputCategory::Markdown,
                complexity: ComplexityLevel::Medium,
                input: r#"# Code Example

Here's some Python code:

```python
def hello():
    print("Hello, world!")

if __name__ == "__main__":
    hello()
```

And some Rust:

```rust
fn main() {
    println!("Hello from Rust!");
}
```
"#.into(),
                expected_structure: Some(ExpectedStructure {
                    block_count: Some(5),
                    content_types: vec!["text".into(), "code".into()],
                    has_headings: true,
                    has_code: true,
                    has_tables: false,
                    has_lists: false,
                }),
            },
            CoreTestInput {
                id: "md_full_featured".into(),
                name: "Full Featured Document".into(),
                description: "Complex markdown with all features".into(),
                category: InputCategory::Markdown,
                complexity: ComplexityLevel::Complex,
                input: r#"# Machine Learning Tutorial

## Introduction

This tutorial covers the basics of machine learning.

> Machine learning is a subset of artificial intelligence.

## Prerequisites

- Python 3.8+
- NumPy
- Pandas
- Scikit-learn

## Getting Started

### Installation

```bash
pip install numpy pandas scikit-learn
```

### Your First Model

```python
from sklearn.linear_model import LinearRegression
import numpy as np

X = np.array([[1], [2], [3], [4]])
y = np.array([2, 4, 6, 8])

model = LinearRegression()
model.fit(X, y)
print(f"Prediction: {model.predict([[5]])}")
```

## Results

| Model | Accuracy | F1 Score |
|-------|----------|----------|
| Linear | 0.85 | 0.82 |
| Random Forest | 0.92 | 0.90 |
| XGBoost | 0.94 | 0.93 |

## Conclusion

Machine learning is powerful!
"#.into(),
                expected_structure: Some(ExpectedStructure {
                    block_count: Some(15),
                    content_types: vec!["text".into(), "code".into(), "table".into()],
                    has_headings: true,
                    has_code: true,
                    has_tables: true,
                    has_lists: true,
                }),
            },
            CoreTestInput {
                id: "md_stress_large".into(),
                name: "Large Document".into(),
                description: "Stress test with many blocks".into(),
                category: InputCategory::Markdown,
                complexity: ComplexityLevel::Stress,
                input: Self::generate_large_markdown(100),
                expected_structure: None,
            },
        ]
    }

    fn ucl_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "ucl_simple_edit".into(),
                name: "Simple Edit".into(),
                description: "Basic UCL edit command".into(),
                category: InputCategory::Ucl,
                complexity: ComplexityLevel::Simple,
                input: r#"EDIT blk_000000000001 SET text = "hello world""#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "ucl_append".into(),
                name: "Append Command".into(),
                description: "UCL append with content type".into(),
                category: InputCategory::Ucl,
                complexity: ComplexityLevel::Simple,
                input: r#"APPEND blk_000000000002 text :: "new paragraph content""#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "ucl_atomic_block".into(),
                name: "Atomic Block".into(),
                description: "Multiple commands in atomic block".into(),
                category: InputCategory::Ucl,
                complexity: ComplexityLevel::Medium,
                input: r#"ATOMIC {
    EDIT blk_000000000001 SET text = "updated title"
    APPEND blk_000000000002 text :: "new content"
    MOVE blk_000000000003 TO blk_000000000004
}"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "ucl_complex_operations".into(),
                name: "Complex Operations".into(),
                description: "Mix of various UCL operations".into(),
                category: InputCategory::Ucl,
                complexity: ComplexityLevel::Complex,
                input: r#"ATOMIC {
    EDIT blk_000000000001 SET text = "Chapter 1: Introduction"
    APPEND blk_000000000001 code :: "python" :: "def main():\n    pass"
    INSERT AFTER blk_000000000002 text :: "Additional context"
    LINK blk_000000000003 TO blk_000000000004 AS "reference"
    DELETE blk_000000000005 CASCADE
    MOVE blk_000000000006 TO blk_000000000007 AT 0
}

EDIT blk_000000000008 SET text = "Final notes""#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "ucl_stress_many".into(),
                name: "Many Commands".into(),
                description: "Stress test with 100 commands".into(),
                category: InputCategory::Ucl,
                complexity: ComplexityLevel::Stress,
                input: Self::generate_ucl_commands(100),
                expected_structure: None,
            },
        ]
    }

    fn document_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "doc_create".into(),
                name: "Create Document".into(),
                description: "Create empty document".into(),
                category: InputCategory::Document,
                complexity: ComplexityLevel::Simple,
                input: "create".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "doc_add_10".into(),
                name: "Add 10 Blocks".into(),
                description: "Add 10 blocks to document".into(),
                category: InputCategory::Document,
                complexity: ComplexityLevel::Simple,
                input: "add:10".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "doc_add_100".into(),
                name: "Add 100 Blocks".into(),
                description: "Add 100 blocks to document".into(),
                category: InputCategory::Document,
                complexity: ComplexityLevel::Medium,
                input: "add:100".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "doc_add_1000".into(),
                name: "Add 1000 Blocks".into(),
                description: "Add 1000 blocks to document".into(),
                category: InputCategory::Document,
                complexity: ComplexityLevel::Complex,
                input: "add:1000".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "doc_lookup_stress".into(),
                name: "Lookup Stress".into(),
                description: "Many block lookups".into(),
                category: InputCategory::Document,
                complexity: ComplexityLevel::Stress,
                input: "lookup:1000:500".into(),
                expected_structure: None,
            },
        ]
    }

    fn normalization_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "norm_whitespace".into(),
                name: "Whitespace Normalization".into(),
                description: "Text with excessive whitespace".into(),
                category: InputCategory::Normalization,
                complexity: ComplexityLevel::Simple,
                input: "  Hello    world   \n\n  test  ".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "norm_unicode".into(),
                name: "Unicode Normalization".into(),
                description: "Text with Unicode characters".into(),
                category: InputCategory::Normalization,
                complexity: ComplexityLevel::Medium,
                input: "Héllo wörld! Ça va? 日本語テスト 한글".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "norm_line_endings".into(),
                name: "Line Ending Normalization".into(),
                description: "Mixed line endings".into(),
                category: InputCategory::Normalization,
                complexity: ComplexityLevel::Simple,
                input: "line1\r\nline2\rline3\nline4".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "norm_large_text".into(),
                name: "Large Text".into(),
                description: "1KB of text to normalize".into(),
                category: InputCategory::Normalization,
                complexity: ComplexityLevel::Complex,
                input: Self::generate_large_text(1024),
                expected_structure: None,
            },
            CoreTestInput {
                id: "norm_stress".into(),
                name: "Stress Test".into(),
                description: "10KB of mixed content".into(),
                category: InputCategory::Normalization,
                complexity: ComplexityLevel::Stress,
                input: Self::generate_large_text(10240),
                expected_structure: None,
            },
        ]
    }

    fn json_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "json_simple".into(),
                name: "Simple JSON".into(),
                description: "Basic JSON object".into(),
                category: InputCategory::Json,
                complexity: ComplexityLevel::Simple,
                input: r#"{"name": "test", "value": 42}"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "json_nested".into(),
                name: "Nested JSON".into(),
                description: "JSON with nested objects".into(),
                category: InputCategory::Json,
                complexity: ComplexityLevel::Medium,
                input: r#"{
    "user": {
        "name": "Alice",
        "age": 30,
        "address": {
            "city": "NYC",
            "zip": "10001"
        }
    },
    "active": true
}"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "json_array".into(),
                name: "JSON Array".into(),
                description: "JSON with arrays".into(),
                category: InputCategory::Json,
                complexity: ComplexityLevel::Medium,
                input: r#"{
    "items": [
        {"id": 1, "name": "Item 1"},
        {"id": 2, "name": "Item 2"},
        {"id": 3, "name": "Item 3"}
    ],
    "total": 3
}"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "json_complex".into(),
                name: "Complex JSON".into(),
                description: "Complex nested structure".into(),
                category: InputCategory::Json,
                complexity: ComplexityLevel::Complex,
                input: Self::generate_complex_json(),
                expected_structure: None,
            },
        ]
    }

    fn table_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "table_simple".into(),
                name: "Simple Table".into(),
                description: "3x3 table".into(),
                category: InputCategory::Table,
                complexity: ComplexityLevel::Simple,
                input: "3x3".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "table_medium".into(),
                name: "Medium Table".into(),
                description: "10x5 table".into(),
                category: InputCategory::Table,
                complexity: ComplexityLevel::Medium,
                input: "10x5".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "table_large".into(),
                name: "Large Table".into(),
                description: "100x10 table".into(),
                category: InputCategory::Table,
                complexity: ComplexityLevel::Complex,
                input: "100x10".into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "table_stress".into(),
                name: "Stress Table".into(),
                description: "1000x20 table".into(),
                category: InputCategory::Table,
                complexity: ComplexityLevel::Stress,
                input: "1000x20".into(),
                expected_structure: None,
            },
        ]
    }

    fn code_block_inputs() -> Vec<CoreTestInput> {
        vec![
            CoreTestInput {
                id: "code_python_simple".into(),
                name: "Simple Python".into(),
                description: "Basic Python function".into(),
                category: InputCategory::CodeBlock,
                complexity: ComplexityLevel::Simple,
                input: r#"python:def hello():
    print("Hello, world!")"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "code_rust_medium".into(),
                name: "Rust Code".into(),
                description: "Medium Rust code block".into(),
                category: InputCategory::CodeBlock,
                complexity: ComplexityLevel::Medium,
                input: r#"rust:use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}"#.into(),
                expected_structure: None,
            },
            CoreTestInput {
                id: "code_typescript".into(),
                name: "TypeScript Code".into(),
                description: "TypeScript with types".into(),
                category: InputCategory::CodeBlock,
                complexity: ComplexityLevel::Medium,
                input: r#"typescript:interface User {
    id: number;
    name: string;
    email: string;
}

async function fetchUser(id: number): Promise<User> {
    const response = await fetch(`/api/users/${id}`);
    return response.json();
}

const users: User[] = [];
users.push({ id: 1, name: "Alice", email: "alice@example.com" });"#.into(),
                expected_structure: None,
            },
        ]
    }

    fn generate_large_markdown(sections: usize) -> String {
        let mut md = String::new();
        md.push_str("# Large Document\n\n");

        for i in 0..sections {
            md.push_str(&format!("## Section {}\n\n", i + 1));
            md.push_str(&format!("This is paragraph {} with some content. ", i + 1));
            md.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. ");
            md.push_str("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\n");

            if i % 5 == 0 {
                md.push_str("```python\n");
                md.push_str(&format!("def function_{}():\n", i));
                md.push_str("    pass\n");
                md.push_str("```\n\n");
            }

            if i % 7 == 0 {
                md.push_str("- Item 1\n- Item 2\n- Item 3\n\n");
            }
        }

        md
    }

    fn generate_ucl_commands(count: usize) -> String {
        let mut cmds = String::from("ATOMIC {\n");
        for i in 0..count {
            cmds.push_str(&format!(
                r#"    EDIT blk_{:012x} SET text = "content {}""#,
                i, i
            ));
            cmds.push('\n');
        }
        cmds.push_str("}\n");
        cmds
    }

    fn generate_large_text(size: usize) -> String {
        let base = "This is a test sentence with some content. ";
        let mut text = String::with_capacity(size);
        while text.len() < size {
            text.push_str(base);
        }
        text.truncate(size);
        text
    }

    fn generate_complex_json() -> String {
        let mut json = String::from("{\n");
        json.push_str(r#"    "metadata": {"version": "1.0", "generated": "2025-01-07"},"#);
        json.push_str("\n    \"data\": [\n");
        
        for i in 0..20 {
            json.push_str(&format!(
                r#"        {{"id": {}, "name": "Item {}", "properties": {{"a": {}, "b": "val{}"}}}}"#,
                i, i, i * 10, i
            ));
            if i < 19 {
                json.push(',');
            }
            json.push('\n');
        }
        
        json.push_str("    ],\n");
        json.push_str(r#"    "summary": {"total": 20, "status": "complete"}"#);
        json.push_str("\n}");
        json
    }
}

impl Default for CoreTestInputRegistry {
    fn default() -> Self {
        Self::new()
    }
}
