//! Benchmark report generation and formatting.

use crate::metrics::{BenchmarkMetrics, TestResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// Complete benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub benchmark_id: String,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: u64,
    pub models_tested: Vec<String>,
    pub metrics: BenchmarkMetrics,
    pub results: Vec<TestResult>,
}

impl BenchmarkReport {
    /// Export report to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Export report to JSON file
    pub fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = self
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(path, json)
    }

    /// Generate a human-readable summary
    pub fn summary(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!("# Benchmark Report: {}\n\n", self.name));
        s.push_str(&format!("**ID:** {}\n", self.benchmark_id));
        s.push_str(&format!(
            "**Date:** {}\n",
            self.start_time.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        s.push_str(&format!("**Duration:** {}s\n\n", self.duration_seconds));

        s.push_str("## Summary\n\n");
        s.push_str(&format!("| Metric | Value |\n"));
        s.push_str(&format!("|--------|-------|\n"));
        s.push_str(&format!("| Total Tests | {} |\n", self.metrics.total_tests));
        s.push_str(&format!("| Passed | {} |\n", self.metrics.passed));
        s.push_str(&format!("| Failed | {} |\n", self.metrics.failed));
        s.push_str(&format!(
            "| Success Rate | {:.1}% |\n",
            self.metrics.success_rate() * 100.0
        ));
        s.push_str(&format!(
            "| Total Cost | ${:.4} |\n",
            self.metrics.total_cost_usd
        ));
        s.push_str(&format!(
            "| Avg Latency | {}ms |\n",
            self.metrics.avg_latency_ms()
        ));
        s.push_str(&format!(
            "| P50 Latency | {}ms |\n",
            self.metrics.latency_p50_ms
        ));
        s.push_str(&format!(
            "| P95 Latency | {}ms |\n",
            self.metrics.latency_p95_ms
        ));
        s.push_str(&format!(
            "| P99 Latency | {}ms |\n\n",
            self.metrics.latency_p99_ms
        ));

        s.push_str("## Results by Model\n\n");
        for (model, metrics) in &self.metrics.by_model {
            s.push_str(&format!("### {}\n\n", model));
            s.push_str(&format!("- Tests: {}\n", metrics.total));
            s.push_str(&format!(
                "- Success Rate: {:.1}%\n",
                metrics.success_rate() * 100.0
            ));
            s.push_str(&format!("- Avg Latency: {}ms\n", metrics.avg_latency_ms()));
            s.push_str(&format!("- Total Cost: ${:.4}\n", metrics.total_cost_usd));
            s.push_str(&format!(
                "- Tokens: {} in / {} out\n\n",
                metrics.total_input_tokens, metrics.total_output_tokens
            ));

            s.push_str("| Command | Success | Avg Latency |\n");
            s.push_str("|---------|---------|-------------|\n");
            for (cmd, cmd_metrics) in &metrics.by_command {
                s.push_str(&format!(
                    "| {} | {:.0}% | {}ms |\n",
                    cmd,
                    cmd_metrics.success_rate() * 100.0,
                    cmd_metrics.avg_latency_ms()
                ));
            }
            s.push_str("\n");
        }

        // Failures section
        let failures: Vec<_> = self.results.iter().filter(|r| !r.is_success()).collect();
        if !failures.is_empty() {
            s.push_str("## Failures\n\n");
            for (i, failure) in failures.iter().enumerate().take(10) {
                s.push_str(&format!(
                    "### {}. {} ({})\n\n",
                    i + 1,
                    failure.test_id,
                    failure.model
                ));
                s.push_str(&format!("- **Error:** {:?}\n", failure.error_category));
                if let Some(ref msg) = failure.error_message {
                    s.push_str(&format!("- **Message:** {}\n", msg));
                }
                s.push_str(&format!(
                    "- **Generated:**\n```\n{}\n```\n\n",
                    failure.generated_ucl
                ));
            }
            if failures.len() > 10 {
                s.push_str(&format!(
                    "... and {} more failures\n\n",
                    failures.len() - 10
                ));
            }
        }

        s
    }

    /// Print summary to stdout
    pub fn print_summary(&self) {
        println!("{}", self.summary());
    }

    /// Write summary to file
    pub fn save_summary(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.summary())
    }

    /// Generate CSV of results
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("test_id,command_type,model,provider,latency_ms,input_tokens,output_tokens,cost_usd,parse_success,execute_success,semantic_score,error_category\n");

        for r in &self.results {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{:.6},{},{},{:.2},{}\n",
                r.test_id,
                r.command_type,
                r.model,
                r.provider,
                r.latency_ms,
                r.input_tokens,
                r.output_tokens,
                r.cost_usd,
                r.parse_success,
                r.execute_success,
                r.semantic_score,
                r.error_category
                    .as_ref()
                    .map(|c| format!("{:?}", c))
                    .unwrap_or_default(),
            ));
        }

        csv
    }

    /// Save CSV to file
    pub fn save_csv(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_csv())
    }

    /// Print a live progress update
    pub fn print_progress<W: Write>(
        writer: &mut W,
        current: usize,
        total: usize,
        result: &TestResult,
    ) {
        let status = if result.is_success() { "✓" } else { "✗" };
        let _ = writeln!(
            writer,
            "[{}/{}] {} {} ({}) - {}ms ${:.4}",
            current,
            total,
            status,
            result.test_id,
            result.model,
            result.latency_ms,
            result.cost_usd
        );
    }
}

