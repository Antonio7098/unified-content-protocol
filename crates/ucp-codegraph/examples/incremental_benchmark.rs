use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use serde::Serialize;
use ucp_codegraph::{
    build_code_graph, build_code_graph_incremental, CodeGraphBuildInput, CodeGraphExtractorConfig,
    CodeGraphIncrementalBuildInput, CodeGraphIncrementalStats,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Serialize)]
struct BenchmarkStep {
    label: String,
    elapsed_ms: u128,
    incremental: Option<CodeGraphIncrementalStats>,
    canonical_fingerprint: String,
    matches_full_build: Option<bool>,
}

#[derive(Serialize)]
struct BenchmarkSummary {
    repo_path: String,
    consumers: usize,
    steps: Vec<BenchmarkStep>,
}

fn main() -> Result<()> {
    let (consumers, format) = parse_args()?;
    let repo_root = create_repo_root();
    let state_file = repo_root.join("codegraph-state.json");
    write_fixture(&repo_root, consumers)?;

    let full_input = build_input(&repo_root, "benchmark-shared");
    let incremental_input = incremental_input(&repo_root, &state_file, "benchmark-shared");

    let (full_elapsed, full) = timed(|| build_code_graph(&full_input))?;
    let (seed_elapsed, seed) = timed(|| build_code_graph_incremental(&incremental_input))?;
    let (no_change_elapsed, no_change) =
        timed(|| build_code_graph_incremental(&incremental_input))?;

    fs::write(
        repo_root.join("src/shared.rs"),
        "pub fn shared() -> i32 { 2 }\n",
    )?;
    let (body_elapsed, body_changed) = timed(|| build_code_graph_incremental(&incremental_input))?;
    let (_, body_full) = timed(|| build_code_graph(&full_input))?;

    fs::write(
        repo_root.join("src/shared.rs"),
        "pub fn shared() -> i32 { 2 }\npub fn shared_extra() -> i32 { shared() * 2 }\n",
    )?;
    let (api_elapsed, api_changed) = timed(|| build_code_graph_incremental(&incremental_input))?;
    let (_, api_full) = timed(|| build_code_graph(&full_input))?;
    let full_fingerprint = full.canonical_fingerprint.clone();

    let summary = BenchmarkSummary {
        repo_path: repo_root.display().to_string(),
        consumers,
        steps: vec![
            BenchmarkStep {
                label: "full".to_string(),
                elapsed_ms: full_elapsed.as_millis(),
                incremental: None,
                canonical_fingerprint: full_fingerprint.clone(),
                matches_full_build: None,
            },
            BenchmarkStep {
                label: "seed_incremental".to_string(),
                elapsed_ms: seed_elapsed.as_millis(),
                incremental: seed.incremental,
                canonical_fingerprint: seed.canonical_fingerprint.clone(),
                matches_full_build: Some(seed.canonical_fingerprint == full_fingerprint),
            },
            BenchmarkStep {
                label: "no_change_incremental".to_string(),
                elapsed_ms: no_change_elapsed.as_millis(),
                incremental: no_change.incremental,
                canonical_fingerprint: no_change.canonical_fingerprint.clone(),
                matches_full_build: Some(no_change.canonical_fingerprint == full_fingerprint),
            },
            BenchmarkStep {
                label: "body_change_incremental".to_string(),
                elapsed_ms: body_elapsed.as_millis(),
                incremental: body_changed.incremental,
                canonical_fingerprint: body_changed.canonical_fingerprint.clone(),
                matches_full_build: Some(
                    body_changed.canonical_fingerprint == body_full.canonical_fingerprint,
                ),
            },
            BenchmarkStep {
                label: "api_change_incremental".to_string(),
                elapsed_ms: api_elapsed.as_millis(),
                incremental: api_changed.incremental,
                canonical_fingerprint: api_changed.canonical_fingerprint.clone(),
                matches_full_build: Some(
                    api_changed.canonical_fingerprint == api_full.canonical_fingerprint,
                ),
            },
        ],
    };

    match format {
        OutputFormat::Text => print_text_summary(&summary),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&summary)?),
    }

    Ok(())
}

fn parse_args() -> Result<(usize, OutputFormat)> {
    let mut consumers = 200usize;
    let mut format = OutputFormat::Text;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--consumers" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing value for --consumers"))?;
                consumers = value.parse()?;
            }
            "--format" => {
                format = match args.next().as_deref() {
                    Some("text") => OutputFormat::Text,
                    Some("json") => OutputFormat::Json,
                    Some(other) => bail!("unsupported format: {other}"),
                    None => bail!("missing value for --format"),
                };
            }
            other => bail!("unknown argument: {other}"),
        }
    }
    Ok((consumers, format))
}

fn create_repo_root() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    env::temp_dir().join(format!(
        "ucp-codegraph-benchmark-{}-{suffix}",
        process::id()
    ))
}

fn write_fixture(repo_root: &Path, consumers: usize) -> Result<()> {
    let src = repo_root.join("src");
    fs::create_dir_all(&src)?;
    fs::write(src.join("shared.rs"), "pub fn shared() -> i32 { 1 }\n")?;

    let mut lib_rs = String::from("pub mod shared;\n");
    for i in 0..consumers {
        let module = format!("consumer_{i}");
        lib_rs.push_str(&format!("pub mod {module};\n"));
        fs::write(
            src.join(format!("{module}.rs")),
            format!("use crate::shared::shared;\npub fn {module}() -> i32 {{ shared() + {i} }}\n"),
        )?;
    }
    fs::write(src.join("lib.rs"), lib_rs)?;
    Ok(())
}

fn build_input(repo_root: &Path, commit_hash: &str) -> CodeGraphBuildInput {
    CodeGraphBuildInput {
        repository_path: repo_root.to_path_buf(),
        commit_hash: commit_hash.to_string(),
        config: CodeGraphExtractorConfig::default(),
    }
}

fn incremental_input(
    repo_root: &Path,
    state_file: &Path,
    commit_hash: &str,
) -> CodeGraphIncrementalBuildInput {
    CodeGraphIncrementalBuildInput {
        build: build_input(repo_root, commit_hash),
        state_file: state_file.to_path_buf(),
    }
}

fn timed<T>(f: impl FnOnce() -> Result<T>) -> Result<(std::time::Duration, T)> {
    let start = Instant::now();
    let value = f()?;
    Ok((start.elapsed(), value))
}

fn print_text_summary(summary: &BenchmarkSummary) {
    println!("Incremental benchmark repo: {}", summary.repo_path);
    println!("consumers: {}", summary.consumers);
    for step in &summary.steps {
        println!("- {}: {} ms", step.label, step.elapsed_ms);
        if let Some(incremental) = &step.incremental {
            println!(
                "  rebuilt={} reused={} direct_invalidations={} surface_changes={} invalidated={}",
                incremental.rebuilt_files,
                incremental.reused_files,
                incremental.direct_invalidated_files,
                incremental.surface_changed_files,
                incremental.invalidated_files
            );
        }
        if let Some(matches) = step.matches_full_build {
            println!("  matches_full_build={matches}");
        }
    }
}
