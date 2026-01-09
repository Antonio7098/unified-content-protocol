//! API request handlers.

use super::state::{AppState, CoreBenchmarkUpdate, RunUpdate};
use crate::core::metrics::{CoreBenchmarkReport, CoreBenchmarkStatus};
use crate::core::{
    runner::{list_benchmarks, list_inputs},
    CoreBenchmarkConfig, CoreBenchmarkRunner, InputCategory, CORE_INPUTS,
};
use crate::documents::{DocumentDetailPayload, DocumentRegistry, DocumentSummary, DOCUMENTS};
use crate::storage::{default_core_benchmark_storage, CoreBenchmarkStorage};
use crate::suite::{
    category::TestCategoryId,
    config::{BenchmarkSuiteConfig, MatrixConfig},
    result::DocumentSnapshot,
    BenchmarkSuite, RunStatus, SuiteRunResult,
};
use crate::test_document;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    Json,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<AppState>>;

// ============================================================================
// Registry Handlers
// ============================================================================

pub async fn get_registry(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    Json(state.registry.summary())
}

pub async fn get_categories(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let categories: Vec<_> = state.registry.categories().cloned().collect();
    Json(categories)
}

pub async fn get_category(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    let category_id = TestCategoryId::new(&id);
    match state.registry.get_category(&category_id) {
        Some(cat) => Json(serde_json::json!({ "category": cat })).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Category not found").into_response(),
    }
}

pub async fn get_category_tests(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    let category_id = TestCategoryId::new(&id);
    match state.registry.get_tests(&category_id) {
        Some(tests) => Json(tests).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Category not found").into_response(),
    }
}

pub async fn list_tests(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let tests: Vec<_> = state.registry.all_tests().into_iter().cloned().collect();
    Json(tests)
}

pub async fn get_test(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    match state.registry.find_test(&id) {
        Some(test) => {
            let category_id = TestCategoryId::new(&test.command_type);
            let category = state.registry.get_category(&category_id).cloned();
            Json(serde_json::json!({
                "test": test,
                "category": category,
                "document": {
                    "description": crate::test_document::document_description()
                }
            }))
            .into_response()
        }
        None => (axum::http::StatusCode::NOT_FOUND, "Test not found").into_response(),
    }
}

// ============================================================================
// Provider Handlers
// ============================================================================

pub async fn get_providers(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    Json(state.available_providers.clone())
}

pub async fn get_available_providers(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let available: Vec<_> = state
        .available_providers
        .iter()
        .filter(|p| p.available)
        .cloned()
        .collect();
    Json(available)
}

// ============================================================================
// Suite Handlers
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateSuiteRequest {
    pub name: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub config: Option<BenchmarkSuiteConfig>,
}

pub async fn list_suites(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let suites: Vec<_> = state.suites.values().cloned().collect();
    Json(suites)
}

pub async fn create_suite(
    State(state): State<SharedState>,
    Json(req): Json<CreateSuiteRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    let categories: Vec<TestCategoryId> = req
        .categories
        .into_iter()
        .map(TestCategoryId::new)
        .collect();

    let suite = BenchmarkSuite::new(req.name, req.description.unwrap_or_default())
        .with_categories(categories)
        .with_config(req.config.unwrap_or_default());

    let id = suite.id.clone();
    state.suites.insert(id.clone(), suite.clone());

    (axum::http::StatusCode::CREATED, Json(suite))
}

pub async fn get_suite(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    match state.suites.get(&id) {
        Some(suite) => Json(serde_json::json!({ "suite": suite })).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Suite not found").into_response(),
    }
}

pub async fn update_suite(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<CreateSuiteRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    if let Some(suite) = state.suites.get_mut(&id) {
        suite.name = req.name;
        suite.description = req.description.unwrap_or_default();
        suite.categories = req
            .categories
            .into_iter()
            .map(TestCategoryId::new)
            .collect();
        if let Some(config) = req.config {
            suite.config = config;
        }
        Json(serde_json::json!({ "suite": suite.clone() })).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, "Suite not found").into_response()
    }
}

