use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::model::{CodeLanguage, ExtractedInput, ExtractedModifiers, FileAnalysis, ImportBinding};
use crate::{
    CodeGraphBuildResult, CodeGraphDiagnostic, CodeGraphExtractorConfig,
    CodeGraphIncrementalBuildInput, CodeGraphIncrementalStats, CODEGRAPH_EXTRACTOR_VERSION,
};

use super::build::{
    analyze_loaded_repo_file, assemble_code_graph_from_analyzed_files, load_repo_file,
    AnalyzedRepoFile,
};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IncrementalBuildState {
    extractor_version: String,
    repository_path: String,
    config: CodeGraphExtractorConfig,
    files: BTreeMap<String, IncrementalFileState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IncrementalFileState {
    relative_path: String,
    language: CodeLanguage,
    content_hash: String,
    #[serde(default)]
    surface_signature: String,
    analysis: Option<FileAnalysis>,
    diagnostics: Vec<CodeGraphDiagnostic>,
    dependencies: Vec<String>,
}

impl IncrementalFileState {
    fn to_analyzed_repo_file(&self) -> AnalyzedRepoFile {
        AnalyzedRepoFile {
            relative_path: self.relative_path.clone(),
            language: self.language,
            content_hash: Some(self.content_hash.clone()),
            analysis: self.analysis.clone(),
            diagnostics: self.diagnostics.clone(),
        }
    }

    fn from_analyzed_repo_file(file: &AnalyzedRepoFile, dependencies: Vec<String>) -> Option<Self> {
        Some(Self {
            relative_path: file.relative_path.clone(),
            language: file.language,
            content_hash: file.content_hash.clone()?,
            surface_signature: compute_file_surface_signature(file.analysis.as_ref()),
            analysis: file.analysis.clone(),
            diagnostics: file.diagnostics.clone(),
            dependencies,
        })
    }
}

pub fn build_code_graph_incremental(
    input: &CodeGraphIncrementalBuildInput,
) -> Result<CodeGraphBuildResult> {
    let repo_root = input
        .build
        .repository_path
        .canonicalize()
        .with_context(|| {
            format!(
                "failed to resolve repository path {}",
                input.build.repository_path.display()
            )
        })?;
    if !repo_root.is_dir() {
        anyhow::bail!(
            "repository path is not a directory: {}",
            repo_root.display()
        );
    }

    let repo_name = repo_root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repository".to_string());

    let normalized_config = normalize_incremental_config(&input.build.config);
    let normalized_repo_path = normalize_path(&repo_root);
    let mut diagnostics = Vec::new();
    let matcher = GitignoreMatcher::from_repository(&repo_root)?;
    let repo_files =
        collect_repository_files(&repo_root, &normalized_config, &matcher, &mut diagnostics)?;

    let state_status = load_compatible_state(
        &input.state_file,
        &normalized_repo_path,
        &normalized_config,
        &mut diagnostics,
    )?;

    let loaded_files = repo_files
        .iter()
        .map(|repo_file| load_repo_file(repo_file, &normalized_config))
        .collect::<Result<Vec<_>>>()?;

    let previous_state = state_status.state.as_ref();
    let state_entries = previous_state.map(|state| state.files.len()).unwrap_or(0);
    let current_paths: BTreeSet<String> = loaded_files
        .iter()
        .map(|loaded| loaded.repo_file.relative_path.clone())
        .collect();
    let deleted_paths: BTreeSet<String> = previous_state
        .map(|state| {
            state
                .files
                .keys()
                .filter(|path| !current_paths.contains(*path))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    let (
        analyzed_files,
        added_files,
        changed_files,
        direct_invalidated_files,
        surface_changed_files,
        rebuilt_files,
        reused_files,
        invalidated_files,
    ) = if let Some(state) = previous_state {
        let mut added_files = 0usize;
        let mut changed_files = 0usize;
        let mut direct_rebuild_paths = BTreeSet::new();

        for loaded in &loaded_files {
            let path = &loaded.repo_file.relative_path;
            match state.files.get(path) {
                None => {
                    direct_rebuild_paths.insert(path.clone());
                    added_files += 1;
                }
                Some(previous) => {
                    if loaded
                        .content_hash
                        .as_ref()
                        .map(|hash| hash != &previous.content_hash)
                        .unwrap_or(true)
                    {
                        direct_rebuild_paths.insert(path.clone());
                        changed_files += 1;
                    }
                }
            }
        }

        let mut pre_analyzed = BTreeMap::new();
        let mut surface_change_roots = deleted_paths.clone();
        let mut surface_changed_files = deleted_paths.len();

        for loaded in loaded_files
            .iter()
            .filter(|loaded| direct_rebuild_paths.contains(&loaded.repo_file.relative_path))
        {
            let analyzed = analyze_loaded_repo_file(loaded.clone());
            let current_surface = compute_file_surface_signature(analyzed.analysis.as_ref());
            let previous_surface = state
                .files
                .get(&loaded.repo_file.relative_path)
                .map(|entry| entry.surface_signature.as_str())
                .unwrap_or("");
            if current_surface != previous_surface {
                surface_change_roots.insert(loaded.repo_file.relative_path.clone());
                surface_changed_files += 1;
            }
            pre_analyzed.insert(loaded.repo_file.relative_path.clone(), analyzed);
        }

        let expanded_invalidations = expand_invalidations(&surface_change_roots, state);
        let rebuild_paths = direct_rebuild_paths
            .union(&expanded_invalidations)
            .cloned()
            .collect::<BTreeSet<_>>();
        let counted_rebuild_paths = rebuild_paths
            .iter()
            .filter(|path| current_paths.contains(*path))
            .count();

        let mut reused_files = 0usize;
        let mut rebuilt_files = 0usize;
        let analyzed_files = loaded_files
            .into_iter()
            .map(|loaded| {
                let path = loaded.repo_file.relative_path.clone();
                if let Some(analyzed) = pre_analyzed.remove(&path) {
                    rebuilt_files += 1;
                    return analyzed;
                }
                if !rebuild_paths.contains(&path) {
                    if let (Some(content_hash), Some(previous)) =
                        (loaded.content_hash.as_ref(), state.files.get(&path))
                    {
                        if content_hash == &previous.content_hash {
                            reused_files += 1;
                            return previous.to_analyzed_repo_file();
                        }
                    }
                }

                rebuilt_files += 1;
                analyze_loaded_repo_file(loaded)
            })
            .collect::<Vec<_>>();

        (
            analyzed_files,
            added_files,
            changed_files,
            direct_rebuild_paths.len() + deleted_paths.len(),
            surface_changed_files,
            rebuilt_files,
            reused_files,
            counted_rebuild_paths + deleted_paths.len(),
        )
    } else {
        let rebuilt_files = loaded_files.len();
        let analyzed_files = loaded_files
            .into_iter()
            .map(analyze_loaded_repo_file)
            .collect::<Vec<_>>();
        (
            analyzed_files,
            0,
            0,
            current_paths.len(),
            0,
            rebuilt_files,
            0,
            current_paths.len(),
        )
    };

    let assembled = assemble_code_graph_from_analyzed_files(
        &repo_root,
        &repo_name,
        &input.build.commit_hash,
        &normalized_config,
        &analyzed_files,
        diagnostics,
    )?;

    write_state(
        &input.state_file,
        &normalized_repo_path,
        &normalized_config,
        &analyzed_files,
        &assembled.dependencies_by_file,
    )?;

    let mut result = assembled.result;
    result.incremental = Some(CodeGraphIncrementalStats {
        requested: true,
        scanned_files: repo_files.len(),
        state_entries,
        direct_invalidated_files,
        surface_changed_files,
        reused_files,
        rebuilt_files,
        added_files,
        changed_files,
        deleted_files: deleted_paths.len(),
        invalidated_files,
        full_rebuild_reason: state_status.full_rebuild_reason,
    });
    Ok(result)
}

#[derive(Debug)]
struct StateLoadStatus {
    state: Option<IncrementalBuildState>,
    full_rebuild_reason: Option<String>,
}

fn load_compatible_state(
    state_file: &Path,
    normalized_repo_path: &str,
    normalized_config: &CodeGraphExtractorConfig,
    diagnostics: &mut Vec<CodeGraphDiagnostic>,
) -> Result<StateLoadStatus> {
    let contents = match fs::read_to_string(state_file) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(StateLoadStatus {
                state: None,
                full_rebuild_reason: Some("missing_state".to_string()),
            });
        }
        Err(err) => {
            diagnostics.push(CodeGraphDiagnostic::warning(
                "CG2009",
                format!(
                    "incremental state unreadable; falling back to full rebuild: {}",
                    err
                ),
            ));
            return Ok(StateLoadStatus {
                state: None,
                full_rebuild_reason: Some("unreadable_state".to_string()),
            });
        }
    };

    let state: IncrementalBuildState = match serde_json::from_str(&contents) {
        Ok(state) => state,
        Err(err) => {
            diagnostics.push(CodeGraphDiagnostic::warning(
                "CG2009",
                format!(
                    "incremental state invalid; falling back to full rebuild: {}",
                    err
                ),
            ));
            return Ok(StateLoadStatus {
                state: None,
                full_rebuild_reason: Some("invalid_state".to_string()),
            });
        }
    };

    if state.extractor_version != CODEGRAPH_EXTRACTOR_VERSION {
        return Ok(StateLoadStatus {
            state: None,
            full_rebuild_reason: Some("extractor_version_changed".to_string()),
        });
    }
    if state.repository_path != normalized_repo_path {
        return Ok(StateLoadStatus {
            state: None,
            full_rebuild_reason: Some("repository_changed".to_string()),
        });
    }
    if state.config != *normalized_config {
        return Ok(StateLoadStatus {
            state: None,
            full_rebuild_reason: Some("config_changed".to_string()),
        });
    }

    Ok(StateLoadStatus {
        state: Some(state),
        full_rebuild_reason: None,
    })
}

fn write_state(
    state_file: &Path,
    normalized_repo_path: &str,
    normalized_config: &CodeGraphExtractorConfig,
    analyzed_files: &[AnalyzedRepoFile],
    dependencies_by_file: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    let mut files = BTreeMap::new();
    for file in analyzed_files {
        let dependencies = dependencies_by_file
            .get(&file.relative_path)
            .cloned()
            .unwrap_or_default();
        if let Some(state) = IncrementalFileState::from_analyzed_repo_file(file, dependencies) {
            files.insert(file.relative_path.clone(), state);
        }
    }

    let state = IncrementalBuildState {
        extractor_version: CODEGRAPH_EXTRACTOR_VERSION.to_string(),
        repository_path: normalized_repo_path.to_string(),
        config: normalized_config.clone(),
        files,
    };
    if let Some(parent) = state_file.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create incremental state directory {}",
                parent.display()
            )
        })?;
    }
    let json = serde_json::to_string_pretty(&state)?;
    fs::write(state_file, json).with_context(|| {
        format!(
            "failed to write incremental state file {}",
            state_file.display()
        )
    })?;
    Ok(())
}

