//! Modular test suite system for benchmarking.
//! 
//! This module provides a flexible, extensible architecture for defining
//! and running benchmark test suites with support for:
//! - Modular test categories (EDIT, DELETE, APPEND, etc.)
//! - Provider/model matrix configurations
//! - Detailed result capture with full context
//! - Pluggable test case generators

pub mod category;
pub mod config;
pub mod result;
pub mod registry;

pub use category::{TestCategory, TestCategoryId};
pub use config::{BenchmarkSuiteConfig, ProviderModelPair, MatrixConfig};
pub use result::{DetailedTestResult, DocumentSnapshot, ExecutionContext};
pub use registry::TestRegistry;

use crate::provider::LlmProvider;
use crate::test_cases::TestCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Unique identifier for a test suite run
pub type SuiteRunId = String;

/// A benchmark suite that can be configured and executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    /// Unique identifier for this suite
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this suite tests
    pub description: String,
    /// Categories included in this suite
    pub categories: Vec<TestCategoryId>,
    /// Configuration for this suite
    pub config: BenchmarkSuiteConfig,
    /// When this suite was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl BenchmarkSuite {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: description.into(),
            categories: Vec::new(),
            config: BenchmarkSuiteConfig::default(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_categories(mut self, categories: Vec<TestCategoryId>) -> Self {
        self.categories = categories;
        self
    }

    pub fn with_config(mut self, config: BenchmarkSuiteConfig) -> Self {
        self.config = config;
        self
    }

    /// Add a single category
    pub fn add_category(&mut self, category: TestCategoryId) {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }
    }

    /// Remove a category
    pub fn remove_category(&mut self, category: &TestCategoryId) {
        self.categories.retain(|c| c != category);
    }
}

/// Result of running a benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteRunResult {
    /// Unique run identifier
    pub run_id: SuiteRunId,
    /// The suite that was run
    pub suite_id: String,
    /// When the run started
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// When the run completed
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Status of the run
    pub status: RunStatus,
    /// Results grouped by provider/model
    pub results_by_provider: HashMap<String, ProviderResults>,
    /// Overall statistics
    pub summary: SuiteSummary,
    /// Configuration used for this run
    pub config: BenchmarkSuiteConfig,
}

/// Status of a suite run
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Pending,
    Running { progress: f32, current_test: String },
    Completed,
    Failed { error: String },
    Cancelled,
}

/// Results for a specific provider/model combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResults {
    pub provider_id: String,
    pub model_id: String,
    pub results_by_category: HashMap<TestCategoryId, CategoryResults>,
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub total_cost_usd: f64,
    pub total_latency_ms: u64,
    pub avg_latency_ms: u64,
}

/// Results for a specific category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResults {
    pub category_id: TestCategoryId,
    pub tests: Vec<DetailedTestResult>,
    pub passed: u32,
    pub failed: u32,
    pub success_rate: f32,
    pub avg_latency_ms: u64,
    pub total_cost_usd: f64,
}

/// Summary statistics for the entire suite run
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuiteSummary {
    pub total_tests: u32,
    pub total_passed: u32,
    pub total_failed: u32,
    pub overall_success_rate: f32,
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub providers_tested: u32,
    pub categories_tested: u32,
}

impl SuiteRunResult {
    pub fn new(suite_id: String, config: BenchmarkSuiteConfig) -> Self {
        Self {
            run_id: uuid::Uuid::new_v4().to_string(),
            suite_id,
            started_at: chrono::Utc::now(),
            completed_at: None,
            status: RunStatus::Pending,
            results_by_provider: HashMap::new(),
            summary: SuiteSummary::default(),
            config,
        }
    }

    pub fn mark_running(&mut self, current_test: String, progress: f32) {
        self.status = RunStatus::Running { progress, current_test };
    }

    pub fn mark_completed(&mut self) {
        self.completed_at = Some(chrono::Utc::now());
        self.status = RunStatus::Completed;
        self.calculate_summary();
    }

    pub fn mark_failed(&mut self, error: String) {
        self.completed_at = Some(chrono::Utc::now());
        self.status = RunStatus::Failed { error };
    }

    fn calculate_summary(&mut self) {
        let mut summary = SuiteSummary::default();
        
        for provider_results in self.results_by_provider.values() {
            summary.total_tests += provider_results.total_tests;
            summary.total_passed += provider_results.passed;
            summary.total_failed += provider_results.failed;
            summary.total_cost_usd += provider_results.total_cost_usd;
            summary.providers_tested += 1;
        }

        if summary.total_tests > 0 {
            summary.overall_success_rate = summary.total_passed as f32 / summary.total_tests as f32;
        }

        if let Some(completed) = self.completed_at {
            summary.total_duration_ms = (completed - self.started_at).num_milliseconds() as u64;
        }

        summary.categories_tested = self.results_by_provider
            .values()
            .flat_map(|p| p.results_by_category.keys())
            .collect::<std::collections::HashSet<_>>()
            .len() as u32;

        self.summary = summary;
    }
}
