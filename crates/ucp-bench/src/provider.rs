//! LLM Provider abstraction layer.
//!
//! Providers: Groq, Cerebras, GMI Cloud, OpenRouter
//!
//! Cost calculation strategy:
//! - Use API-reported token counts (prompt_tokens, completion_tokens) when available
//! - Fall back to heuristic estimation only if API doesn't return usage
//! - Groq/Cerebras/GMI: static pricing tables from official docs
//! - OpenRouter: dynamic pricing fetched from /api/v1/models endpoint

use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;
use thiserror::Error;

use crate::tokenizer::count_tokens;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Rate limited: retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },
    #[error("Timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Missing API key for provider: {0}")]
    MissingApiKey(String),
}

pub type ProviderResult<T> = Result<T, ProviderError>;

/// Pricing information for cost calculation (per token, USD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPricing {
    pub input_cost_per_token: f64,
    pub output_cost_per_token: f64,
    pub currency: String,
}

impl TokenPricing {
    pub fn calculate_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        (input_tokens as f64) * self.input_cost_per_token
            + (output_tokens as f64) * self.output_cost_per_token
    }
}

/// Message role in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: Role::System, content: content.into() }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: Role::User, content: content.into() }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: Role::Assistant, content: content.into() }
    }
}

/// Request to LLM provider
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stop_sequences: Vec<String>,
}

impl CompletionRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            max_tokens: Some(2048),
            temperature: Some(0.0),
            stop_sequences: Vec::new(),
        }
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

/// Reason for completion finishing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    Error,
}

/// Response from LLM provider
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub latency_ms: u64,
    pub finish_reason: FinishReason,
    pub model: String,
}

/// Abstract LLM provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Provider identifier (e.g., "openai", "anthropic")
    fn provider_id(&self) -> &str;

    /// Model identifier (e.g., "gpt-4o", "claude-3-sonnet")
    fn model_id(&self) -> &str;

    /// Execute a completion request
    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse>;

    /// Get pricing info for cost calculation
    fn pricing(&self) -> TokenPricing;

    /// Full identifier (provider/model)
    fn full_id(&self) -> String {
        format!("{}/{}", self.provider_id(), self.model_id())
    }
}

// =============================================================================
// Pricing Tables (from official documentation)
// =============================================================================

/// Groq pricing - https://groq.com/pricing (as of 2024)
/// Prices are per token in USD
fn groq_pricing_for(model: &str) -> TokenPricing {
    let (input, output) = match model {
        // Llama 4 models
        "meta-llama/llama-4-scout-17b-16e-instruct" => (0.11 / 1_000_000.0, 0.34 / 1_000_000.0),
        "meta-llama/llama-4-maverick-17b-128e-instruct" => (0.50 / 1_000_000.0, 0.77 / 1_000_000.0),
        // Llama 3.3 models
        "llama-3.3-70b-versatile" => (0.59 / 1_000_000.0, 0.79 / 1_000_000.0),
        "llama-3.3-70b-specdec" => (0.59 / 1_000_000.0, 0.99 / 1_000_000.0),
        // Llama 3.1 models
        "llama-3.1-8b-instant" => (0.05 / 1_000_000.0, 0.08 / 1_000_000.0),
        // Qwen models
        "qwen/qwen3-32b" => (0.34 / 1_000_000.0, 0.34 / 1_000_000.0),
        // GPT-OSS models
        "openai/gpt-oss-20b" => (0.10 / 1_000_000.0, 0.10 / 1_000_000.0),
        "openai/gpt-oss-120b" => (0.60 / 1_000_000.0, 0.60 / 1_000_000.0),
        // Default fallback
        _ => (0.05 / 1_000_000.0, 0.08 / 1_000_000.0),
    };
    TokenPricing {
        input_cost_per_token: input,
        output_cost_per_token: output,
        currency: "USD".into(),
    }
}

