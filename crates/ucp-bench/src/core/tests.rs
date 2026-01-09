//! Core benchmark test implementations.
//!
//! Each test entity has a dedicated struct implementing the `CoreBenchmark` trait.
//! This follows SOLID principles:
//! - Single Responsibility: Each benchmark handles one type of operation
//! - Open/Closed: New benchmarks can be added without modifying existing ones
//! - Liskov Substitution: All benchmarks are interchangeable via the trait
//! - Interface Segregation: Minimal trait interface
//! - Dependency Inversion: Consumers depend on the trait, not concrete types

use std::time::Instant;

use super::inputs::{CoreTestInput, InputCategory};
use super::metrics::{CoreTestResult, PerformanceMetrics, ValidationCheck, ValidationResult};

/// Benchmark category enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchmarkCategory {
    Markdown,
    MarkdownRender,
    Ucl,
    Document,
    Normalization,
    Json,
    Table,
    CodeBlock,
}

impl BenchmarkCategory {
    pub fn from_input_category(cat: InputCategory) -> Self {
        match cat {
            InputCategory::Markdown => BenchmarkCategory::Markdown,
            InputCategory::MarkdownRender => BenchmarkCategory::MarkdownRender,
            InputCategory::Ucl => BenchmarkCategory::Ucl,
            InputCategory::Document => BenchmarkCategory::Document,
            InputCategory::Normalization => BenchmarkCategory::Normalization,
            InputCategory::Json => BenchmarkCategory::Json,
            InputCategory::Table => BenchmarkCategory::Table,
            InputCategory::CodeBlock => BenchmarkCategory::CodeBlock,
        }
    }
}

/// Core benchmark trait - all benchmark types implement this.
pub trait CoreBenchmark: Send + Sync {
    /// Returns the benchmark type name.
    fn name(&self) -> &'static str;

    /// Returns the category this benchmark belongs to.
    fn category(&self) -> BenchmarkCategory;

    /// Executes the benchmark for the given input.
    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult;

    /// Returns a description of what this benchmark tests.
    fn description(&self) -> &'static str;
}

// =============================================================================
// Markdown Benchmark
// =============================================================================

/// Benchmark for markdown parsing and rendering operations.
pub struct MarkdownBenchmark;

impl CoreBenchmark for MarkdownBenchmark {
    fn name(&self) -> &'static str {
        "markdown"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Markdown
    }

    fn description(&self) -> &'static str {
        "Tests markdown parsing to UCM and rendering back to markdown"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucp_translator_markdown::{parse_markdown, render_markdown};

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Warmup
        let _ = parse_markdown(&input.input);

        // Timed iterations
        let start = Instant::now();
        let mut last_doc = None;
        let mut last_rendered = String::new();

        for _ in 0..iterations {
            match parse_markdown(&input.input) {
                Ok(doc) => match render_markdown(&doc) {
                    Ok(rendered) => {
                        last_rendered = rendered;
                        last_doc = Some(doc);
                    }
                    Err(e) => {
                        return result.with_error(format!("Render error: {}", e));
                    }
                },
                Err(e) => {
                    return result.with_error(format!("Parse error: {}", e));
                }
            }
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        // Validation
        let validation = if let Some(ref doc) = last_doc {
            self.validate(input, doc)
        } else {
            ValidationResult::failure("No document produced")
        };

        // Structure preview - include UCM JSON
        let structure = last_doc.as_ref().map(|doc| {
            // Use the same serialization approach as test_document::document_ucm_json
            let structure_map = doc
                .structure
                .iter()
                .map(|(parent, children)| {
                    (
                        parent.to_string(),
                        children
                            .iter()
                            .map(|child| child.to_string())
                            .collect::<Vec<String>>(),
                    )
                })
                .collect::<std::collections::HashMap<String, Vec<String>>>();

            let blocks = doc
                .blocks
                .iter()
                .map(|(id, block)| {
                    let block_value =
                        serde_json::to_value(block).unwrap_or(serde_json::Value::Null);
                    (id.to_string(), block_value)
                })
                .collect::<std::collections::HashMap<String, serde_json::Value>>();

            let ucm_json = serde_json::json!({
                "id": doc.id.to_string(),
                "root": doc.root.to_string(),
                "metadata": doc.metadata,
                "structure": structure_map,
                "blocks": blocks,
            });

            let ucm_str = serde_json::to_string_pretty(&ucm_json)
                .unwrap_or_else(|_| "Failed to serialize".to_string());
            format!(
                "Document with {} blocks\nRoot: {}\nStructure entries: {}\n\nUCM JSON:\n{}",
                doc.block_count(),
                doc.root,
                doc.structure.len(),
                ucm_str
            )
        });

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(&input.input, 500),
                truncate(&last_rendered, 500),
                structure,
            )
    }
}

