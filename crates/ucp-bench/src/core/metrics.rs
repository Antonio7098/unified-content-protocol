//! Core benchmark metrics and results.
//!
//! Tracks performance metrics (timing, memory, throughput) and validation results.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::inputs::InputCategory;

/// Validation result for structural accuracy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub checks: Vec<ValidationCheck>,
    pub error_message: Option<String>,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            valid: true,
            checks: vec![],
            error_message: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            checks: vec![],
            error_message: Some(message.into()),
        }
    }

    pub fn with_checks(mut self, checks: Vec<ValidationCheck>) -> Self {
        self.valid = checks.iter().all(|c| c.passed);
        self.checks = checks;
        self
    }
}

/// A single validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl ValidationCheck {
    pub fn pass(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            expected: None,
            actual: None,
        }
    }

    pub fn fail(
        name: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            passed: false,
            expected: Some(expected.into()),
            actual: Some(actual.into()),
        }
    }
}

/// Performance metrics for a single benchmark execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub duration_ns: u64,
    pub duration_ms: f64,
    pub memory_bytes: Option<u64>,
    pub throughput_ops_sec: Option<f64>,
    pub iterations: u32,
}

impl PerformanceMetrics {
    pub fn from_duration(duration: Duration, iterations: u32) -> Self {
        let nanos = duration.as_nanos() as u64;
        let ms = duration.as_secs_f64() * 1000.0;
        let ops_sec = if ms > 0.0 {
            Some((iterations as f64 / ms) * 1000.0)
        } else {
            None
        };

        Self {
            duration_ns: nanos,
            duration_ms: ms,
            memory_bytes: None,
            throughput_ops_sec: ops_sec,
            iterations,
        }
    }

    pub fn with_memory(mut self, bytes: u64) -> Self {
        self.memory_bytes = Some(bytes);
        self
    }
}

/// Result of a single core benchmark test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTestResult {
    pub test_id: String,
    pub input_id: String,
    pub category: InputCategory,
    pub benchmark_type: String,
    pub success: bool,
    pub performance: PerformanceMetrics,
    pub validation: ValidationResult,
    pub input_preview: String,
    pub output_preview: String,
    pub structure_preview: Option<String>,
    pub error_message: Option<String>,
    pub executed_at: DateTime<Utc>,
}

impl CoreTestResult {
    pub fn new(
        test_id: impl Into<String>,
        input_id: impl Into<String>,
        category: InputCategory,
        benchmark_type: impl Into<String>,
    ) -> Self {
        Self {
            test_id: test_id.into(),
            input_id: input_id.into(),
            category,
            benchmark_type: benchmark_type.into(),
            success: false,
            performance: PerformanceMetrics {
                duration_ns: 0,
                duration_ms: 0.0,
                memory_bytes: None,
                throughput_ops_sec: None,
                iterations: 1,
            },
            validation: ValidationResult::success(),
            input_preview: String::new(),
            output_preview: String::new(),
            structure_preview: None,
            error_message: None,
            executed_at: Utc::now(),
        }
    }

    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn with_performance(mut self, perf: PerformanceMetrics) -> Self {
        self.performance = perf;
        self
    }

    pub fn with_validation(mut self, validation: ValidationResult) -> Self {
        self.success = validation.valid;
        self.validation = validation;
        self
    }

    pub fn with_previews(
        mut self,
        input: impl Into<String>,
        output: impl Into<String>,
        structure: Option<String>,
    ) -> Self {
        self.input_preview = input.into();
        self.output_preview = output.into();
        self.structure_preview = structure;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.success = false;
        self.error_message = Some(error.into());
        self
    }
}

/// Aggregate metrics for a benchmark run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoreBenchmarkMetrics {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub success_rate: f32,
    pub total_duration_ms: f64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: f64,
    pub max_duration_ms: f64,
    pub p50_duration_ms: f64,
    pub p95_duration_ms: f64,
    pub p99_duration_ms: f64,
    pub total_throughput_ops_sec: f64,
    pub by_category: HashMap<String, CategoryMetrics>,
}