/// Cerebras pricing - https://cerebras.ai/pricing (as of 2024)
/// Prices are per token in USD
fn cerebras_pricing_for(model: &str) -> TokenPricing {
    let (input, output) = match model {
        // Llama 3.3 70B
        "llama-3.3-70b" => (0.85 / 1_000_000.0, 1.20 / 1_000_000.0),
        "llama3.3-70b" => (0.85 / 1_000_000.0, 1.20 / 1_000_000.0),
        // Llama 3.1 models
        "llama-3.1-8b" | "llama3.1-8b" => (0.10 / 1_000_000.0, 0.10 / 1_000_000.0),
        "llama-3.1-70b" | "llama3.1-70b" => (0.85 / 1_000_000.0, 1.20 / 1_000_000.0),
        // Qwen models
        "qwen-3-32b" | "qwen3-32b" => (0.40 / 1_000_000.0, 0.80 / 1_000_000.0),
        // Default fallback
        _ => (0.10 / 1_000_000.0, 0.10 / 1_000_000.0),
    };
    TokenPricing {
        input_cost_per_token: input,
        output_cost_per_token: output,
        currency: "USD".into(),
    }
}

/// GMI Cloud pricing - https://docs.gmicloud.ai/inference-engine/billing/price
/// Prices are per token in USD (serverless tier)
fn gmi_pricing_for(model: &str) -> TokenPricing {
    let (input, output) = match model {
        // DeepSeek models
        "deepseek-ai/DeepSeek-R1" => (0.55 / 1_000_000.0, 2.19 / 1_000_000.0),
        "deepseek-ai/DeepSeek-V3" => (0.27 / 1_000_000.0, 1.10 / 1_000_000.0),
        // Llama models
        "meta-llama/Llama-4-Maverick-17B-128E-Instruct-FP8" => (0.18 / 1_000_000.0, 0.59 / 1_000_000.0),
        "meta-llama/Llama-3.3-70B-Instruct" => (0.59 / 1_000_000.0, 0.79 / 1_000_000.0),
        // Qwen models
        "Qwen/Qwen3-235B-A22B-FP8" => (0.30 / 1_000_000.0, 0.60 / 1_000_000.0),
        "Qwen/QwQ-32B" => (0.20 / 1_000_000.0, 0.60 / 1_000_000.0),
        // Default fallback
        _ => (0.20 / 1_000_000.0, 0.60 / 1_000_000.0),
    };
    TokenPricing {
        input_cost_per_token: input,
        output_cost_per_token: output,
        currency: "USD".into(),
    }
}

/// OpenRouter pricing cache - fetched dynamically from /api/v1/models
/// Key: model ID, Value: (input_cost_per_token, output_cost_per_token)
static OPENROUTER_PRICING_CACHE: Lazy<RwLock<HashMap<String, (f64, f64)>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Fetch OpenRouter pricing from API and cache it
pub async fn fetch_openrouter_pricing(api_key: &str) -> Result<(), ProviderError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://openrouter.ai/api/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap_or_default();
        return Err(ProviderError::Api {
            status: status.as_u16(),
            message: text,
        });
    }

    let json: Value = response.json().await?;
    let mut cache = OPENROUTER_PRICING_CACHE.write().unwrap();

    if let Some(models) = json["data"].as_array() {
        for model in models {
            if let (Some(id), Some(pricing)) = (model["id"].as_str(), model.get("pricing")) {
                let prompt = pricing["prompt"]
                    .as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let completion = pricing["completion"]
                    .as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                cache.insert(id.to_string(), (prompt, completion));
            }
        }
    }

    Ok(())
}

