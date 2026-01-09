//! Metrics collection for benchmarks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error categories for failed tests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    ParseError,
    ExecutionError,
    SemanticError,
    TimeoutError,
    RateLimitError,
    InvalidResponse,
    ProviderError,
}

/// Result of a single benchmark test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: String,
    pub command_type: String,
    pub model: String,
    pub provider: String,
    pub timestamp: DateTime<Utc>,

    // Execution metrics
    pub latency_ms: u64,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_usd: f64,

    // Evaluation
    pub parse_success: bool,
    pub execute_success: bool,
    pub semantic_score: f32,
    pub efficiency_score: f32,

    // Generated output
    pub generated_ucl: String,
    pub expected_pattern: Option<String>,

    // Error info
    pub error_message: Option<String>,
    pub error_category: Option<ErrorCategory>,

    // Debug info (for verbose output)
    pub debug_prompt: Option<String>,
    pub debug_raw_response: Option<String>,
}

impl TestResult {
    pub fn success(
        test_id: impl Into<String>,
        command_type: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
    ) -> Self {
        Self {
            test_id: test_id.into(),
            command_type: command_type.into(),
            model: model.into(),
            provider: provider.into(),
            timestamp: Utc::now(),
            latency_ms: 0,
            input_tokens: 0,
            output_tokens: 0,
            cost_usd: 0.0,
            parse_success: true,
            execute_success: true,
            semantic_score: 1.0,
            efficiency_score: 1.0,
            generated_ucl: String::new(),
            expected_pattern: None,
            error_message: None,
            error_category: None,
            debug_prompt: None,
            debug_raw_response: None,
        }
    }

    pub fn failure(
        test_id: impl Into<String>,
        command_type: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
        category: ErrorCategory,
        message: impl Into<String>,
    ) -> Self {
        Self {
            test_id: test_id.into(),
            command_type: command_type.into(),
            model: model.into(),
            provider: provider.into(),
            timestamp: Utc::now(),
            latency_ms: 0,
            input_tokens: 0,
            output_tokens: 0,
            cost_usd: 0.0,
            parse_success: false,
            execute_success: false,
            semantic_score: 0.0,
            efficiency_score: 0.0,
            generated_ucl: String::new(),
            expected_pattern: None,
            error_message: Some(message.into()),
            error_category: Some(category),
            debug_prompt: None,
            debug_raw_response: None,
        }
    }

    pub fn is_success(&self) -> bool {
        self.parse_success && self.execute_success && self.error_category.is_none()
    }
}

/// Aggregated metrics for a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub total_cost_usd: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_latency_ms: u64,

    // Latency percentiles
    pub latency_p50_ms: u64,
    pub latency_p95_ms: u64,
    pub latency_p99_ms: u64,

    // By command type
    pub by_command: HashMap<String, CommandMetrics>,

    // By model
    pub by_model: HashMap<String, ModelMetrics>,

    // Error breakdown
    pub errors_by_category: HashMap<String, u32>,
}