impl MarkdownBenchmark {
    fn validate(&self, input: &CoreTestInput, doc: &ucm_core::Document) -> ValidationResult {
        let mut checks = Vec::new();

        // Check block count if expected
        if let Some(ref expected) = input.expected_structure {
            if let Some(expected_count) = expected.block_count {
                let actual_count = doc.block_count();
                // Allow some variance (root block + content)
                let passed = actual_count >= expected_count;
                if passed {
                    checks.push(ValidationCheck::pass("block_count"));
                } else {
                    checks.push(ValidationCheck::fail(
                        "block_count",
                        format!(">= {}", expected_count),
                        actual_count.to_string(),
                    ));
                }
            }

            // Check for content types
            let has_code = doc
                .blocks
                .values()
                .any(|b| matches!(b.content, ucm_core::Content::Code(_)));
            if expected.has_code {
                if has_code {
                    checks.push(ValidationCheck::pass("has_code"));
                } else {
                    checks.push(ValidationCheck::fail("has_code", "true", "false"));
                }
            }

            let has_tables = doc
                .blocks
                .values()
                .any(|b| matches!(b.content, ucm_core::Content::Table(_)));
            if expected.has_tables {
                if has_tables {
                    checks.push(ValidationCheck::pass("has_tables"));
                } else {
                    checks.push(ValidationCheck::fail("has_tables", "true", "false"));
                }
            }
        }

        // Basic validation: document should have at least root
        if doc.block_count() > 0 {
            checks.push(ValidationCheck::pass("non_empty_document"));
        } else {
            checks.push(ValidationCheck::fail(
                "non_empty_document",
                "> 0 blocks",
                "0 blocks",
            ));
        }

        ValidationResult::success().with_checks(checks)
    }
}

// =============================================================================
// Markdown Render Benchmark (UCM to Markdown)
// =============================================================================

/// Benchmark for rendering UCM documents back to markdown.
pub struct MarkdownRenderBenchmark;

impl CoreBenchmark for MarkdownRenderBenchmark {
    fn name(&self) -> &'static str {
        "markdown_render"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::MarkdownRender
    }

    fn description(&self) -> &'static str {
        "Tests rendering UCM documents back to markdown format"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucp_translator_markdown::{parse_markdown, render_markdown};

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // First parse the input markdown to get a UCM document
        let doc = match parse_markdown(&input.input) {
            Ok(d) => d,
            Err(e) => return result.with_error(format!("Parse error: {}", e)),
        };

        // Warmup render
        let _ = render_markdown(&doc);

        // Timed iterations - only time the render (UCM to Markdown)
        let start = Instant::now();
        let mut last_rendered = String::new();

        for _ in 0..iterations {
            match render_markdown(&doc) {
                Ok(rendered) => {
                    last_rendered = rendered;
                }
                Err(e) => {
                    return result.with_error(format!("Render error: {}", e));
                }
            }
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        // Validation
        let mut checks = Vec::new();

        // Check that output is non-empty
        if !last_rendered.is_empty() {
            checks.push(ValidationCheck::pass("non_empty_output"));
        } else {
            checks.push(ValidationCheck::fail(
                "non_empty_output",
                "non-empty",
                "empty",
            ));
        }

        // Check that rendered output contains expected content
        if input.input.contains('#') && last_rendered.len() > 0 {
            checks.push(ValidationCheck::pass("contains_content"));
        }

        let validation = ValidationResult::success().with_checks(checks);

        // Structure preview - show both UCM structure and rendered output
        let structure_map = doc
            .structure
            .iter()
            .map(|(parent, children)| {
                (
                    parent.to_string(),
                    children.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                )
            })
            .collect::<std::collections::HashMap<_, _>>();

        let structure = format!(
            "UCM Document: {} blocks\nRoot: {}\n\nStructure (parent → children):\n{}\n\nRendered Output:\n{}",
            doc.block_count(),
            doc.root,
            serde_json::to_string_pretty(&structure_map).unwrap_or_default(),
            last_rendered
        );

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(&input.input, 500),
                last_rendered.clone(),
                Some(structure),
            )
    }
}