/// Get OpenRouter pricing for a model (from cache or default)
fn openrouter_pricing_for(model: &str) -> TokenPricing {
    let cache = OPENROUTER_PRICING_CACHE.read().unwrap();
    let (input, output) = cache.get(model).copied().unwrap_or_else(|| {
        // Fallback pricing if not in cache - conservative estimate
        (0.001 / 1000.0, 0.002 / 1000.0)
    });
    TokenPricing {
        input_cost_per_token: input,
        output_cost_per_token: output,
        currency: "USD".into(),
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Extract token usage from API response
fn extract_token_usage(json: &Value, field: &str) -> Option<u32> {
    json["usage"][field].as_u64().map(|v| v as u32)
}

/// Snapshot messages for token estimation fallback
fn snapshot_messages(messages: &[Message]) -> String {
    messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

// =============================================================================
// Provider Implementations
// =============================================================================

/// Groq provider implementation
pub struct GroqProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl GroqProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://api.groq.com/openai/v1".into(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait]
impl LlmProvider for GroqProvider {
    fn provider_id(&self) -> &str { "groq" }
    fn model_id(&self) -> &str { &self.model }

    fn pricing(&self) -> TokenPricing {
        groq_pricing_for(&self.model)
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        let start = Instant::now();
        let prompt_snapshot = snapshot_messages(&request.messages);

        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                },
                "content": m.content,
            })
        }).collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if !request.stop_sequences.is_empty() {
            body["stop"] = serde_json::json!(request.stop_sequences);
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let latency_ms = start.elapsed().as_millis() as u64;
        let status = response.status();

        if status == 429 {
            return Err(ProviderError::RateLimited { retry_after_ms: 1000 });
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api { status: status.as_u16(), message: text });
        }

        let json: Value = response.json().await?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ProviderError::InvalidResponse("missing content".into()))?
            .to_string();

        let input_tokens = extract_token_usage(&json, "prompt_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &prompt_snapshot) as u32);
        let output_tokens = extract_token_usage(&json, "completion_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &content) as u32);

        let finish_reason = match json["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("tool_calls") => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        };

        Ok(CompletionResponse {
            content,
            input_tokens,
            output_tokens,
            latency_ms,
            finish_reason,
            model: self.model.clone(),
        })
    }
}

/// Cerebras provider implementation
pub struct CerebrasProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl CerebrasProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://api.cerebras.ai/v1".into(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait]
impl LlmProvider for CerebrasProvider {
    fn provider_id(&self) -> &str { "cerebras" }
    fn model_id(&self) -> &str { &self.model }

    fn pricing(&self) -> TokenPricing {
        cerebras_pricing_for(&self.model)
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        let start = Instant::now();
        let prompt_snapshot = snapshot_messages(&request.messages);

        let mut messages: Vec<serde_json::Value> = Vec::new();
        let mut system_content = String::new();

        // Cerebras doesn't support a separate 'system' field in the request body
        // Instead, we prepend system messages to the first user message
        for m in &request.messages {
            match m.role {
                Role::System => system_content.push_str(&m.content),
                Role::User => {
                    let content = if !system_content.is_empty() {
                        format!("{}\n\n{}", system_content, m.content)
                    } else {
                        m.content.clone()
                    };
                    messages.push(serde_json::json!({ "role": "user", "content": content }));
                    system_content.clear(); // Only apply to first user message
                }
                Role::Assistant => messages.push(serde_json::json!({ "role": "assistant", "content": m.content })),
            }
        }

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(2048),
        });
        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if !request.stop_sequences.is_empty() {
            body["stop_sequences"] = serde_json::json!(request.stop_sequences);
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let latency_ms = start.elapsed().as_millis() as u64;
        let status = response.status();

        if status == 429 {
            return Err(ProviderError::RateLimited { retry_after_ms: 1000 });
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api { status: status.as_u16(), message: text });
        }

        let json: Value = response.json().await?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ProviderError::InvalidResponse("missing content".into()))?
            .to_string();

        let input_tokens = extract_token_usage(&json, "prompt_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &prompt_snapshot) as u32);
        let output_tokens = extract_token_usage(&json, "completion_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &content) as u32);

        let finish_reason = match json["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("tool_calls") => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        };

        Ok(CompletionResponse {
            content,
            input_tokens,
            output_tokens,
            latency_ms,
            finish_reason,
            model: self.model.clone(),
        })
    }
}

/// OpenRouter provider implementation
pub struct OpenRouterProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
    referer: String,
    site_name: String,
}

