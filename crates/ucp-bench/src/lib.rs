//! UCP Benchmarking System
//!
//! Comprehensive benchmarking for both system performance and LLM evaluation.
//! 
//! ## Architecture
//! 
//! - **suite**: Modular test suite system with categories, configs, and detailed results
//! - **api**: REST API server for the benchmark UI
//! - **agent**: LLM agent that generates and executes UCL commands
//! - **provider**: LLM provider implementations (Groq, Cerebras, OpenRouter, etc.)
//! - **runner**: Benchmark orchestration and execution
//! - **core**: Core benchmarking for non-LLM operations (parsing, normalization, etc.)

pub mod agent;
pub mod api;
pub mod core;
pub mod documents;
pub mod id_mapper;
pub mod metrics;
pub mod provider;
pub mod report;
pub mod runner;
pub mod storage;
pub mod suite;
pub mod test_cases;
pub mod test_document;
pub mod tokenizer;

pub use agent::BenchmarkAgent;
pub use documents::{DocumentDefinition, DocumentRegistry};
pub use metrics::{BenchmarkMetrics, TestResult};
pub use provider::{
    CompletionRequest,
    CompletionResponse,
    LlmProvider,
    TokenPricing,
    GroqProvider,
    CerebrasProvider,
    GmiCloudProvider,
    OpenRouterProvider,
    MockProvider,
    fetch_openrouter_pricing,
};
pub use report::BenchmarkReport;
pub use storage::{
    default_storage,
    default_storage_root,
    default_core_benchmark_storage,
    BenchmarkStorage,
    FileBenchmarkStorage,
    StoredBenchmark,
    StoredBenchmarkMetadata,
    CoreBenchmarkStorage,
    CoreBenchmarkSummary,
};
pub use runner::{BenchmarkRunner, BenchmarkConfig};
pub use suite::{BenchmarkSuite, TestRegistry, TestCategory, TestCategoryId};
pub use test_cases::TestCase;
pub use test_document::create_test_document;
pub use tokenizer::count_tokens;

// Core benchmarking exports
pub use core::{
    CoreBenchmark, CoreBenchmarkConfig, CoreBenchmarkRunner,
    CoreBenchmarkMetrics, CoreTestResult, CoreTestInput,
    CoreTestInputRegistry, InputCategory, CORE_INPUTS,
    BenchmarkCategory,
};

// ID mapping for LLM prompts (token optimization)
pub use id_mapper::IdMapper;