fn expand_invalidations(
    initial_invalidations: &BTreeSet<String>,
    state: &IncrementalBuildState,
) -> BTreeSet<String> {
    let mut reverse_dependencies: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (file, entry) in &state.files {
        for dependency in &entry.dependencies {
            reverse_dependencies
                .entry(dependency.clone())
                .or_default()
                .insert(file.clone());
        }
    }

    let mut invalidated = BTreeSet::new();
    let mut queue: VecDeque<String> = initial_invalidations.iter().cloned().collect();
    while let Some(path) = queue.pop_front() {
        if !invalidated.insert(path.clone()) {
            continue;
        }
        if let Some(dependents) = reverse_dependencies.get(&path) {
            queue.extend(dependents.iter().cloned());
        }
    }
    invalidated
}

fn normalize_incremental_config(config: &CodeGraphExtractorConfig) -> CodeGraphExtractorConfig {
    let mut normalized = config.clone();
    normalized.include_extensions.sort();
    normalized.include_extensions.dedup();
    normalized.exclude_dirs.sort();
    normalized.exclude_dirs.dedup();
    normalized
}

#[derive(Serialize)]
struct SurfaceSignatureSymbol {
    name: String,
    qualified_name: String,
    parent_identity: Option<String>,
    kind: String,
    modifiers: ExtractedModifiers,
    inputs: Vec<ExtractedInput>,
    output: Option<String>,
    type_info: Option<String>,
    exported: bool,
}