impl OpenRouterProvider {
    pub fn new(
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://openrouter.ai/api/v1".into(),
            referer: "https://github.com/unified-content/ucp".into(),
            site_name: "UCP Bench".into(),
        }
    }

    pub fn with_metadata(mut self, referer: impl Into<String>, site_name: impl Into<String>) -> Self {
        self.referer = referer.into();
        self.site_name = site_name.into();
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait]
impl LlmProvider for OpenRouterProvider {
    fn provider_id(&self) -> &str { "openrouter" }
    fn model_id(&self) -> &str { &self.model }

    fn pricing(&self) -> TokenPricing {
        openrouter_pricing_for(&self.model)
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        let start = Instant::now();
        let prompt_snapshot = snapshot_messages(&request.messages);

        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                },
                "content": m.content,
            })
        }).collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if !request.stop_sequences.is_empty() {
            body["stop"] = serde_json::json!(request.stop_sequences);
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", &self.referer)
            .header("X-Title", &self.site_name)
            .json(&body)
            .send()
            .await?;

        let latency_ms = start.elapsed().as_millis() as u64;
        let status = response.status();

        if status == 429 {
            return Err(ProviderError::RateLimited { retry_after_ms: 1000 });
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api { status: status.as_u16(), message: text });
        }

        let json: Value = response.json().await?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ProviderError::InvalidResponse("missing content".into()))?
            .to_string();

        let input_tokens = extract_token_usage(&json, "prompt_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &prompt_snapshot) as u32);
        let output_tokens = extract_token_usage(&json, "completion_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &content) as u32);

        let finish_reason = match json["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("tool_calls") => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        };

        Ok(CompletionResponse {
            content,
            input_tokens,
            output_tokens,
            latency_ms,
            finish_reason,
            model: self.model.clone(),
        })
    }
}

/// GMI Cloud provider implementation
pub struct GmiCloudProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl GmiCloudProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://api.gmi-serving.com/v1".into(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait]
impl LlmProvider for GmiCloudProvider {
    fn provider_id(&self) -> &str { "gmi" }
    fn model_id(&self) -> &str { &self.model }

    fn pricing(&self) -> TokenPricing {
        gmi_pricing_for(&self.model)
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        let start = Instant::now();
        let prompt_snapshot = snapshot_messages(&request.messages);

        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                },
                "content": m.content,
            })
        }).collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
        });

        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if !request.stop_sequences.is_empty() {
            body["stop"] = serde_json::json!(request.stop_sequences);
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let latency_ms = start.elapsed().as_millis() as u64;
        let status = response.status();

        if status == 429 {
            return Err(ProviderError::RateLimited { retry_after_ms: 1000 });
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api { status: status.as_u16(), message: text });
        }

        let json: Value = response.json().await?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ProviderError::InvalidResponse("missing content".into()))?
            .to_string();

        let input_tokens = extract_token_usage(&json, "prompt_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &prompt_snapshot) as u32);
        let output_tokens = extract_token_usage(&json, "completion_tokens")
            .unwrap_or_else(|| count_tokens(self.model_id(), &content) as u32);

        let finish_reason = match json["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("tool_calls") => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        };

        Ok(CompletionResponse {
            content,
            input_tokens,
            output_tokens,
            latency_ms,
            finish_reason,
            model: self.model.clone(),
        })
    }
}

/// Mock provider for testing without API calls
pub struct MockProvider {
    model: String,
    responses: std::sync::Mutex<Vec<String>>,
    latency_ms: u64,
}