impl CoreBenchmarkMetrics {
    pub fn from_results(results: &[CoreTestResult]) -> Self {
        if results.is_empty() {
            return Self::default();
        }

        let total = results.len() as u32;
        let passed = results.iter().filter(|r| r.success).count() as u32;
        let failed = total - passed;

        let mut durations: Vec<f64> = results.iter().map(|r| r.performance.duration_ms).collect();
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let total_duration: f64 = durations.iter().sum();
        let avg = total_duration / durations.len() as f64;
        let min = durations.first().copied().unwrap_or(0.0);
        let max = durations.last().copied().unwrap_or(0.0);

        let p50 = percentile(&durations, 50.0);
        let p95 = percentile(&durations, 95.0);
        let p99 = percentile(&durations, 99.0);

        let total_throughput: f64 = results
            .iter()
            .filter_map(|r| r.performance.throughput_ops_sec)
            .sum();

        // Group by category
        let mut by_category: HashMap<String, Vec<&CoreTestResult>> = HashMap::new();
        for result in results {
            by_category
                .entry(format!("{:?}", result.category))
                .or_default()
                .push(result);
        }

        let category_metrics: HashMap<String, CategoryMetrics> = by_category
            .into_iter()
            .map(|(cat, cat_results)| {
                let cat_total = cat_results.len() as u32;
                let cat_passed = cat_results.iter().filter(|r| r.success).count() as u32;
                let cat_durations: Vec<f64> = cat_results
                    .iter()
                    .map(|r| r.performance.duration_ms)
                    .collect();
                let cat_avg = cat_durations.iter().sum::<f64>() / cat_durations.len() as f64;

                (
                    cat,
                    CategoryMetrics {
                        total: cat_total,
                        passed: cat_passed,
                        failed: cat_total - cat_passed,
                        success_rate: if cat_total > 0 {
                            cat_passed as f32 / cat_total as f32
                        } else {
                            0.0
                        },
                        avg_duration_ms: cat_avg,
                    },
                )
            })
            .collect();

        Self {
            total_tests: total,
            passed,
            failed,
            success_rate: if total > 0 {
                passed as f32 / total as f32
            } else {
                0.0
            },
            total_duration_ms: total_duration,
            avg_duration_ms: avg,
            min_duration_ms: min,
            max_duration_ms: max,
            p50_duration_ms: p50,
            p95_duration_ms: p95,
            p99_duration_ms: p99,
            total_throughput_ops_sec: total_throughput,
            by_category: category_metrics,
        }
    }
}

/// Per-category metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryMetrics {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub success_rate: f32,
    pub avg_duration_ms: f64,
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Complete core benchmark report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBenchmarkReport {
    pub report_id: String,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: CoreBenchmarkStatus,
    pub config: CoreBenchmarkReportConfig,
    pub metrics: CoreBenchmarkMetrics,
    pub results: Vec<CoreTestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBenchmarkReportConfig {
    pub categories: Vec<String>,
    pub iterations: u32,
    pub warmup_iterations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CoreBenchmarkStatus {
    Pending,
    Running { progress: f32, current_test: String },
    Completed,
    Failed { error: String },
}

impl CoreBenchmarkReport {
    pub fn new(name: impl Into<String>, config: CoreBenchmarkReportConfig) -> Self {
        Self {
            report_id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            started_at: Utc::now(),
            completed_at: None,
            status: CoreBenchmarkStatus::Pending,
            config,
            metrics: CoreBenchmarkMetrics::default(),
            results: vec![],
        }
    }

    pub fn mark_running(&mut self, current_test: String, progress: f32) {
        self.status = CoreBenchmarkStatus::Running {
            progress,
            current_test,
        };
    }

    pub fn mark_completed(&mut self) {
        self.completed_at = Some(Utc::now());
        self.status = CoreBenchmarkStatus::Completed;
        self.metrics = CoreBenchmarkMetrics::from_results(&self.results);
    }

    pub fn mark_failed(&mut self, error: String) {
        self.completed_at = Some(Utc::now());
        self.status = CoreBenchmarkStatus::Failed { error };
        self.metrics = CoreBenchmarkMetrics::from_results(&self.results);
    }

    pub fn add_result(&mut self, result: CoreTestResult) {
        self.results.push(result);
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("# Core Benchmark Report: {}\n\n", self.name));
        s.push_str(&format!("**ID:** {}\n", self.report_id));
        s.push_str(&format!(
            "**Date:** {}\n",
            self.started_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if let Some(completed) = self.completed_at {
            let duration = completed - self.started_at;
            s.push_str(&format!(
                "**Duration:** {}ms\n\n",
                duration.num_milliseconds()
            ));
        }

        s.push_str("## Summary\n\n");
        s.push_str("| Metric | Value |\n");
        s.push_str("|--------|-------|\n");
        s.push_str(&format!("| Total Tests | {} |\n", self.metrics.total_tests));
        s.push_str(&format!("| Passed | {} |\n", self.metrics.passed));
        s.push_str(&format!("| Failed | {} |\n", self.metrics.failed));
        s.push_str(&format!(
            "| Success Rate | {:.1}% |\n",
            self.metrics.success_rate * 100.0
        ));
        s.push_str(&format!(
            "| Avg Duration | {:.3}ms |\n",
            self.metrics.avg_duration_ms
        ));
        s.push_str(&format!(
            "| P50 Duration | {:.3}ms |\n",
            self.metrics.p50_duration_ms
        ));
        s.push_str(&format!(
            "| P95 Duration | {:.3}ms |\n",
            self.metrics.p95_duration_ms
        ));
        s.push_str(&format!(
            "| P99 Duration | {:.3}ms |\n\n",
            self.metrics.p99_duration_ms
        ));

        s.push_str("## By Category\n\n");
        for (cat, metrics) in &self.metrics.by_category {
            s.push_str(&format!("### {}\n\n", cat));
            s.push_str(&format!("- Tests: {}\n", metrics.total));
            s.push_str(&format!(
                "- Success Rate: {:.1}%\n",
                metrics.success_rate * 100.0
            ));
            s.push_str(&format!(
                "- Avg Duration: {:.3}ms\n\n",
                metrics.avg_duration_ms
            ));
        }

        s
    }
}