pub async fn delete_suite(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut state = state.write().await;
    match state.suites.remove(&id) {
        Some(_) => axum::http::StatusCode::NO_CONTENT.into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Suite not found").into_response(),
    }
}

// ============================================================================
// Run Handlers
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct StartRunRequest {
    pub suite_id: String,
    pub config_override: Option<BenchmarkSuiteConfig>,
}

pub async fn list_runs(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let runs: Vec<_> = state.runs.values().cloned().collect();
    Json(runs)
}

pub async fn start_run(
    State(state): State<SharedState>,
    Json(req): Json<StartRunRequest>,
) -> impl IntoResponse {
    let mut state_guard = state.write().await;

    let suite = match state_guard.suites.get(&req.suite_id) {
        Some(s) => s.clone(),
        None => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Suite not found" })),
            )
                .into_response()
        }
    };

    let config = req.config_override.unwrap_or(suite.config.clone());
    let mut run = SuiteRunResult::new(suite.id.clone(), config.clone());
    let run_id = run.run_id.clone();

    run.status = RunStatus::Running {
        progress: 0.0,
        current_test: "Initializing...".into(),
    };

    state_guard.runs.insert(run_id.clone(), run.clone());

    // Broadcast run started
    state_guard.broadcast_update(RunUpdate::RunStarted {
        run_id: run_id.clone(),
    });

    // Spawn background task to run the benchmark
    let state_clone = state.clone();
    let run_id_clone = run_id.clone();
    tokio::spawn(async move {
        run_benchmark(state_clone, run_id_clone, suite, config).await;
    });

    (
        axum::http::StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "run_id": run_id,
            "status": "started"
        })),
    )
        .into_response()
}

pub async fn get_run(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    match state.runs.get(&id) {
        Some(run) => Json(run).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Run not found").into_response(),
    }
}

pub async fn cancel_run(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut state = state.write().await;
    if let Some(run) = state.runs.get_mut(&id) {
        if matches!(run.status, RunStatus::Running { .. } | RunStatus::Pending) {
            run.status = RunStatus::Cancelled;
            run.completed_at = Some(chrono::Utc::now());
            return Json(serde_json::json!({ "status": "cancelled" })).into_response();
        }
    }
    (axum::http::StatusCode::BAD_REQUEST, "Cannot cancel run").into_response()
}