impl MockProvider {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            responses: std::sync::Mutex::new(Vec::new()),
            latency_ms: 100,
        }
    }

    pub fn with_responses(mut self, responses: Vec<String>) -> Self {
        self.responses = std::sync::Mutex::new(responses);
        self
    }

    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.latency_ms = latency_ms;
        self
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    fn provider_id(&self) -> &str { "mock" }
    fn model_id(&self) -> &str { &self.model }

    fn pricing(&self) -> TokenPricing {
        TokenPricing {
            input_cost_per_token: 0.0,
            output_cost_per_token: 0.0,
            currency: "USD".into(),
        }
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.latency_ms)).await;

        let content = {
            let mut responses = self.responses.lock().unwrap();
            if responses.is_empty() {
                // Generate a mock UCL response based on last user message
                let last_msg = request.messages.last()
                    .map(|m| m.content.as_str())
                    .unwrap_or("");
                if last_msg.contains("EDIT") {
                    r#"EDIT blk_000000000001 SET text = "updated""#.to_string()
                } else if last_msg.contains("APPEND") {
                    r#"APPEND blk_000000000001 text :: "new content""#.to_string()
                } else {
                    r#"EDIT blk_000000000001 SET text = "response""#.to_string()
                }
            } else {
                responses.remove(0)
            }
        };

        let input_tokens = request.messages.iter()
            .map(|m| m.content.len() / 4)
            .sum::<usize>() as u32;
        let output_tokens = (content.len() / 4) as u32;

        Ok(CompletionResponse {
            content,
            input_tokens,
            output_tokens,
            latency_ms: self.latency_ms,
            finish_reason: FinishReason::Stop,
            model: self.model.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockProvider::new("mock-model")
            .with_responses(vec!["EDIT blk_abc SET x = 1".into()]);

        let request = CompletionRequest::new(vec![
            Message::user("Generate an EDIT command"),
        ]);

        let response = provider.complete(request).await.unwrap();
        assert_eq!(response.content, "EDIT blk_abc SET x = 1");
    }

    #[test]
    fn test_pricing_calculation() {
        // Test with per-token pricing (e.g., $1/1M input, $2/1M output)
        let pricing = TokenPricing {
            input_cost_per_token: 1.0 / 1_000_000.0,
            output_cost_per_token: 2.0 / 1_000_000.0,
            currency: "USD".into(),
        };

        // 1000 input tokens + 500 output tokens
        // = 1000 * (1/1M) + 500 * (2/1M)
        // = 0.001 + 0.001 = 0.002
        let cost = pricing.calculate_cost(1000, 500);
        assert!((cost - 0.002).abs() < 0.0000001);
    }

    #[test]
    fn test_groq_pricing() {
        let pricing = groq_pricing_for("llama-3.1-8b-instant");
        // $0.05/1M input, $0.08/1M output
        assert!((pricing.input_cost_per_token - 0.05 / 1_000_000.0).abs() < 1e-12);
        assert!((pricing.output_cost_per_token - 0.08 / 1_000_000.0).abs() < 1e-12);
    }

    #[test]
    fn test_cerebras_pricing() {
        let pricing = cerebras_pricing_for("llama-3.3-70b");
        // $0.85/1M input, $1.20/1M output
        assert!((pricing.input_cost_per_token - 0.85 / 1_000_000.0).abs() < 1e-12);
        assert!((pricing.output_cost_per_token - 1.20 / 1_000_000.0).abs() < 1e-12);
    }

    #[test]
    fn test_gmi_pricing() {
        let pricing = gmi_pricing_for("deepseek-ai/DeepSeek-R1");
        // $0.55/1M input, $2.19/1M output
        assert!((pricing.input_cost_per_token - 0.55 / 1_000_000.0).abs() < 1e-12);
        assert!((pricing.output_cost_per_token - 2.19 / 1_000_000.0).abs() < 1e-12);
    }
}