impl BenchmarkMetrics {
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed: 0,
            failed: 0,
            total_cost_usd: 0.0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_latency_ms: 0,
            latency_p50_ms: 0,
            latency_p95_ms: 0,
            latency_p99_ms: 0,
            by_command: HashMap::new(),
            by_model: HashMap::new(),
            errors_by_category: HashMap::new(),
        }
    }

    pub fn from_results(results: &[TestResult]) -> Self {
        let mut metrics = Self::new();
        let mut latencies: Vec<u64> = Vec::new();

        for result in results {
            metrics.total_tests += 1;
            if result.is_success() {
                metrics.passed += 1;
            } else {
                metrics.failed += 1;
            }

            metrics.total_cost_usd += result.cost_usd;
            metrics.total_input_tokens += result.input_tokens as u64;
            metrics.total_output_tokens += result.output_tokens as u64;
            metrics.total_latency_ms += result.latency_ms;
            latencies.push(result.latency_ms);

            // By command
            metrics
                .by_command
                .entry(result.command_type.clone())
                .or_insert_with(CommandMetrics::new)
                .add_result(result);

            // By model
            let model_key = format!("{}/{}", result.provider, result.model);
            metrics
                .by_model
                .entry(model_key)
                .or_insert_with(ModelMetrics::new)
                .add_result(result);

            // Error categories
            if let Some(ref cat) = result.error_category {
                let key = format!("{:?}", cat);
                *metrics.errors_by_category.entry(key).or_insert(0) += 1;
            }
        }

        // Calculate percentiles
        latencies.sort_unstable();
        if !latencies.is_empty() {
            metrics.latency_p50_ms = percentile(&latencies, 50);
            metrics.latency_p95_ms = percentile(&latencies, 95);
            metrics.latency_p99_ms = percentile(&latencies, 99);
        }

        metrics
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            self.passed as f64 / self.total_tests as f64
        }
    }

    pub fn avg_latency_ms(&self) -> u64 {
        if self.total_tests == 0 {
            0
        } else {
            self.total_latency_ms / self.total_tests as u64
        }
    }
}

impl Default for BenchmarkMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics for a specific command type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetrics {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub total_latency_ms: u64,
    pub total_cost_usd: f64,
    pub avg_semantic_score: f64,
}

impl CommandMetrics {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            total_latency_ms: 0,
            total_cost_usd: 0.0,
            avg_semantic_score: 0.0,
        }
    }

    pub fn add_result(&mut self, result: &TestResult) {
        self.total += 1;
        if result.is_success() {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        self.total_latency_ms += result.latency_ms;
        self.total_cost_usd += result.cost_usd;

        // Running average for semantic score
        let n = self.total as f64;
        self.avg_semantic_score =
            self.avg_semantic_score * (n - 1.0) / n + result.semantic_score as f64 / n;
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }

    pub fn avg_latency_ms(&self) -> u64 {
        if self.total == 0 {
            0
        } else {
            self.total_latency_ms / self.total as u64
        }
    }
}

impl Default for CommandMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics for a specific model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub total_latency_ms: u64,
    pub total_cost_usd: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub by_command: HashMap<String, CommandMetrics>,
}

impl ModelMetrics {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            total_latency_ms: 0,
            total_cost_usd: 0.0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            by_command: HashMap::new(),
        }
    }

    pub fn add_result(&mut self, result: &TestResult) {
        self.total += 1;
        if result.is_success() {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        self.total_latency_ms += result.latency_ms;
        self.total_cost_usd += result.cost_usd;
        self.total_input_tokens += result.input_tokens as u64;
        self.total_output_tokens += result.output_tokens as u64;

        self.by_command
            .entry(result.command_type.clone())
            .or_insert_with(CommandMetrics::new)
            .add_result(result);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }

    pub fn avg_latency_ms(&self) -> u64 {
        if self.total == 0 {
            0
        } else {
            self.total_latency_ms / self.total as u64
        }
    }
}

impl Default for ModelMetrics {
    fn default() -> Self {
        Self::new()
    }
}

fn percentile(sorted: &[u64], p: u32) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let idx = (sorted.len() as f64 * p as f64 / 100.0).ceil() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_from_results() {
        let results = vec![
            TestResult::success("t1", "EDIT", "gpt-4o", "openai"),
            TestResult::success("t2", "EDIT", "gpt-4o", "openai"),
            TestResult::failure(
                "t3",
                "APPEND",
                "gpt-4o",
                "openai",
                ErrorCategory::ParseError,
                "bad syntax",
            ),
        ];

        let metrics = BenchmarkMetrics::from_results(&results);
        assert_eq!(metrics.total_tests, 3);
        assert_eq!(metrics.passed, 2);
        assert_eq!(metrics.failed, 1);
    }

    #[test]
    fn test_success_rate() {
        let metrics = BenchmarkMetrics {
            total_tests: 10,
            passed: 8,
            failed: 2,
            ..Default::default()
        };
        assert!((metrics.success_rate() - 0.8).abs() < 0.001);
    }
}
