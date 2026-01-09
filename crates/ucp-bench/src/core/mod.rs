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
pub use runner::{CoreBenchmarkConfig, CoreBenchmarkRunner};
pub use tests::{
    BenchmarkCategory, CodeBlockBenchmark, CoreBenchmark, DocumentBenchmark, JsonBenchmark,
    MarkdownBenchmark, NormalizationBenchmark, TableBenchmark, UclBenchmark,
};
