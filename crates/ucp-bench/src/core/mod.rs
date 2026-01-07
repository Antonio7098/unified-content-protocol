//! Core benchmarking system for non-LLM operations.
//!
//! This module provides a SOLID-compliant framework for benchmarking core UCP
//! operations like parsing, rendering, normalization, and document manipulation.

pub mod inputs;
pub mod metrics;
pub mod runner;
pub mod tests;

pub use inputs::{CoreTestInput, CoreTestInputRegistry, InputCategory, CORE_INPUTS};
pub use metrics::{CoreBenchmarkMetrics, CoreTestResult, ValidationResult};
pub use runner::{CoreBenchmarkRunner, CoreBenchmarkConfig};
pub use tests::{
    CoreBenchmark, BenchmarkCategory,
    MarkdownBenchmark, UclBenchmark, DocumentBenchmark, NormalizationBenchmark,
    JsonBenchmark, TableBenchmark, CodeBlockBenchmark,
};