pub async fn get_run_results(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    match state.runs.get(&id) {
        Some(run) => Json(&run.results_by_provider).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Run not found").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct TestResultQuery {
    pub provider: Option<String>,
}

pub async fn get_test_result(
    State(state): State<SharedState>,
    Path((run_id, test_id)): Path<(String, String)>,
    Query(query): Query<TestResultQuery>,
) -> impl IntoResponse {
    let state = state.read().await;

    let run = match state.runs.get(&run_id) {
        Some(r) => r,
        None => return (axum::http::StatusCode::NOT_FOUND, "Run not found").into_response(),
    };

    // Search for the test result
    for (provider_key, provider_results) in &run.results_by_provider {
        if let Some(ref filter) = query.provider {
            if !provider_key.contains(filter) {
                continue;
            }
        }

        for category_results in provider_results.results_by_category.values() {
            for result in &category_results.tests {
                if result.test_id == test_id {
                    return Json(result).into_response();
                }
            }
        }
    }

    (axum::http::StatusCode::NOT_FOUND, "Test result not found").into_response()
}

// ============================================================================
// Document Handlers
// ============================================================================

/// List available benchmark documents
pub async fn list_documents() -> impl IntoResponse {
    let docs = crate::documents::DOCUMENTS.list();
    Json(docs)
}

/// Get a specific benchmark document
pub async fn get_document(Path(id): Path<String>) -> impl IntoResponse {
    match crate::documents::DOCUMENTS.get(&id) {
        Some(def) => {
            let doc = def.build();
            let snapshot = DocumentSnapshot::from_document(&doc);
            let ucm = def.to_ucm_json(&doc);
            let markdown = match ucp_translator_markdown::render_markdown(&doc) {
                Ok(md) => md,
                Err(e) => format!("Error rendering markdown: {}", e),
            };
            Json(serde_json::json!({
                "ucm": ucm,
                "description": def.prompt_text(),
                "snapshot": snapshot,
                "markdown": markdown,
            }))
            .into_response()
        }
        None => (
            axum::http::StatusCode::NOT_FOUND,
            "Document not found",
        )
            .into_response(),
    }
}

pub async fn get_test_document() -> impl IntoResponse {
    let doc = test_document::create_test_document();
    let snapshot = DocumentSnapshot::from_document(&doc);
    let ucm = test_document::document_ucm_json(&doc);
    let markdown = match ucp_translator_markdown::render_markdown(&doc) {
        Ok(md) => md,
        Err(e) => format!("Error rendering markdown: {}", e),
    };
    Json(serde_json::json!({
        "ucm": ucm,
        "description": test_document::document_description(),
        "snapshot": snapshot,
        "markdown": markdown,
    }))
}

// ============================================================================
// WebSocket Handler
// ============================================================================

pub async fn websocket_handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to run updates
    let mut rx = {
        let state = state.read().await;
        state.subscribe_to_updates()
    };

    // Send updates to the client
    let send_task = tokio::spawn(async move {
        while let Ok(update) = rx.recv().await {
            let msg = serde_json::to_string(&update).unwrap_or_default();
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages (for future use)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(_msg)) = receiver.next().await {
            // Handle client messages if needed
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

// ============================================================================
// Background Benchmark Runner
// ============================================================================

async fn run_benchmark(
    state: SharedState,
    run_id: String,
    suite: BenchmarkSuite,
    config: BenchmarkSuiteConfig,
) {
    use crate::agent::BenchmarkAgent;
    use crate::provider::{
        CerebrasProvider, GroqProvider, LlmProvider, MockProvider, OpenRouterProvider,
    };
    use crate::suite::result::{DetailedTestResult, ExecutionContext};
    use crate::suite::{CategoryResults, ProviderResults};
    use std::sync::Arc;

    let registry = {
        let state = state.read().await;
        state.registry.clone()
    };

    // Get test cases for selected categories, filtered by document when provided
    let selected_document_id = config.document_id.clone();
    let selected_cases = registry.get_tests_for_categories(&suite.categories);
    let test_cases: Vec<&crate::test_cases::TestCase> = selected_cases
        .into_iter()
        .filter(|case| {
            if let Some(ref doc_id) = selected_document_id {
                &case.document_id == doc_id
            } else {
                true
            }
        })
        .collect();
    let total_tests = test_cases.len() * config.matrix.pairs.len();
    let mut completed = 0;

    for pair in config.matrix.enabled_pairs() {
        let provider: Arc<dyn LlmProvider> = match pair.provider_id.as_str() {
            "groq" => {
                if let Ok(key) = std::env::var("GROQ_API_KEY") {
                    Arc::new(GroqProvider::new(key, pair.model_id.clone()))
                } else {
                    continue;
                }
            }
            "cerebras" => {
                if let Ok(key) = std::env::var("CEREBRAS_API_KEY") {
                    Arc::new(CerebrasProvider::new(key, pair.model_id.clone()))
                } else {
                    continue;
                }
            }
            "openrouter" => {
                if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
                    Arc::new(OpenRouterProvider::new(key, pair.model_id.clone()))
                } else {
                    continue;
                }
            }
            "mock" => Arc::new(MockProvider::new(&pair.model_id)),
            _ => continue,
        };

        let provider_key = pair.full_id();
        let mut provider_results = ProviderResults {
            provider_id: pair.provider_id.clone(),
            model_id: pair.model_id.clone(),
            results_by_category: std::collections::HashMap::new(),
            total_tests: 0,
            passed: 0,
            failed: 0,
            total_cost_usd: 0.0,
            total_latency_ms: 0,
            avg_latency_ms: 0,
        };

        let doc_def = selected_document_id
            .as_deref()
            .and_then(|id| crate::documents::DOCUMENTS.get(id))
            .or_else(|| crate::documents::DOCUMENTS.default())
            .expect("No document definition available");
        let mut agent = BenchmarkAgent::new(provider.clone(), doc_def, config.execute_commands);

        for test_case in &test_cases {
            // Check if run was cancelled
            {
                let state = state.read().await;
                if let Some(run) = state.runs.get(&run_id) {
                    if matches!(run.status, RunStatus::Cancelled) {
                        return;
                    }
                }
            }

            // Update progress
            completed += 1;
            let progress = completed as f32 / total_tests as f32;
            {
                let mut state = state.write().await;
                if let Some(run) = state.runs.get_mut(&run_id) {
                    run.status = RunStatus::Running {
                        progress,
                        current_test: test_case.id.clone(),
                    };
                }
                state.broadcast_update(RunUpdate::RunProgress {
                    run_id: run_id.clone(),
                    progress,
                    current_test: test_case.id.clone(),
                });
            }

            // Run the test
            let result = agent.run_test(test_case).await;

            // Convert to detailed result
            let mut detailed = DetailedTestResult::new(
                test_case.id.clone(),
                test_case.command_type.clone(),
                pair.provider_id.clone(),
                pair.model_id.clone(),
            );

            detailed.latency_ms = result.latency_ms;
            detailed.input_tokens = result.input_tokens;
            detailed.output_tokens = result.output_tokens;
            detailed.cost_usd = result.cost_usd;
            detailed.success = result.is_success();
            detailed.parse_success = result.parse_success;
            detailed.execute_success = result.execute_success;
            detailed.semantic_score = result.semantic_score;
            detailed.efficiency_score = result.efficiency_score;

            detailed.context = ExecutionContext {
                test_description: test_case.description.clone(),
                task_prompt: test_case.prompt.clone(),
                system_prompt: String::new(), // Could capture from agent
                full_user_prompt: result.debug_prompt.clone().unwrap_or_default(),
                raw_response: result.debug_raw_response.clone().unwrap_or_default(),
                extracted_ucl: result.generated_ucl.clone(),
                parsed_command: None,
                expected_pattern: result.expected_pattern.clone(),
                pattern_matched: None,
            };

            if let Some(ref msg) = result.error_message {
                detailed.error = Some(crate::suite::result::TestError {
                    category: match result.error_category {
                        Some(crate::metrics::ErrorCategory::ParseError) => {
                            crate::suite::result::ErrorCategory::ParseError
                        }
                        Some(crate::metrics::ErrorCategory::ExecutionError) => {
                            crate::suite::result::ErrorCategory::ExecutionError
                        }
                        _ => crate::suite::result::ErrorCategory::ProviderError,
                    },
                    message: msg.clone(),
                    details: None,
                });
            }

            // Add to category results
            let category_id = TestCategoryId::new(&test_case.command_type);
            let category_results = provider_results
                .results_by_category
                .entry(category_id.clone())
                .or_insert_with(|| CategoryResults {
                    category_id,
                    tests: Vec::new(),
                    passed: 0,
                    failed: 0,
                    success_rate: 0.0,
                    avg_latency_ms: 0,
                    total_cost_usd: 0.0,
                });

            if detailed.success {
                category_results.passed += 1;
                provider_results.passed += 1;
            } else {
                category_results.failed += 1;
                provider_results.failed += 1;
            }

            category_results.total_cost_usd += detailed.cost_usd;
            provider_results.total_cost_usd += detailed.cost_usd;
            provider_results.total_latency_ms += detailed.latency_ms;
            provider_results.total_tests += 1;

            category_results.tests.push(detailed);

            // Broadcast test completed
            {
                let state = state.read().await;
                state.broadcast_update(RunUpdate::TestCompleted {
                    run_id: run_id.clone(),
                    test_id: test_case.id.clone(),
                    success: result.is_success(),
                });
            }

            // Reset agent for next test
            agent.reset();
        }

        // Calculate averages
        if provider_results.total_tests > 0 {
            provider_results.avg_latency_ms =
                provider_results.total_latency_ms / provider_results.total_tests as u64;
        }

        for cat_results in provider_results.results_by_category.values_mut() {
            let total = cat_results.passed + cat_results.failed;
            if total > 0 {
                cat_results.success_rate = cat_results.passed as f32 / total as f32;
                cat_results.avg_latency_ms =
                    cat_results.tests.iter().map(|t| t.latency_ms).sum::<u64>() / total as u64;
            }
        }

        // Store provider results
        {
            let mut state = state.write().await;
            if let Some(run) = state.runs.get_mut(&run_id) {
                run.results_by_provider
                    .insert(provider_key, provider_results);
            }
        }
    }

    // Mark run as completed
    {
        let mut state = state.write().await;
        if let Some(run) = state.runs.get_mut(&run_id) {
            run.mark_completed();
        }
        state.broadcast_update(RunUpdate::RunCompleted {
            run_id: run_id.clone(),
        });
    }
}

// ============================================================================
// Core Benchmark Handlers
// ============================================================================

/// List all available core benchmark types.
pub async fn list_core_benchmarks() -> impl IntoResponse {
    Json(list_benchmarks())
}

/// List all core test inputs.
pub async fn list_core_inputs() -> impl IntoResponse {
    Json(list_inputs())
}

/// Get a specific core test input.
pub async fn get_core_input(Path(id): Path<String>) -> impl IntoResponse {
    match CORE_INPUTS.get(&id) {
        Some(input) => Json(serde_json::json!({ "input": input })).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Input not found").into_response(),
    }
}

/// List core input categories.
pub async fn list_core_categories() -> impl IntoResponse {
    let categories: Vec<_> = InputCategory::all()
        .iter()
        .map(|c| {
            serde_json::json!({
                "id": format!("{:?}", c),
                "name": c.display_name(),
                "input_count": CORE_INPUTS.by_category(*c).len(),
            })
        })
        .collect();
    Json(categories)
}

/// Get inputs for a specific category.
pub async fn get_core_category_inputs(Path(id): Path<String>) -> impl IntoResponse {
    let category = match id.to_lowercase().as_str() {
        "markdown" => InputCategory::Markdown,
        "ucl" => InputCategory::Ucl,
        "document" => InputCategory::Document,
        "normalization" => InputCategory::Normalization,
        "json" => InputCategory::Json,
        "table" => InputCategory::Table,
        "codeblock" | "code_block" => InputCategory::CodeBlock,
        _ => return (axum::http::StatusCode::NOT_FOUND, "Category not found").into_response(),
    };

    let inputs: Vec<_> = CORE_INPUTS
        .by_category(category)
        .into_iter()
        .cloned()
        .collect();
    Json(inputs).into_response()
}

#[derive(Debug, Deserialize)]
pub struct StartCoreBenchmarkRequest {
    pub name: String,
    pub categories: Option<Vec<String>>,
    pub input_ids: Option<Vec<String>>,
    pub iterations: Option<u32>,
}

/// Start a core benchmark run.
pub async fn start_core_benchmark(
    State(state): State<SharedState>,
    Json(req): Json<StartCoreBenchmarkRequest>,
) -> impl IntoResponse {
    // Parse categories
    let categories = req
        .categories
        .map(|cats| {
            cats.iter()
                .filter_map(|c| match c.to_lowercase().as_str() {
                    "markdown" => Some(InputCategory::Markdown),
                    "markdownrender" | "markdown_render" | "ucm_to_markdown" => {
                        Some(InputCategory::MarkdownRender)
                    }
                    "ucl" => Some(InputCategory::Ucl),
                    "document" => Some(InputCategory::Document),
                    "normalization" => Some(InputCategory::Normalization),
                    "json" => Some(InputCategory::Json),
                    "table" => Some(InputCategory::Table),
                    "codeblock" | "code_block" => Some(InputCategory::CodeBlock),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_else(|| InputCategory::all().to_vec());

    let config = CoreBenchmarkConfig {
        categories,
        iterations: req.iterations.unwrap_or(100),
        warmup_iterations: 10,
        input_ids: req.input_ids.unwrap_or_default(),
    };

    // Create initial report placeholder
    let report = CoreBenchmarkReport::new(
        &req.name,
        crate::core::metrics::CoreBenchmarkReportConfig {
            categories: config
                .categories
                .iter()
                .map(|c| format!("{:?}", c))
                .collect(),
            iterations: config.iterations,
            warmup_iterations: config.warmup_iterations,
        },
    );
    let report_id = report.report_id.clone();

    // Store in state
    {
        let mut state_guard = state.write().await;
        state_guard.core_runs.insert(report_id.clone(), report);
        state_guard.broadcast_core_update(CoreBenchmarkUpdate::Started {
            report_id: report_id.clone(),
        });
    }

    // Spawn background task
    let state_clone = state.clone();
    let report_id_clone = report_id.clone();
    let name = req.name.clone();
    tokio::spawn(async move {
        run_core_benchmark(state_clone, report_id_clone, name, config).await;
    });

    (
        axum::http::StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "report_id": report_id,
            "status": "started"
        })),
    )
        .into_response()
}

/// List active core benchmark runs.
pub async fn list_core_runs(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let runs: Vec<_> = state.core_runs.values().cloned().collect();
    Json(runs)
}

/// Get a specific core benchmark run.
pub async fn get_core_run(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;
    match state.core_runs.get(&id) {
        Some(run) => Json(run).into_response(),
        None => {
            // Try loading from storage
            let storage = default_core_benchmark_storage();
            match storage.load(&id) {
                Ok(report) => Json(report).into_response(),
                Err(_) => (axum::http::StatusCode::NOT_FOUND, "Run not found").into_response(),
            }
        }
    }
}

/// Get test result from a core benchmark run.
pub async fn get_core_test_result(
    State(state): State<SharedState>,
    Path((run_id, test_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let state = state.read().await;

    let report = match state.core_runs.get(&run_id) {
        Some(r) => r.clone(),
        None => {
            // Try loading from storage
            let storage = default_core_benchmark_storage();
            match storage.load(&run_id) {
                Ok(r) => r,
                Err(_) => {
                    return (axum::http::StatusCode::NOT_FOUND, "Run not found").into_response()
                }
            }
        }
    };

    for result in &report.results {
        if result.test_id == test_id {
            return Json(result).into_response();
        }
    }

    (axum::http::StatusCode::NOT_FOUND, "Test result not found").into_response()
}

/// List stored core benchmark history.
pub async fn list_core_history() -> impl IntoResponse {
    let storage = default_core_benchmark_storage();
    match storage.list_summaries() {
        Ok(summaries) => Json(summaries).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to load history: {}", e),
        )
            .into_response(),
    }
}

/// Delete a stored core benchmark.
pub async fn delete_core_run(Path(id): Path<String>) -> impl IntoResponse {
    let storage = default_core_benchmark_storage();
    match storage.delete(&id) {
        Ok(_) => axum::http::StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete: {}", e),
        )
            .into_response(),
    }
}

/// Background task to run core benchmarks.
async fn run_core_benchmark(
    state: SharedState,
    report_id: String,
    name: String,
    config: CoreBenchmarkConfig,
) {
    let (tx, _) = tokio::sync::broadcast::channel(100);

    // Create runner with updates channel
    let runner = CoreBenchmarkRunner::new(config).with_updates(tx.clone());

    // Subscribe to updates
    let mut rx = tx.subscribe();
    let state_for_updates = state.clone();
    let report_id_for_updates = report_id.clone();

    // Spawn task to forward updates to state
    tokio::spawn(async move {
        use crate::core::runner::CoreBenchmarkUpdate as RunnerUpdate;

        while let Ok(update) = rx.recv().await {
            let mut state_guard = state_for_updates.write().await;

            match &update {
                RunnerUpdate::Progress {
                    progress,
                    current_test,
                    ..
                } => {
                    if let Some(report) = state_guard.core_runs.get_mut(&report_id_for_updates) {
                        report.mark_running(current_test.clone(), *progress);
                    }
                    state_guard.broadcast_core_update(CoreBenchmarkUpdate::Progress {
                        report_id: report_id_for_updates.clone(),
                        progress: *progress,
                        current_test: current_test.clone(),
                    });
                }
                RunnerUpdate::TestCompleted {
                    test_id, success, ..
                } => {
                    state_guard.broadcast_core_update(CoreBenchmarkUpdate::TestCompleted {
                        report_id: report_id_for_updates.clone(),
                        test_id: test_id.clone(),
                        success: *success,
                    });
                }
                _ => {}
            }
        }
    });

    // Run the benchmark
    let report = runner.run(name).await;

    // Store results
    {
        let mut state_guard = state.write().await;
        state_guard
            .core_runs
            .insert(report_id.clone(), report.clone());

        // Persist to storage
        let storage = default_core_benchmark_storage();
        if let Err(e) = storage.save(&report) {
            tracing::error!("Failed to save core benchmark report: {}", e);
        }

        match &report.status {
            CoreBenchmarkStatus::Completed => {
                state_guard.broadcast_core_update(CoreBenchmarkUpdate::Completed {
                    report_id: report_id.clone(),
                });
            }
            CoreBenchmarkStatus::Failed { error } => {
                state_guard.broadcast_core_update(CoreBenchmarkUpdate::Failed {
                    report_id: report_id.clone(),
                    error: error.clone(),
                });
            }
            _ => {}
        }
    }
}

// ============================================================================
// Playground Handlers
// ============================================================================

/// List available documents for playground.
pub async fn list_playground_documents() -> impl IntoResponse {
    let docs = crate::documents::DOCUMENTS.list();
    Json(docs)
}

/// Get a specific document for playground.
pub async fn get_playground_document(Path(id): Path<String>) -> impl IntoResponse {
    match crate::documents::DOCUMENTS.get(&id) {
        Some(def) => {
            let doc = def.build();
            let snapshot = DocumentSnapshot::from_document(&doc);
            let ucm = def.to_ucm_json(&doc);

            let markdown = match ucp_translator_markdown::render_markdown(&doc) {
                Ok(md) => md,
                Err(e) => format!("Error rendering markdown: {}", e),
            };

            let payload = PlaygroundDocumentDetail {
                summary: DocumentSummary {
                    id: def.id.to_string(),
                    name: def.name.to_string(),
                    summary: def.summary.to_string(),
                    tags: def.tags.iter().map(|t| t.to_string()).collect(),
                },
                llm_description: def.prompt_text().to_string(),
                snapshot,
                ucm,
                markdown,
            };

            Json(payload).into_response()
        }
        None => (axum::http::StatusCode::NOT_FOUND, "Document not found").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PlaygroundChatRequest {
    pub document_id: String,
    pub provider_id: String,
    pub model_id: String,
    pub message: String,
    pub execute_commands: bool,
}

#[derive(Debug, Serialize)]
pub struct PlaygroundDocumentDetail {
    pub summary: DocumentSummary,
    pub llm_description: String,
    pub snapshot: DocumentSnapshot,
    pub ucm: Value,
    pub markdown: String,
}

#[derive(Debug, Serialize)]
pub struct PlaygroundChatResponse {
    pub message_id: String,
    pub timestamp: String,
    pub user_message: String,
    pub full_prompt: String,
    pub raw_response: String,
    pub extracted_ucl: String,
    pub parsed_commands: Vec<String>,
    pub parse_success: bool,
    pub execute_success: Option<bool>,
    pub document_before: Option<serde_json::Value>,
    pub document_after: Option<serde_json::Value>,
    pub diff: Option<serde_json::Value>,
    pub error: Option<String>,
    pub latency_ms: u64,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Send a chat message in playground and get response.
pub async fn playground_chat(Json(req): Json<PlaygroundChatRequest>) -> impl IntoResponse {
    use crate::agent::BenchmarkAgent;
    use crate::provider::{
        CerebrasProvider, GroqProvider, LlmProvider, MockProvider, OpenRouterProvider,
    };
    use crate::test_document;
    use std::sync::Arc;

    // Get document definition
    let doc_def = match crate::documents::DOCUMENTS.get(&req.document_id) {
        Some(def) => def,
        None => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Document not found"
                })),
            )
                .into_response();
        }
    };

    // Create provider
    let provider: Arc<dyn LlmProvider> = match req.provider_id.as_str() {
        "groq" => match std::env::var("GROQ_API_KEY") {
            Ok(key) => Arc::new(GroqProvider::new(key, req.model_id.clone())),
            Err(_) => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "GROQ_API_KEY not set"
                    })),
                )
                    .into_response();
            }
        },
        "cerebras" => match std::env::var("CEREBRAS_API_KEY") {
            Ok(key) => Arc::new(CerebrasProvider::new(key, req.model_id.clone())),
            Err(_) => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "CEREBRAS_API_KEY not set"
                    })),
                )
                    .into_response();
            }
        },
        "openrouter" => match std::env::var("OPENROUTER_API_KEY") {
            Ok(key) => Arc::new(OpenRouterProvider::new(key, req.model_id.clone())),
            Err(_) => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "OPENROUTER_API_KEY not set"
                    })),
                )
                    .into_response();
            }
        },
        "mock" => Arc::new(MockProvider::new(&req.model_id)),
        _ => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Unknown provider"
                })),
            )
                .into_response();
        }
    };

    // Create agent
    let mut agent = BenchmarkAgent::new(provider.clone(), doc_def.clone(), req.execute_commands);

    // Build prompt
    let user_prompt = format!(
        "{}\n\n## Document Structure\n{}\n\n## Task\n{}\n\nGenerate the UCL command:",
        "You are interacting with a document. The user wants to make changes to it.",
        doc_def.prompt_text(),
        req.message
    );

    let system_prompt = r#"You are a UCL (Unified Content Language) command generator. Your task is to generate valid UCL commands to manipulate documents.