/// Comparison between two benchmark runs
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub baseline_id: String,
    pub current_id: String,
    pub success_rate_delta: f64,
    pub latency_delta_ms: i64,
    pub cost_delta_usd: f64,
    pub regressions: Vec<String>,
    pub improvements: Vec<String>,
}

impl BenchmarkComparison {
    pub fn compare(baseline: &BenchmarkReport, current: &BenchmarkReport) -> Self {
        let success_rate_delta = current.metrics.success_rate() - baseline.metrics.success_rate();
        let latency_delta_ms =
            current.metrics.avg_latency_ms() as i64 - baseline.metrics.avg_latency_ms() as i64;
        let cost_delta_usd = current.metrics.total_cost_usd - baseline.metrics.total_cost_usd;

        let mut regressions = Vec::new();
        let mut improvements = Vec::new();

        // Compare by model
        for (model, current_metrics) in &current.metrics.by_model {
            if let Some(baseline_metrics) = baseline.metrics.by_model.get(model) {
                let rate_delta = current_metrics.success_rate() - baseline_metrics.success_rate();
                if rate_delta < -0.05 {
                    regressions.push(format!(
                        "{}: success rate dropped {:.1}%",
                        model,
                        rate_delta * 100.0
                    ));
                } else if rate_delta > 0.05 {
                    improvements.push(format!(
                        "{}: success rate improved {:.1}%",
                        model,
                        rate_delta * 100.0
                    ));
                }
            }
        }

        Self {
            baseline_id: baseline.benchmark_id.clone(),
            current_id: current.benchmark_id.clone(),
            success_rate_delta,
            latency_delta_ms,
            cost_delta_usd,
            regressions,
            improvements,
        }
    }

    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str("# Benchmark Comparison\n\n");
        s.push_str(&format!("Baseline: {}\n", self.baseline_id));
        s.push_str(&format!("Current: {}\n\n", self.current_id));

        let success_icon = if self.success_rate_delta >= 0.0 {
            "📈"
        } else {
            "📉"
        };
        let latency_icon = if self.latency_delta_ms <= 0 {
            "⚡"
        } else {
            "🐢"
        };
        let cost_icon = if self.cost_delta_usd <= 0.0 {
            "💰"
        } else {
            "💸"
        };

        s.push_str(&format!(
            "{} Success Rate: {:+.1}%\n",
            success_icon,
            self.success_rate_delta * 100.0
        ));
        s.push_str(&format!(
            "{} Latency: {:+}ms\n",
            latency_icon, self.latency_delta_ms
        ));
        s.push_str(&format!(
            "{} Cost: ${:+.4}\n\n",
            cost_icon, self.cost_delta_usd
        ));

        if !self.regressions.is_empty() {
            s.push_str("## Regressions ⚠️\n");
            for r in &self.regressions {
                s.push_str(&format!("- {}\n", r));
            }
            s.push('\n');
        }

        if !self.improvements.is_empty() {
            s.push_str("## Improvements ✅\n");
            for i in &self.improvements {
                s.push_str(&format!("- {}\n", i));
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::BenchmarkMetrics;

    #[test]
    fn test_report_summary() {
        let report = BenchmarkReport {
            benchmark_id: "test_001".into(),
            name: "test".into(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 60,
            models_tested: vec!["openai/gpt-4o".into()],
            metrics: BenchmarkMetrics::new(),
            results: Vec::new(),
        };

        let summary = report.summary();
        assert!(summary.contains("Benchmark Report"));
        assert!(summary.contains("test_001"));
    }

    #[test]
    fn test_report_csv() {
        let mut report = BenchmarkReport {
            benchmark_id: "test_001".into(),
            name: "test".into(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 60,
            models_tested: vec![],
            metrics: BenchmarkMetrics::new(),
            results: vec![TestResult::success("t1", "EDIT", "gpt-4o", "openai")],
        };

        let csv = report.to_csv();
        assert!(csv.contains("test_id,command_type"));
        assert!(csv.contains("t1,EDIT,gpt-4o"));
    }
}