#[derive(Serialize)]
struct SurfaceSignatureReexport {
    module: String,
    symbols: Vec<String>,
    bindings: Vec<ImportBinding>,
    wildcard: bool,
}

#[derive(Serialize)]
struct SurfaceSignatureSnapshot {
    symbols: Vec<SurfaceSignatureSymbol>,
    export_bindings: Vec<ImportBinding>,
    exported_symbol_names: Vec<String>,
    default_exported_symbol_names: Vec<String>,
    reexports: Vec<SurfaceSignatureReexport>,
}

fn compute_file_surface_signature(analysis: Option<&FileAnalysis>) -> String {
    let Some(analysis) = analysis else {
        return String::new();
    };

    let mut symbols = analysis
        .symbols
        .iter()
        .map(|symbol| SurfaceSignatureSymbol {
            name: symbol.name.clone(),
            qualified_name: symbol.qualified_name.clone(),
            parent_identity: symbol.parent_identity.clone(),
            kind: symbol.kind.clone(),
            modifiers: symbol.modifiers.clone(),
            inputs: symbol.inputs.clone(),
            output: symbol.output.clone(),
            type_info: symbol.type_info.clone(),
            exported: symbol.exported,
        })
        .collect::<Vec<_>>();
    symbols.sort_by(|left, right| {
        left.qualified_name
            .cmp(&right.qualified_name)
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.parent_identity.cmp(&right.parent_identity))
            .then_with(|| left.name.cmp(&right.name))
    });

    let mut export_bindings = analysis.export_bindings.clone();
    export_bindings.sort();

    let mut exported_symbol_names = analysis
        .exported_symbol_names
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    exported_symbol_names.sort();

    let mut default_exported_symbol_names = analysis
        .default_exported_symbol_names
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    default_exported_symbol_names.sort();

    let mut reexports = analysis
        .imports
        .iter()
        .filter(|import| import.reexported)
        .map(|import| {
            let mut symbols = import.symbols.clone();
            symbols.sort();
            let mut bindings = import.bindings.clone();
            bindings.sort();
            SurfaceSignatureReexport {
                module: import.module.clone(),
                symbols,
                bindings,
                wildcard: import.wildcard,
            }
        })
        .collect::<Vec<_>>();
    reexports.sort_by(|left, right| {
        left.module
            .cmp(&right.module)
            .then_with(|| left.wildcard.cmp(&right.wildcard))
            .then_with(|| left.symbols.cmp(&right.symbols))
            .then_with(|| left.bindings.cmp(&right.bindings))
    });

    let snapshot = SurfaceSignatureSnapshot {
        symbols,
        export_bindings,
        exported_symbol_names,
        default_exported_symbol_names,
        reexports,
    };
    let serialized = serde_json::to_string(&snapshot).expect("surface signature serialization");
    super::build::hash_source(&serialized)
}
