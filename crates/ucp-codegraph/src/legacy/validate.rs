use std::collections::HashMap;

use ucm_core::{BlockId, Document, EdgeType};

use crate::model::*;

use super::canonical::validate_required_metadata;
use super::{block_logical_key, block_path, logical_key_index, node_class};

pub fn validate_code_graph_profile(doc: &Document) -> CodeGraphValidationResult {
    let mut diagnostics = Vec::new();

    match doc.metadata.custom.get("profile").and_then(|v| v.as_str()) {
        Some(CODEGRAPH_PROFILE) => {}
        Some(other) => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1001",
            format!(
                "invalid profile marker '{}', expected '{}'",
                other, CODEGRAPH_PROFILE
            ),
        )),
        None => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1001",
            "missing document metadata.custom.profile marker",
        )),
    }

    match doc
        .metadata
        .custom
        .get("profile_version")
        .and_then(|v| v.as_str())
    {
        Some(CODEGRAPH_PROFILE_VERSION) => {}
        Some(other) => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1002",
            format!(
                "invalid profile version '{}', expected '{}'",
                other, CODEGRAPH_PROFILE_VERSION
            ),
        )),
        None => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1002",
            "missing document metadata.custom.profile_version marker",
        )),
    }

    let mut logical_keys: HashMap<String, Vec<BlockId>> = HashMap::new();
    let mut class_counts: HashMap<String, usize> = HashMap::new();

    for (id, block) in &doc.blocks {
        if *id == doc.root {
            continue;
        }

        let class = node_class(block);
        let Some(class_name) = class else {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1010",
                    "block missing node_class metadata (or custom semantic role)",
                )
                .with_path(block_path(block).unwrap_or_else(|| id.to_string())),
            );
            continue;
        };

        *class_counts.entry(class_name.clone()).or_default() += 1;

        match block_logical_key(block) {
            Some(logical_key) => {
                logical_keys.entry(logical_key).or_default().push(*id);
            }
            None => diagnostics.push(
                CodeGraphDiagnostic::error("CG1011", "missing required logical_key metadata")
                    .with_path(block_path(block).unwrap_or_else(|| id.to_string())),
            ),
        }

        validate_required_metadata(&class_name, block, &mut diagnostics);
    }

    for class in ["repository", "directory", "file", "symbol"] {
        if class_counts.get(class).copied().unwrap_or(0) == 0 {
            diagnostics.push(CodeGraphDiagnostic::warning(
                "CG1012",
                format!("profile has no '{}' nodes", class),
            ));
        }
    }

    for (logical_key, ids) in logical_keys {
        if ids.len() > 1 {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1013",
                    format!(
                        "logical_key '{}' is duplicated by {} blocks",
                        logical_key,
                        ids.len()
                    ),
                )
                .with_logical_key(logical_key),
            );
        }
    }

    let logical_by_id = logical_key_index(doc);

    for (source_id, block) in &doc.blocks {
        let Some(source_class) = node_class(block) else {
            continue;
        };
        for edge in &block.edges {
            let target_block = match doc.get_block(&edge.target) {
                Some(b) => b,
                None => {
                    diagnostics.push(
                        CodeGraphDiagnostic::error(
                            "CG1014",
                            format!("edge references missing target block {}", edge.target),
                        )
                        .with_logical_key(
                            logical_by_id
                                .get(source_id)
                                .cloned()
                                .unwrap_or_else(|| source_id.to_string()),
                        ),
                    );
                    continue;
                }
            };

            let target_class = node_class(target_block).unwrap_or_default();

            match &edge.edge_type {
                EdgeType::References => {
                    if source_class != "file" || target_class != "file" {
                        diagnostics.push(
                            CodeGraphDiagnostic::error(
                                "CG1015",
                                "references edges must connect file -> file",
                            )
                            .with_logical_key(
                                logical_by_id
                                    .get(source_id)
                                    .cloned()
                                    .unwrap_or_else(|| source_id.to_string()),
                            ),
                        );
                    }
                }
                EdgeType::Custom(name) if name == "exports" => {
                    if source_class != "file" || target_class != "symbol" {
                        diagnostics.push(
                            CodeGraphDiagnostic::error(
                                "CG1016",
                                "exports edges must connect file -> symbol",
                            )
                            .with_logical_key(
                                logical_by_id
                                    .get(source_id)
                                    .cloned()
                                    .unwrap_or_else(|| source_id.to_string()),
                            ),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    CodeGraphValidationResult {
        valid: diagnostics
            .iter()
            .all(|d| d.severity != CodeGraphSeverity::Error),
        diagnostics,
    }
}
