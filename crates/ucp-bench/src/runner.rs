//! Benchmark runner orchestration.

use crate::agent::BenchmarkAgent;
use crate::metrics::{BenchmarkMetrics, TestResult};
use crate::provider::LlmProvider;
use crate::report::BenchmarkReport;
use crate::test_cases::{generate_test_cases, TestCase};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{info, warn};

/// Configuration for benchmark runs
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub name: String,
    pub max_concurrent: usize,
    pub retry_on_rate_limit: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub filter_commands: Option<Vec<String>>,
    pub filter_test_ids: Option<Vec<String>>,
    /// If true, actually execute commands and validate document state changes
    pub execute_commands: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            name: "benchmark".into(),
            max_concurrent: 5,
            retry_on_rate_limit: true,
            max_retries: 3,
            retry_delay_ms: 1000,
            filter_commands: None,
            filter_test_ids: None,
            execute_commands: false,
        }
    }
}

impl BenchmarkConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn with_concurrency(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    pub fn filter_commands(mut self, commands: Vec<String>) -> Self {
        self.filter_commands = Some(commands);
        self
    }

    pub fn filter_tests(mut self, test_ids: Vec<String>) -> Self {
        self.filter_test_ids = Some(test_ids);
        self
    }

    pub fn with_execution(mut self, execute: bool) -> Self {
        self.execute_commands = execute;
        self
    }
}

/// Benchmark runner that orchestrates test execution
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    providers: Vec<Arc<dyn LlmProvider>>,
    test_cases: Vec<TestCase>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            providers: Vec::new(),
            test_cases: generate_test_cases(),
        }
    }

    pub fn add_provider<P: LlmProvider + 'static>(mut self, provider: P) -> Self {
        self.providers.push(Arc::new(provider));
        self
    }

    pub fn with_test_cases(mut self, cases: Vec<TestCase>) -> Self {
        self.test_cases = cases;
        self
    }

    /// Filter test cases based on config
    fn filtered_test_cases(&self) -> Vec<TestCase> {
        let mut cases = self.test_cases.clone();

        if let Some(ref commands) = self.config.filter_commands {
            cases.retain(|c| commands.contains(&c.command_type));
        }

        if let Some(ref ids) = self.config.filter_test_ids {
            cases.retain(|c| ids.contains(&c.id));
        }

        cases
    }

    /// Run benchmarks for all providers
    pub async fn run(&self) -> BenchmarkReport {
        let start_time = Utc::now();
        let mut all_results: Vec<TestResult> = Vec::new();
        let test_cases = self.filtered_test_cases();

        info!(
            "Starting benchmark '{}' with {} providers and {} test cases",
            self.config.name,
            self.providers.len(),
            test_cases.len()
        );

        for provider in &self.providers {
            info!(
                "Running tests for provider: {}/{}",
                provider.provider_id(),
                provider.model_id()
            );

            let results = self.run_provider(provider.clone(), &test_cases).await;
            all_results.extend(results);
        }

        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);

        let metrics = BenchmarkMetrics::from_results(&all_results);

        info!(
            "Benchmark complete: {}/{} passed ({:.1}%), cost: ${:.4}",
            metrics.passed,
            metrics.total_tests,
            metrics.success_rate() * 100.0,
            metrics.total_cost_usd
        );

        BenchmarkReport {
            benchmark_id: format!("bench_{}", start_time.format("%Y%m%d_%H%M%S")),
            name: self.config.name.clone(),
            start_time,
            end_time,
            duration_seconds: duration.num_seconds() as u64,
            models_tested: self.providers.iter().map(|p| p.full_id()).collect(),
            metrics,
            results: all_results,
        }
    }

    /// Run tests for a single provider
    async fn run_provider(
        &self,
        provider: Arc<dyn LlmProvider>,
        test_cases: &[TestCase],
    ) -> Vec<TestResult> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent));
        let mut handles = Vec::new();

        for test_case in test_cases {
            let provider = provider.clone();
            let test_case = test_case.clone();
            let semaphore = semaphore.clone();
            let config = self.config.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                run_single_test(provider, test_case, &config).await
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => warn!("Test task failed: {}", e),
            }
        }

        results
    }
}

async fn run_single_test(
    provider: Arc<dyn LlmProvider>,
    test_case: TestCase,
    config: &BenchmarkConfig,
) -> TestResult {
    let doc_def = crate::documents::DOCUMENTS.default().expect("No default document");
    let mut agent = BenchmarkAgent::new(provider, doc_def, config.execute_commands);
    let mut retries = 0;

    loop {
        let result = agent.run_test(&test_case).await;

        // Check if we should retry on rate limit
        if config.retry_on_rate_limit
            && result.error_category == Some(crate::metrics::ErrorCategory::RateLimitError)
            && retries < config.max_retries
        {
            retries += 1;
            warn!(
                "Rate limited on test {}, retry {}/{}",
                test_case.id, retries, config.max_retries
            );
            tokio::time::sleep(tokio::time::Duration::from_millis(
                config.retry_delay_ms * retries as u64,
            ))
            .await;
            continue;
        }

        return result;
    }
}

/// Quick benchmark runner for simple use cases
pub async fn run_quick_benchmark<P: LlmProvider + 'static>(
    provider: P,
    commands: Option<Vec<&str>>,
) -> BenchmarkReport {
    let mut config = BenchmarkConfig::new("quick-benchmark");

    if let Some(cmds) = commands {
        config = config.filter_commands(cmds.into_iter().map(String::from).collect());
    }

    BenchmarkRunner::new(config)
        .add_provider(provider)
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::MockProvider;

    #[tokio::test]
    async fn test_runner_with_mock() {
        let provider = MockProvider::new("test-model");
        let config = BenchmarkConfig::new("test")
            .with_concurrency(2)
            .filter_commands(vec!["EDIT".into()]);

        let runner = BenchmarkRunner::new(config).add_provider(provider);

        let report = runner.run().await;
        assert!(report.metrics.total_tests > 0);
    }

    #[test]
    fn test_config_builder() {
        let config = BenchmarkConfig::new("my-bench")
            .with_concurrency(10)
            .filter_commands(vec!["EDIT".into(), "APPEND".into()]);

        assert_eq!(config.name, "my-bench");
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.filter_commands, Some(vec!["EDIT".into(), "APPEND".into()]));
    }
}