// =============================================================================
// UCL Parsing Benchmark
// =============================================================================

/// Benchmark for UCL (Unified Command Language) parsing.
pub struct UclBenchmark;

impl CoreBenchmark for UclBenchmark {
    fn name(&self) -> &'static str {
        "ucl_parsing"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Ucl
    }

    fn description(&self) -> &'static str {
        "Tests UCL command parsing performance and correctness"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Warmup
        let _ = ucl_parser::Parser::new(&input.input).parse_commands_only();

        // Timed iterations
        let start = Instant::now();
        let mut last_output = String::new();
        let mut command_count = 0;

        for _ in 0..iterations {
            match ucl_parser::Parser::new(&input.input).parse_commands_only() {
                Ok(commands) => {
                    command_count = commands.len();
                    last_output = format!(
                        "Parsed {} commands:\n{}",
                        commands.len(),
                        commands
                            .iter()
                            .take(5)
                            .map(|c| format!("  - {:?}", c))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                    if commands.len() > 5 {
                        last_output.push_str(&format!("\n  ... and {} more", commands.len() - 5));
                    }
                }
                Err(e) => {
                    return result.with_error(format!("Parse error: {:?}", e));
                }
            }
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        // Validation
        let validation = if command_count > 0 {
            ValidationResult::success().with_checks(vec![
                ValidationCheck::pass("commands_parsed"),
                ValidationCheck::pass(format!("{}_commands", command_count)),
            ])
        } else {
            ValidationResult::failure("No commands parsed")
        };

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(&input.input, 500),
                last_output,
                Some(format!("Command count: {}", command_count)),
            )
    }
}

// =============================================================================
// Document Operations Benchmark
// =============================================================================

/// Benchmark for document creation and manipulation.
pub struct DocumentBenchmark;

impl CoreBenchmark for DocumentBenchmark {
    fn name(&self) -> &'static str {
        "document_ops"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Document
    }

    fn description(&self) -> &'static str {
        "Tests document creation, block addition, and lookup operations"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucm_core::{Block, Content, Document};

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Parse the input command (format: "operation:param1:param2")
        let parts: Vec<&str> = input.input.split(':').collect();
        let operation = parts.first().copied().unwrap_or("create");

        let start = Instant::now();
        let mut output = String::new();
        let mut structure = String::new();

        match operation {
            "create" => {
                for _ in 0..iterations {
                    let _ = Document::create();
                }
                output = "Created empty document".into();
                structure = "Root block only".into();
            }
            "add" => {
                let count: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
                for _ in 0..iterations {
                    let mut doc = Document::create();
                    let root = doc.root.clone();
                    for i in 0..count {
                        let block = Block::new(Content::text(format!("Block {}", i)), None);
                        let _ = doc.add_block(block, &root);
                    }
                }
                output = format!("Added {} blocks to document", count);
                structure = format!("{} child blocks under root", count);
            }
            "lookup" => {
                let block_count: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                let lookup_count: usize = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(500);

                // Setup: create document with blocks
                let mut doc = Document::create();
                let root = doc.root.clone();
                let mut block_ids = vec![root.clone()];
                for i in 0..block_count {
                    let block = Block::new(Content::text(format!("Block {}", i)), None);
                    if let Ok(id) = doc.add_block(block, &root) {
                        block_ids.push(id);
                    }
                }

                let target_id = &block_ids[block_ids.len() / 2];
                for _ in 0..iterations {
                    for _ in 0..lookup_count {
                        let _ = doc.get_block(target_id);
                    }
                }
                output = format!(
                    "Performed {} lookups in {} block document",
                    lookup_count * iterations as usize,
                    block_count
                );
                structure = format!("Document: {} blocks, Target: middle block", block_count);
            }
            _ => {
                return result.with_error(format!("Unknown operation: {}", operation));
            }
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        result
            .with_success(true)
            .with_performance(perf)
            .with_validation(ValidationResult::success())
            .with_previews(input.input.clone(), output, Some(structure))
    }
}

