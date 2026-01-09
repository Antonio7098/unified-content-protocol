//! Benchmark report persistence.

use std::fs;
use std::path::{Path, PathBuf};

use crate::report::BenchmarkReport;
use crate::suite::SuiteRunResult;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

const DEFAULT_STORAGE_ENV: &str = "UCP_BENCH_STORAGE_DIR";
const DEFAULT_STORAGE_DIR: &str = "benchmarks/history";
const SUITE_RUN_SUBDIR: &str = "suite_runs";

/// Metadata describing a stored benchmark run.
#[derive(Debug, Clone)]
pub struct StoredBenchmarkMetadata {
    /// Logical benchmark identifier (e.g., `bench_20250107_134501`).
    pub benchmark_id: String,
    /// Human-readable benchmark name.
    pub name: String,
    /// When the artifact was written to disk.
    pub saved_at: DateTime<Utc>,
    /// Duration of the run in seconds.
    pub duration_seconds: u64,
    /// Aggregate totals taken from the metrics block.
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
}

/// Result of persisting a benchmark report.
#[derive(Debug, Clone)]
pub struct StoredBenchmark {
    pub metadata: StoredBenchmarkMetadata,
    pub json_path: PathBuf,
    pub summary_path: PathBuf,
}

/// Storage interface for benchmark reports.
pub trait BenchmarkStorage {
    fn save(&self, report: &BenchmarkReport) -> Result<StoredBenchmark>;
}

/// File-system-backed benchmark storage.
#[derive(Debug, Clone)]
pub struct FileBenchmarkStorage {
    root: PathBuf,
}

impl FileBenchmarkStorage {
    /// Create a new storage rooted at the provided directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Directory where artifacts are persisted.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn ensure_root_exists(&self) -> Result<()> {
        if !self.root.exists() {
            fs::create_dir_all(&self.root)
                .context("failed to create benchmark storage directory")?;
        }
        Ok(())
    }

    fn file_stem(report: &BenchmarkReport) -> String {
        let slug = slugify(&report.name);
        format!("{}_{}", report.benchmark_id, slug)
    }
}

impl BenchmarkStorage for FileBenchmarkStorage {
    fn save(&self, report: &BenchmarkReport) -> Result<StoredBenchmark> {
        self.ensure_root_exists()?;

        let file_stem = Self::file_stem(report);
        let json_path = self.root.join(format!("{file_stem}.json"));
        let summary_path = self.root.join(format!("{file_stem}.md"));

        let json = report
            .to_json()
            .context("failed to serialize benchmark report to JSON")?;
        fs::write(&json_path, json).context("failed to write benchmark JSON artifact")?;
        fs::write(&summary_path, report.summary())
            .context("failed to write benchmark summary artifact")?;

        let metadata = StoredBenchmarkMetadata {
            benchmark_id: report.benchmark_id.clone(),
            name: report.name.clone(),
            saved_at: Utc::now(),
            duration_seconds: report.duration_seconds,
            total_tests: report.metrics.total_tests,
            passed: report.metrics.passed,
            failed: report.metrics.failed,
        };

        Ok(StoredBenchmark {
            metadata,
            json_path,
            summary_path,
        })
    }
}

/// Resolve the root directory used for benchmark storage.
///
/// Users can override the location by exporting `UCP_BENCH_STORAGE_DIR`.
pub fn default_storage_root() -> PathBuf {
    std::env::var(DEFAULT_STORAGE_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_STORAGE_DIR))
}

/// Construct the default file-system storage implementation.
pub fn default_storage() -> FileBenchmarkStorage {
    FileBenchmarkStorage::new(default_storage_root())
}

/// Persistent storage for suite run results used by the UI/API.
#[derive(Debug, Clone)]
pub struct SuiteRunStorage {
    root: PathBuf,
}

impl SuiteRunStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn runs_dir(&self) -> PathBuf {
        self.root.join(SUITE_RUN_SUBDIR)
    }

    fn ensure_runs_dir(&self) -> Result<()> {
        if !self.runs_dir().exists() {
            fs::create_dir_all(self.runs_dir())
                .context("failed to create suite run storage directory")?;
        }
        Ok(())
    }

    pub fn save_run(&self, run: &SuiteRunResult) -> Result<PathBuf> {
        self.ensure_runs_dir()?;
        let path = self.runs_dir().join(format!("{}.json", run.run_id));
        let contents =
            serde_json::to_vec_pretty(run).context("failed to serialize suite run to json")?;
        fs::write(&path, contents).context("failed to write suite run artifact")?;
        Ok(path)
    }

    pub fn load_runs(&self) -> Result<Vec<SuiteRunResult>> {
        self.ensure_runs_dir()?;
        let mut runs = Vec::new();
        for entry in fs::read_dir(self.runs_dir()).context("failed to read suite run directory")? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            match fs::read(&path)
                .map_err(anyhow::Error::from)
                .and_then(|bytes| {
                    serde_json::from_slice::<SuiteRunResult>(&bytes)
                        .context("failed to parse suite run json")
                }) {
                Ok(run) => runs.push(run),
                Err(err) => {
                    tracing::warn!("Skipping invalid suite run file {:?}: {}", path, err);
                }
            }
        }

        runs.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        Ok(runs)
    }
}

