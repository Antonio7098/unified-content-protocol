//! Core benchmark runner.
//!
//! Orchestrates execution of core benchmarks with configurable options.

use std::sync::Arc;
use tokio::sync::broadcast;

use super::inputs::{CoreTestInput, CoreTestInputRegistry, InputCategory, CORE_INPUTS};
use super::metrics::{
    CoreBenchmarkReport, CoreBenchmarkReportConfig, CoreBenchmarkStatus, CoreTestResult,
};
use super::tests::{all_benchmarks, benchmark_for_category, CoreBenchmark};

/// Configuration for core benchmark runs.
#[derive(Debug, Clone)]
pub struct CoreBenchmarkConfig {
    /// Categories to benchmark.
    pub categories: Vec<InputCategory>,
    /// Number of iterations per test.
    pub iterations: u32,
    /// Number of warmup iterations.
    pub warmup_iterations: u32,
    /// Optional specific input IDs to run (if empty, runs all for selected categories).
    pub input_ids: Vec<String>,
}

impl Default for CoreBenchmarkConfig {
    fn default() -> Self {
        Self {
            categories: InputCategory::all().to_vec(),
            iterations: 100,
            warmup_iterations: 10,
            input_ids: vec![],
        }
    }
}

impl CoreBenchmarkConfig {
    pub fn with_categories(mut self, categories: Vec<InputCategory>) -> Self {
        self.categories = categories;
        self
    }

    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_inputs(mut self, input_ids: Vec<String>) -> Self {
        self.input_ids = input_ids;
        self
    }
}

/// Update message for real-time progress tracking.
#[derive(Debug, Clone)]
pub enum CoreBenchmarkUpdate {
    Started {
        report_id: String,
    },
    Progress {
        report_id: String,
        progress: f32,
        current_test: String,
    },
    TestCompleted {
        report_id: String,
        test_id: String,
        success: bool,
    },
    Completed {
        report_id: String,
    },
    Failed {
        report_id: String,
        error: String,
    },
}

/// Core benchmark runner.
pub struct CoreBenchmarkRunner {
    config: CoreBenchmarkConfig,
    input_registry: Arc<CoreTestInputRegistry>,
    updates_tx: Option<broadcast::Sender<CoreBenchmarkUpdate>>,
}

impl CoreBenchmarkRunner {
    pub fn new(config: CoreBenchmarkConfig) -> Self {
        Self {
            config,
            input_registry: Arc::new(CORE_INPUTS.clone()),
            updates_tx: None,
        }
    }

    pub fn with_updates(mut self, tx: broadcast::Sender<CoreBenchmarkUpdate>) -> Self {
        self.updates_tx = Some(tx);
        self
    }

    pub fn with_registry(mut self, registry: Arc<CoreTestInputRegistry>) -> Self {
        self.input_registry = registry;
        self
    }

    /// Run all configured benchmarks.
    pub async fn run(&self, name: impl Into<String>) -> CoreBenchmarkReport {
        let name = name.into();
        let report_config = CoreBenchmarkReportConfig {
            categories: self
                .config
                .categories
                .iter()
                .map(|c| format!("{:?}", c))
                .collect(),
            iterations: self.config.iterations,
            warmup_iterations: self.config.warmup_iterations,
        };

        let mut report = CoreBenchmarkReport::new(&name, report_config);

        self.broadcast(CoreBenchmarkUpdate::Started {
            report_id: report.report_id.clone(),
        });

        // Collect inputs to run
        let inputs = self.collect_inputs();
        let total = inputs.len();

        if total == 0 {
            report.mark_failed("No inputs to benchmark".into());
            self.broadcast(CoreBenchmarkUpdate::Failed {
                report_id: report.report_id.clone(),
                error: "No inputs to benchmark".into(),
            });
            return report;
        }

        // Run benchmarks
        for (idx, input) in inputs.iter().enumerate() {
            let progress = (idx as f32 + 0.5) / total as f32;
            report.mark_running(input.name.clone(), progress);

            self.broadcast(CoreBenchmarkUpdate::Progress {
                report_id: report.report_id.clone(),
                progress,
                current_test: input.name.clone(),
            });

            // Get the appropriate benchmark for this input's category
            let benchmark = benchmark_for_category(input.category);
            let result = benchmark.run(input, self.config.iterations);

            let success = result.success;
            let test_id = result.test_id.clone();

            report.add_result(result);

            self.broadcast(CoreBenchmarkUpdate::TestCompleted {
                report_id: report.report_id.clone(),
                test_id,
                success,
            });

            // Small yield to allow other tasks
            tokio::task::yield_now().await;
        }

        report.mark_completed();

        self.broadcast(CoreBenchmarkUpdate::Completed {
            report_id: report.report_id.clone(),
        });

        report
    }