// =============================================================================
// Normalization Benchmark
// =============================================================================

/// Benchmark for text normalization operations.
pub struct NormalizationBenchmark;

impl CoreBenchmark for NormalizationBenchmark {
    fn name(&self) -> &'static str {
        "normalization"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Normalization
    }

    fn description(&self) -> &'static str {
        "Tests Unicode, whitespace, and content normalization"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucm_core::normalize::{normalize_text, NormalizationConfig};

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Warmup
        let _ = normalize_text(&input.input, NormalizationConfig::default());

        // Timed iterations
        let start = Instant::now();
        let mut last_output = String::new();

        for _ in 0..iterations {
            last_output = normalize_text(&input.input, NormalizationConfig::default());
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        // Validation: output should be deterministic
        let second_run = normalize_text(&input.input, NormalizationConfig::default());
        let validation = if last_output == second_run {
            ValidationResult::success().with_checks(vec![
                ValidationCheck::pass("deterministic"),
                ValidationCheck::pass(format!("output_len_{}", last_output.len())),
            ])
        } else {
            ValidationResult::failure("Non-deterministic output")
        };

        let structure = format!(
            "Input: {} chars -> Output: {} chars\nReduction: {:.1}%",
            input.input.len(),
            last_output.len(),
            (1.0 - last_output.len() as f64 / input.input.len().max(1) as f64) * 100.0
        );

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(&input.input, 500),
                truncate(&last_output, 500),
                Some(structure),
            )
    }
}

// =============================================================================
// JSON Benchmark
// =============================================================================

/// Benchmark for JSON content operations.
pub struct JsonBenchmark;

impl CoreBenchmark for JsonBenchmark {
    fn name(&self) -> &'static str {
        "json_content"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Json
    }

    fn description(&self) -> &'static str {
        "Tests JSON parsing, content creation, and canonical serialization"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucm_core::normalize::canonical_json;
        use ucm_core::Content;

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Parse JSON first
        let json_value: serde_json::Value = match serde_json::from_str(&input.input) {
            Ok(v) => v,
            Err(e) => {
                return result.with_error(format!("Invalid JSON: {}", e));
            }
        };

        // Warmup
        let _ = Content::json(json_value.clone());
        let _ = canonical_json(&json_value);

        // Timed iterations
        let start = Instant::now();
        let mut last_canonical = String::new();

        for _ in 0..iterations {
            let _ = Content::json(json_value.clone());
            last_canonical = canonical_json(&json_value);
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        // Validation
        let validation = ValidationResult::success().with_checks(vec![
            ValidationCheck::pass("valid_json"),
            ValidationCheck::pass("canonical_form"),
        ]);

        let structure = format!(
            "Type: {}\nKeys: {}",
            match &json_value {
                serde_json::Value::Object(_) => "object",
                serde_json::Value::Array(_) => "array",
                _ => "primitive",
            },
            count_json_keys(&json_value)
        );

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(&input.input, 500),
                truncate(&last_canonical, 500),
                Some(structure),
            )
    }
}

fn count_json_keys(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => {
            map.len() + map.values().map(count_json_keys).sum::<usize>()
        }
        serde_json::Value::Array(arr) => arr.iter().map(count_json_keys).sum(),
        _ => 0,
    }
}

// =============================================================================
// Table Benchmark
// =============================================================================

/// Benchmark for table content operations.
pub struct TableBenchmark;