## UCL Command Reference

### EDIT - Modify block content
```
EDIT <block_id> SET <path> = <value>
EDIT <block_id> SET <path> += <value>
```

### APPEND - Add new blocks
```
APPEND <parent_id> <content_type> :: <content>
APPEND <parent_id> <content_type> WITH label = "name" :: <content>
```

### MOVE - Relocate blocks
```
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```

### DELETE - Remove blocks
```
DELETE <block_id>
DELETE <block_id> CASCADE
```

## Rules
1. Output ONLY the UCL command(s), no explanations or markdown
2. Use exact block IDs as provided
3. String values must be quoted with double quotes
4. Block IDs have format: blk_XXXXXXXXXXXX (12 hex chars)
"#;

    let request = crate::provider::CompletionRequest::new(vec![
        crate::provider::Message::system(system_prompt),
        crate::provider::Message::user(&user_prompt),
    ])
    .with_max_tokens(1024)
    .with_temperature(0.0);

    // Call LLM
    let response = match provider.complete(request).await {
        Ok(r) => r,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // Extract UCL
    let raw_response = response.content.clone();
    let extracted_ucl = crate::agent::extract_ucl(&raw_response);

    // Parse commands
    let parse_result = ucl_parser::Parser::new(&extracted_ucl).parse_commands_only();

    let (parsed_commands, parse_success) = match &parse_result {
        Ok(cmds) => (cmds.iter().map(|c| format!("{:?}", c)).collect(), true),
        Err(_) => (vec![], false),
    };

    // Execute if requested
    let (document_before, document_after, diff, execute_success) =
        if req.execute_commands && parse_success {
            let doc_before = doc_def.to_ucm_json(agent.document());
            let execute_result = if let Ok(ref cmds) = parse_result {
                agent.execute_and_validate_for_playground(cmds)
            } else {
                Err("Parse failed".to_string())
            };
            let doc_after = doc_def.to_ucm_json(agent.document());

            let diff_json = if let Ok(ref cmds) = parse_result {
                Some(serde_json::json!({
                    "commands": cmds.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>()
                }))
            } else {
                None
            };

            (
                Some(doc_before),
                Some(doc_after),
                diff_json,
                Some(execute_result.is_ok()),
            )
        } else {
            (None, None, None, None)
        };

    let message_id = uuid::Uuid::new_v4().to_string();

    let response_data = PlaygroundChatResponse {
        message_id: message_id.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_message: req.message,
        full_prompt: format!("{}\n\n{}", system_prompt, user_prompt),
        raw_response,
        extracted_ucl,
        parsed_commands,
        parse_success,
        execute_success,
        document_before,
        document_after,
        diff,
        error: None,
        latency_ms: response.latency_ms,
        input_tokens: response.input_tokens,
        output_tokens: response.output_tokens,
    };

    Json(response_data).into_response()
}
