//! Configuration for benchmark suites.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a benchmark suite run
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BenchmarkSuiteConfig {
    /// Provider/model pairs to test
    pub matrix: MatrixConfig,
    /// Maximum concurrent requests per provider
    pub concurrency: usize,
    /// Whether to execute commands (not just parse)
    pub execute_commands: bool,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    /// Whether to capture detailed debug info
    pub capture_debug_info: bool,
    /// Whether to capture document snapshots before/after
    pub capture_document_snapshots: bool,
    /// Document definition to use when executing tests
    pub document_id: Option<String>,
}

impl Default for BenchmarkSuiteConfig {
    fn default() -> Self {
        Self {
            matrix: MatrixConfig::default(),
            concurrency: 3,
            execute_commands: false,
            retry: RetryConfig::default(),
            timeout_ms: 30000,
            capture_debug_info: true,
            capture_document_snapshots: true,
            document_id: None,
        }
    }
}

impl BenchmarkSuiteConfig {
    pub fn with_matrix(mut self, matrix: MatrixConfig) -> Self {
        self.matrix = matrix;
        self
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn with_execution(mut self, execute: bool) -> Self {
        self.execute_commands = execute;
        self
    }

    pub fn with_debug_capture(mut self, capture: bool) -> Self {
        self.capture_debug_info = capture;
        self
    }

    pub fn with_document_snapshots(mut self, capture: bool) -> Self {
        self.capture_document_snapshots = capture;
        self
    }

    pub fn with_document_id(mut self, document_id: Option<String>) -> Self {
        self.document_id = document_id;
        self
    }
}

/// Matrix configuration for provider/model combinations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatrixConfig {
    /// List of provider/model pairs to test
    pub pairs: Vec<ProviderModelPair>,
}

impl MatrixConfig {
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    pub fn add_pair(mut self, provider: impl Into<String>, model: impl Into<String>) -> Self {
        self.pairs.push(ProviderModelPair {
            provider_id: provider.into(),
            model_id: model.into(),
            enabled: true,
            custom_config: None,
        });
        self
    }

    pub fn add_provider_models(
        mut self,
        provider: impl Into<String>,
        models: Vec<impl Into<String>>,
    ) -> Self {
        let provider = provider.into();
        for model in models {
            self.pairs.push(ProviderModelPair {
                provider_id: provider.clone(),
                model_id: model.into(),
                enabled: true,
                custom_config: None,
            });
        }
        self
    }

    /// Create a matrix from available providers
    pub fn from_available_providers() -> Self {
        Self::new()
            .add_provider_models(
                "groq",
                vec!["llama-3.1-8b-instant", "llama-3.3-70b-versatile"],
            )
            .add_provider_models("cerebras", vec!["llama3.1-8b", "llama-3.3-70b"])
            .add_provider_models(
                "openrouter",
                vec![
                    "meta-llama/llama-3.1-8b-instruct",
                    "anthropic/claude-3.5-sonnet",
                ],
            )
    }

    pub fn enabled_pairs(&self) -> impl Iterator<Item = &ProviderModelPair> {
        self.pairs.iter().filter(|p| p.enabled)
    }
}

/// A provider/model pair for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderModelPair {
    pub provider_id: String,
    pub model_id: String,
    pub enabled: bool,
    pub custom_config: Option<ProviderCustomConfig>,
}

impl ProviderModelPair {
    pub fn full_id(&self) -> String {
        format!("{}/{}", self.provider_id, self.model_id)
    }
}

/// Custom configuration for a specific provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCustomConfig {
    pub temperature: Option<f32>,
    pub custom_params: HashMap<String, serde_json::Value>,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RetryConfig {
    pub enabled: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub retry_on_rate_limit: bool,
    pub retry_on_timeout: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay_ms: 1000,
            retry_on_rate_limit: true,
            retry_on_timeout: true,
        }
    }
}