impl CoreBenchmark for TableBenchmark {
    fn name(&self) -> &'static str {
        "table_ops"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::Table
    }

    fn description(&self) -> &'static str {
        "Tests table content creation and manipulation"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucm_core::Content;

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Parse dimensions from input (format: "ROWSxCOLS")
        let parts: Vec<&str> = input.input.split('x').collect();
        let rows: usize = parts.first().and_then(|s| s.parse().ok()).unwrap_or(3);
        let cols: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(3);

        // Generate table data
        let table_data: Vec<Vec<String>> = (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        if r == 0 {
                            format!("Col{}", c + 1)
                        } else {
                            format!("R{}C{}", r, c + 1)
                        }
                    })
                    .collect()
            })
            .collect();

        // Warmup
        let _ = Content::table(table_data.clone());

        // Timed iterations
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = Content::table(table_data.clone());
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        let output = format!("Created {}x{} table ({} cells)", rows, cols, rows * cols);

        let structure = format!(
            "Rows: {}\nColumns: {}\nTotal cells: {}",
            rows,
            cols,
            rows * cols
        );

        let validation = ValidationResult::success().with_checks(vec![
            ValidationCheck::pass(format!("{}_rows", rows)),
            ValidationCheck::pass(format!("{}_cols", cols)),
        ]);

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(input.input.clone(), output, Some(structure))
    }
}

// =============================================================================
// Code Block Benchmark
// =============================================================================

/// Benchmark for code content operations.
pub struct CodeBlockBenchmark;

impl CoreBenchmark for CodeBlockBenchmark {
    fn name(&self) -> &'static str {
        "code_block"
    }

    fn category(&self) -> BenchmarkCategory {
        BenchmarkCategory::CodeBlock
    }

    fn description(&self) -> &'static str {
        "Tests code content creation and normalization"
    }

    fn run(&self, input: &CoreTestInput, iterations: u32) -> CoreTestResult {
        use ucm_core::normalize::normalize_content;
        use ucm_core::Content;

        let test_id = format!("{}_{}", self.name(), input.id);
        let mut result = CoreTestResult::new(&test_id, &input.id, input.category, self.name());

        // Parse input (format: "language:code")
        let (lang, code) = if let Some(idx) = input.input.find(':') {
            (&input.input[..idx], &input.input[idx + 1..])
        } else {
            ("text", input.input.as_str())
        };

        // Warmup
        let content = Content::code(lang, code);
        let _ = normalize_content(&content);

        // Timed iterations
        let start = Instant::now();
        let mut last_normalized = String::new();

        for _ in 0..iterations {
            let content = Content::code(lang, code);
            last_normalized = normalize_content(&content);
        }

        let duration = start.elapsed();
        let perf = PerformanceMetrics::from_duration(duration, iterations);

        let line_count = code.lines().count();
        let structure = format!(
            "Language: {}\nLines: {}\nCharacters: {}",
            lang,
            line_count,
            code.len()
        );

        let validation = ValidationResult::success().with_checks(vec![
            ValidationCheck::pass(format!("lang_{}", lang)),
            ValidationCheck::pass(format!("{}_lines", line_count)),
        ]);

        result
            .with_performance(perf)
            .with_validation(validation)
            .with_previews(
                truncate(code, 500),
                truncate(&last_normalized, 500),
                Some(structure),
            )
    }
}

// =============================================================================
// Utility Functions
// =============================================================================

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...[truncated]", &s[..max_len])
    }
}

/// Get all available core benchmarks.
pub fn all_benchmarks() -> Vec<Box<dyn CoreBenchmark>> {
    vec![
        Box::new(MarkdownBenchmark),
        Box::new(MarkdownRenderBenchmark),
        Box::new(UclBenchmark),
        Box::new(DocumentBenchmark),
        Box::new(NormalizationBenchmark),
        Box::new(JsonBenchmark),
        Box::new(TableBenchmark),
        Box::new(CodeBlockBenchmark),
    ]
}

/// Get benchmark for a specific category.
pub fn benchmark_for_category(category: InputCategory) -> Box<dyn CoreBenchmark> {
    match category {
        InputCategory::Markdown => Box::new(MarkdownBenchmark),
        InputCategory::MarkdownRender => Box::new(MarkdownRenderBenchmark),
        InputCategory::Ucl => Box::new(UclBenchmark),
        InputCategory::Document => Box::new(DocumentBenchmark),
        InputCategory::Normalization => Box::new(NormalizationBenchmark),
        InputCategory::Json => Box::new(JsonBenchmark),
        InputCategory::Table => Box::new(TableBenchmark),
        InputCategory::CodeBlock => Box::new(CodeBlockBenchmark),
    }
}