    /// Run benchmarks synchronously (for CLI use).
    pub fn run_sync(&self, name: impl Into<String>) -> CoreBenchmarkReport {
        let name = name.into();
        let report_config = CoreBenchmarkReportConfig {
            categories: self
                .config
                .categories
                .iter()
                .map(|c| format!("{:?}", c))
                .collect(),
            iterations: self.config.iterations,
            warmup_iterations: self.config.warmup_iterations,
        };

        let mut report = CoreBenchmarkReport::new(&name, report_config);

        let inputs = self.collect_inputs();
        let total = inputs.len();

        if total == 0 {
            report.mark_failed("No inputs to benchmark".into());
            return report;
        }

        for (idx, input) in inputs.iter().enumerate() {
            let progress = (idx as f32 + 0.5) / total as f32;
            report.mark_running(input.name.clone(), progress);

            let benchmark = benchmark_for_category(input.category);
            let result = benchmark.run(input, self.config.iterations);
            report.add_result(result);
        }

        report.mark_completed();
        report
    }

    fn collect_inputs(&self) -> Vec<CoreTestInput> {
        if !self.config.input_ids.is_empty() {
            // Run specific inputs
            self.config
                .input_ids
                .iter()
                .filter_map(|id| self.input_registry.get(id).cloned())
                .collect()
        } else {
            // Run all inputs for selected categories
            self.config
                .categories
                .iter()
                .flat_map(|cat| self.input_registry.by_category(*cat))
                .cloned()
                .collect()
        }
    }

    fn broadcast(&self, update: CoreBenchmarkUpdate) {
        if let Some(ref tx) = self.updates_tx {
            let _ = tx.send(update);
        }
    }
}

/// List all available benchmarks with descriptions.
pub fn list_benchmarks() -> Vec<BenchmarkInfo> {
    all_benchmarks()
        .into_iter()
        .map(|b| BenchmarkInfo {
            name: b.name().to_string(),
            category: format!("{:?}", b.category()),
            description: b.description().to_string(),
        })
        .collect()
}

/// Information about an available benchmark.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkInfo {
    pub name: String,
    pub category: String,
    pub description: String,
}

/// List all available test inputs.
pub fn list_inputs() -> Vec<InputInfo> {
    CORE_INPUTS
        .list()
        .into_iter()
        .map(|i| InputInfo {
            id: i.id.clone(),
            name: i.name.clone(),
            description: i.description.clone(),
            category: format!("{:?}", i.category),
            complexity: format!("{:?}", i.complexity),
        })
        .collect()
}

/// Information about an available input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub complexity: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_markdown() {
        let config = CoreBenchmarkConfig::default()
            .with_categories(vec![InputCategory::Markdown])
            .with_iterations(10);

        let runner = CoreBenchmarkRunner::new(config);
        let report = runner.run("Test Markdown").await;

        assert!(report.metrics.total_tests > 0);
        assert!(matches!(report.status, CoreBenchmarkStatus::Completed));
    }

    #[test]
    fn test_list_benchmarks() {
        let benchmarks = list_benchmarks();
        assert!(!benchmarks.is_empty());
        assert!(benchmarks.iter().any(|b| b.name == "markdown"));
    }

    #[test]
    fn test_list_inputs() {
        let inputs = list_inputs();
        assert!(!inputs.is_empty());
    }
}