impl Default for SuiteRunStorage {
    fn default() -> Self {
        default_suite_run_storage()
    }
}

pub fn default_suite_run_storage() -> SuiteRunStorage {
    SuiteRunStorage::new(default_storage_root())
}

// =============================================================================
// Core Benchmark Storage
// =============================================================================

use crate::core::metrics::CoreBenchmarkReport;

const CORE_BENCHMARK_SUBDIR: &str = "core_benchmarks";

/// Persistent storage for core benchmark reports.
#[derive(Debug, Clone)]
pub struct CoreBenchmarkStorage {
    root: PathBuf,
}

impl CoreBenchmarkStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn benchmarks_dir(&self) -> PathBuf {
        self.root.join(CORE_BENCHMARK_SUBDIR)
    }

    fn ensure_dir(&self) -> Result<()> {
        let dir = self.benchmarks_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .context("failed to create core benchmark storage directory")?;
        }
        Ok(())
    }

    pub fn save(&self, report: &CoreBenchmarkReport) -> Result<PathBuf> {
        self.ensure_dir()?;
        let filename = format!("{}.json", report.report_id);
        let path = self.benchmarks_dir().join(&filename);
        let contents = serde_json::to_vec_pretty(report)
            .context("failed to serialize core benchmark report")?;
        fs::write(&path, contents).context("failed to write core benchmark report")?;

        // Also write summary markdown
        let summary_path = self
            .benchmarks_dir()
            .join(format!("{}.md", report.report_id));
        fs::write(&summary_path, report.summary())
            .context("failed to write core benchmark summary")?;

        Ok(path)
    }

    pub fn load(&self, report_id: &str) -> Result<CoreBenchmarkReport> {
        self.ensure_dir()?;
        let path = self.benchmarks_dir().join(format!("{}.json", report_id));
        let contents = fs::read(&path).context("failed to read core benchmark report")?;
        serde_json::from_slice(&contents).context("failed to parse core benchmark report")
    }

    pub fn load_all(&self) -> Result<Vec<CoreBenchmarkReport>> {
        self.ensure_dir()?;
        let mut reports = Vec::new();

        for entry in fs::read_dir(self.benchmarks_dir())
            .context("failed to read core benchmark directory")?
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            match fs::read(&path)
                .map_err(anyhow::Error::from)
                .and_then(|bytes| {
                    serde_json::from_slice::<CoreBenchmarkReport>(&bytes)
                        .context("failed to parse core benchmark report")
                }) {
                Ok(report) => reports.push(report),
                Err(err) => {
                    tracing::warn!("Skipping invalid core benchmark file {:?}: {}", path, err);
                }
            }
        }

        reports.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        Ok(reports)
    }

    pub fn list_summaries(&self) -> Result<Vec<CoreBenchmarkSummary>> {
        let reports = self.load_all()?;
        Ok(reports
            .into_iter()
            .map(CoreBenchmarkSummary::from)
            .collect())
    }

    pub fn delete(&self, report_id: &str) -> Result<()> {
        self.ensure_dir()?;
        let json_path = self.benchmarks_dir().join(format!("{}.json", report_id));
        let md_path = self.benchmarks_dir().join(format!("{}.md", report_id));

        if json_path.exists() {
            fs::remove_file(&json_path).context("failed to delete core benchmark report")?;
        }
        if md_path.exists() {
            fs::remove_file(&md_path).context("failed to delete core benchmark summary")?;
        }

        Ok(())
    }
}

impl Default for CoreBenchmarkStorage {
    fn default() -> Self {
        default_core_benchmark_storage()
    }
}

pub fn default_core_benchmark_storage() -> CoreBenchmarkStorage {
    CoreBenchmarkStorage::new(default_storage_root())
}

/// Summary of a stored core benchmark for listing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreBenchmarkSummary {
    pub report_id: String,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub success_rate: f32,
    pub avg_duration_ms: f64,
    pub categories: Vec<String>,
}

impl From<CoreBenchmarkReport> for CoreBenchmarkSummary {
    fn from(report: CoreBenchmarkReport) -> Self {
        Self {
            report_id: report.report_id,
            name: report.name,
            started_at: report.started_at,
            completed_at: report.completed_at,
            total_tests: report.metrics.total_tests,
            passed: report.metrics.passed,
            failed: report.metrics.failed,
            success_rate: report.metrics.success_rate,
            avg_duration_ms: report.metrics.avg_duration_ms,
            categories: report.config.categories,
        }
    }
}

fn slugify(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }

    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "benchmark".into()
    } else {
        slug.to_string()
    }
}
