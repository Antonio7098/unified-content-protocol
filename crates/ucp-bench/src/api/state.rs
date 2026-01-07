//! Application state for the API server.

use crate::suite::{
    BenchmarkSuite, SuiteRunResult, TestRegistry, TestCategoryId,
    config::MatrixConfig,
};
use crate::core::metrics::CoreBenchmarkReport;
use std::collections::HashMap;
use tokio::sync::broadcast;

/// Application state shared across API handlers
pub struct AppState {
    /// Test registry with all categories and test cases
    pub registry: TestRegistry,
    /// Saved benchmark suites
    pub suites: HashMap<String, BenchmarkSuite>,
    /// Active and completed runs (LLM benchmarks)
    pub runs: HashMap<String, SuiteRunResult>,
    /// Channel for broadcasting run updates
    pub run_updates: broadcast::Sender<RunUpdate>,
    /// Available provider configurations
    pub available_providers: Vec<ProviderInfo>,
    /// Active and completed core benchmark runs
    pub core_runs: HashMap<String, CoreBenchmarkReport>,
    /// Channel for broadcasting core benchmark updates
    pub core_updates: broadcast::Sender<CoreBenchmarkUpdate>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        let (core_tx, _) = broadcast::channel(100);
        
        Self {
            registry: TestRegistry::new(),
            suites: HashMap::new(),
            runs: HashMap::new(),
            run_updates: tx,
            available_providers: Self::detect_available_providers(),
            core_runs: HashMap::new(),
            core_updates: core_tx,
        }
    }

    fn detect_available_providers() -> Vec<ProviderInfo> {
        let mut providers = Vec::new();

        // Check for Groq
        if std::env::var("GROQ_API_KEY").is_ok() {
            providers.push(ProviderInfo {
                id: "groq".into(),
                name: "Groq".into(),
                available: true,
                models: vec![
                    ModelInfo { id: "llama-3.1-8b-instant".into(), name: "Llama 3.1 8B Instant".into(), context_length: 131072 },
                    ModelInfo { id: "llama-3.3-70b-versatile".into(), name: "Llama 3.3 70B Versatile".into(), context_length: 131072 },
                ],
            });
        }

        // Check for Cerebras
        if std::env::var("CEREBRAS_API_KEY").is_ok() {
            providers.push(ProviderInfo {
                id: "cerebras".into(),
                name: "Cerebras".into(),
                available: true,
                models: vec![
                    ModelInfo { id: "llama3.1-8b".into(), name: "Llama 3.1 8B".into(), context_length: 8192 },
                    ModelInfo { id: "llama-3.3-70b".into(), name: "Llama 3.3 70B".into(), context_length: 8192 },
                ],
            });
        }

        // Check for OpenRouter
        if std::env::var("OPENROUTER_API_KEY").is_ok() {
            providers.push(ProviderInfo {
                id: "openrouter".into(),
                name: "OpenRouter".into(),
                available: true,
                models: vec![
                    ModelInfo { id: "meta-llama/llama-3.1-8b-instruct".into(), name: "Llama 3.1 8B Instruct".into(), context_length: 131072 },
                    ModelInfo { id: "anthropic/claude-3.5-sonnet".into(), name: "Claude 3.5 Sonnet".into(), context_length: 200000 },
                ],
            });
        }

        // Check for GMI Cloud
        if std::env::var("GMI_API_KEY").is_ok() {
            providers.push(ProviderInfo {
                id: "gmi".into(),
                name: "GMI Cloud".into(),
                available: true,
                models: vec![
                    ModelInfo { id: "deepseek-ai/DeepSeek-V3.1".into(), name: "DeepSeek V3.1".into(), context_length: 65536 },
                ],
            });
        }

        // Always add mock provider
        providers.push(ProviderInfo {
            id: "mock".into(),
            name: "Mock Provider".into(),
            available: true,
            models: vec![
                ModelInfo { id: "mock-model".into(), name: "Mock Model".into(), context_length: 4096 },
            ],
        });

        providers
    }

    pub fn subscribe_to_updates(&self) -> broadcast::Receiver<RunUpdate> {
        self.run_updates.subscribe()
    }

    pub fn broadcast_update(&self, update: RunUpdate) {
        let _ = self.run_updates.send(update);
    }

    pub fn subscribe_to_core_updates(&self) -> broadcast::Receiver<CoreBenchmarkUpdate> {
        self.core_updates.subscribe()
    }

    pub fn broadcast_core_update(&self, update: CoreBenchmarkUpdate) {
        let _ = self.core_updates.send(update);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about an available provider
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub models: Vec<ModelInfo>,
}

/// Information about a model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub context_length: u32,
}

/// Update message for WebSocket clients (LLM benchmarks)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RunUpdate {
    RunStarted { run_id: String },
    RunProgress { run_id: String, progress: f32, current_test: String },
    TestCompleted { run_id: String, test_id: String, success: bool },
    RunCompleted { run_id: String },
    RunFailed { run_id: String, error: String },
}

/// Update message for core benchmark WebSocket clients
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum CoreBenchmarkUpdate {
    Started { report_id: String },
    Progress { report_id: String, progress: f32, current_test: String },
    TestCompleted { report_id: String, test_id: String, success: bool },
    Completed { report_id: String },
    Failed { report_id: String, error: String },
}
