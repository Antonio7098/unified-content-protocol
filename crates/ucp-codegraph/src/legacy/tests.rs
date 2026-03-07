use super::*;
use crate::model::*;
use crate::{CodeGraphBuildInput, CodeGraphExtractorConfig};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use tempfile::tempdir;
use ucm_core::{Block, BlockId, Content, Document, EdgeType};

fn symbol_logical_keys(doc: &Document) -> Vec<String> {
    let mut out: Vec<_> = doc
        .blocks
        .values()
        .filter(|block| node_class(block).as_deref() == Some("symbol"))
        .filter_map(block_logical_key)
        .collect();
    out.sort();
    out
}

fn logical_key_to_block_id(doc: &Document) -> BTreeMap<String, BlockId> {
    doc.blocks
        .iter()
        .filter_map(|(id, block)| block_logical_key(block).map(|key| (key, *id)))
        .collect()
}

fn symbol_block_by_prefix<'a>(doc: &'a Document, prefix: &str) -> Option<&'a Block> {
    doc.blocks
        .values()
        .filter(|block| node_class(block).as_deref() == Some("symbol"))
        .filter_map(|block| block_logical_key(block).map(|key| (key, block)))
        .find(|(key, _)| *key == prefix)
        .map(|(_, block)| block)
        .or_else(|| {
            doc.blocks.values().find(|block| {
                node_class(block).as_deref() == Some("symbol")
                    && block_logical_key(block)
                        .map(|key| key.starts_with(prefix))
                        .unwrap_or(false)
            })
        })
}

fn symbol_exported(doc: &Document, prefix: &str) -> bool {
    symbol_block_by_prefix(doc, prefix)
        .and_then(|block| block.metadata.custom.get(META_EXPORTED))
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn symbol_kind(doc: &Document, prefix: &str) -> Option<String> {
    symbol_block_by_prefix(doc, prefix)
        .and_then(|block| block.metadata.custom.get(META_SYMBOL_KIND))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn symbol_summary(doc: &Document, prefix: &str) -> Option<String> {
    symbol_block_by_prefix(doc, prefix).and_then(|block| block.metadata.summary.clone())
}

fn symbol_content_string_field(doc: &Document, prefix: &str, field: &str) -> Option<String> {
    let block = symbol_block_by_prefix(doc, prefix)?;
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field)?.as_str().map(|value| value.to_string())
}

fn symbol_content_json_field(
    doc: &Document,
    prefix: &str,
    field: &str,
) -> Option<serde_json::Value> {
    let block = symbol_block_by_prefix(doc, prefix)?;
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field).cloned()
}

fn symbol_content_json_subfield(
    doc: &Document,
    prefix: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    symbol_content_json_field(doc, prefix, field)?
        .get(subfield)
        .cloned()
}

fn block_metadata_custom_field(block: &Block, field: &str) -> Option<serde_json::Value> {
    block.metadata.custom.get(field).cloned()
}

fn block_metadata_custom_subfield(
    block: &Block,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_field(block, field)?
        .get(subfield)
        .cloned()
}

fn repository_block(doc: &Document) -> Option<&Block> {
    doc.blocks
        .values()
        .find(|block| node_class(block).as_deref() == Some("repository"))
}

fn repository_content_json_field(doc: &Document, field: &str) -> Option<serde_json::Value> {
    let block = repository_block(doc)?;
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field).cloned()
}

fn repository_content_json_subfield(
    doc: &Document,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    repository_content_json_field(doc, field)?
        .get(subfield)
        .cloned()
}

fn repository_metadata_custom_subfield(
    doc: &Document,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_subfield(repository_block(doc)?, field, subfield)
}

fn directory_block_by_key<'a>(doc: &'a Document, logical_key: &str) -> Option<&'a Block> {
    doc.blocks.values().find(|block| {
        node_class(block).as_deref() == Some("directory")
            && block_logical_key(block).as_deref() == Some(logical_key)
    })
}

fn directory_content_json_field(
    doc: &Document,
    logical_key: &str,
    field: &str,
) -> Option<serde_json::Value> {
    let block = directory_block_by_key(doc, logical_key)?;
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field).cloned()
}

fn directory_content_json_subfield(
    doc: &Document,
    logical_key: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    directory_content_json_field(doc, logical_key, field)?
        .get(subfield)
        .cloned()
}

fn directory_metadata_custom_subfield(
    doc: &Document,
    logical_key: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_subfield(directory_block_by_key(doc, logical_key)?, field, subfield)
}

fn file_content_json_field(
    doc: &Document,
    logical_key: &str,
    field: &str,
) -> Option<serde_json::Value> {
    let block = file_block_by_key(doc, logical_key)?;
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field).cloned()
}

fn file_content_json_subfield(
    doc: &Document,
    logical_key: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    file_content_json_field(doc, logical_key, field)?
        .get(subfield)
        .cloned()
}

fn file_metadata_custom_subfield(
    doc: &Document,
    logical_key: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_subfield(file_block_by_key(doc, logical_key)?, field, subfield)
}

fn symbol_metadata_custom_subfield(
    doc: &Document,
    prefix: &str,
    field: &str,
    subfield: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_subfield(symbol_block_by_prefix(doc, prefix)?, field, subfield)
}

fn symbol_metadata_custom_field(
    doc: &Document,
    prefix: &str,
    field: &str,
) -> Option<serde_json::Value> {
    block_metadata_custom_field(symbol_block_by_prefix(doc, prefix)?, field)
}

fn file_block_by_key<'a>(doc: &'a Document, logical_key: &str) -> Option<&'a Block> {
    doc.blocks.values().find(|block| {
        node_class(block).as_deref() == Some("file")
            && block_logical_key(block).as_deref() == Some(logical_key)
    })
}

fn file_summary(doc: &Document, logical_key: &str) -> Option<String> {
    file_block_by_key(doc, logical_key).and_then(|block| block.metadata.summary.clone())
}

fn block_logical_key_by_id(doc: &Document, block_id: BlockId) -> Option<String> {
    doc.blocks.get(&block_id).and_then(block_logical_key)
}

fn file_has_edge_to_symbol(
    doc: &Document,
    file_key: &str,
    edge_type: &str,
    relation: &str,
    symbol_prefix: &str,
) -> bool {
    let Some(block) = file_block_by_key(doc, file_key) else {
        return false;
    };

    block.edges.iter().any(|edge| {
        edge_type_name(&edge.edge_type) == edge_type
            && edge
                .metadata
                .custom
                .get("relation")
                .and_then(|value| value.as_str())
                == Some(relation)
            && block_logical_key_by_id(doc, edge.target)
                .map(|key| key.starts_with(symbol_prefix))
                .unwrap_or(false)
    })
}

fn symbol_has_edge_to_symbol(
    doc: &Document,
    source_prefix: &str,
    edge_type: &str,
    relation: &str,
    target_prefix: &str,
) -> bool {
    let Some(block) = symbol_block_by_prefix(doc, source_prefix) else {
        return false;
    };

    block.edges.iter().any(|edge| {
        edge_type_name(&edge.edge_type) == edge_type
            && edge
                .metadata
                .custom
                .get("relation")
                .and_then(|value| value.as_str())
                == Some(relation)
            && block_logical_key_by_id(doc, edge.target)
                .map(|key| key.starts_with(target_prefix))
                .unwrap_or(false)
    })
}

fn edge_type_name(edge_type: &EdgeType) -> String {
    match edge_type {
        EdgeType::References => "references".to_string(),
        EdgeType::Custom(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

#[test]
fn test_validate_profile_detects_missing_markers() {
    let doc = Document::create();
    let result = validate_code_graph_profile(&doc);
    assert!(!result.valid);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.code == "CG1001" || d.code == "CG1002"));
}

#[test]
fn test_canonical_fingerprint_stable_for_equivalent_docs() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn a() {}\n").unwrap();

    let input = CodeGraphBuildInput {
        repository_path: root.to_path_buf(),
        commit_hash: "abc123".to_string(),
        config: CodeGraphExtractorConfig::default(),
    };

    let first = build_code_graph(&input).unwrap();
    let second = build_code_graph(&input).unwrap();

    assert_eq!(first.canonical_fingerprint, second.canonical_fingerprint);
    assert_eq!(
        canonical_codegraph_json(&first.document).unwrap(),
        canonical_codegraph_json(&second.document).unwrap()
    );
}

#[test]
fn test_portable_document_roundtrip_preserves_fingerprint() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("pkg")).unwrap();
    fs::write(
        dir.path().join("pkg/main.py"),
        "from .util import helper\n\ndef run():\n    return helper()\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("pkg/util.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "def456".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    let portable = PortableDocument::from_document(&build.document);
    let json = serde_json::to_string_pretty(&portable).unwrap();
    let decoded: PortableDocument = serde_json::from_str(&json).unwrap();
    let roundtripped = decoded.to_document().unwrap();

    let fp1 = canonical_fingerprint(&build.document).unwrap();
    let fp2 = canonical_fingerprint(&roundtripped).unwrap();
    assert_eq!(fp1, fp2);
}

#[test]
fn test_unresolved_import_produces_diagnostic() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "use crate::missing::thing;\npub fn keep() {}\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ghi789".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(build
        .diagnostics
        .iter()
        .any(|d| d.code == "CG2006" && d.severity == CodeGraphSeverity::Warning));
}

#[test]
fn test_gitignore_rule_matches() {
    let rule = GitignoreRule::from_pattern("target/").unwrap();
    assert!(rule.regex.is_match("target"));
    assert!(rule.regex.is_match("target/debug/app"));
}

#[test]
fn test_import_resolution_ts_relative() {
    let mut known = BTreeSet::new();
    known.insert("src/main.ts".to_string());
    known.insert("src/util.ts".to_string());

    let resolved = resolve_ts_import("src/main.ts", "./util", &known);
    assert_eq!(
        resolved,
        ImportResolution::Resolved("src/util.ts".to_string())
    );
}

#[test]
fn test_rust_use_group_expansion() {
    let imports = expand_rust_use_declaration(
        "use crate::{block::{Block, BlockState}, edge::Edge, util::{self, helper as helper_fn}};",
    );

    assert_eq!(
        imports,
        vec![
            "crate::block::Block".to_string(),
            "crate::block::BlockState".to_string(),
            "crate::edge::Edge".to_string(),
            "crate::util".to_string(),
            "crate::util::helper".to_string(),
        ]
    );
}

#[test]
fn test_workspace_rust_imports_resolve_without_external_warnings() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("crates/demo/src")).unwrap();
    fs::write(
            dir.path().join("crates/demo/src/lib.rs"),
            "use anyhow::Result;\nuse crate::block::{helper, Thing};\npub fn run() -> Result<()> { helper(); let _ = Thing; Ok(()) }\n",
        )
        .unwrap();
    fs::write(
        dir.path().join("crates/demo/src/block.rs"),
        "pub struct Thing;\npub fn helper() {}\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "workspace-imports".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
    assert!(build.stats.reference_edges >= 1);
}

#[test]
fn test_rust_crate_root_symbol_imports_resolve_to_entry_file() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub struct Document;\npub type Result<T> = std::result::Result<T, ()>;\nmod inner;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/inner.rs"),
        "use crate::Document;\nuse crate::Result;\npub fn run() {}\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "crate-root-symbols".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
    assert!(build.stats.reference_edges >= 1);
}

#[test]
fn test_python_relative_module_import_and_symbol_edges_are_captured() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("pkg")).unwrap();
    fs::write(
        dir.path().join("pkg/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("pkg/mod.py"),
        "from . import helper\nfrom .helper import helper as helper_fn\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "python-relative-imports".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:pkg/mod.py",
        "imports_symbol",
        "imports_symbol",
        "symbol:pkg/helper.py::helper",
    ));
}

#[test]
fn test_python_all_marks_reexported_package_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("pkg")).unwrap();
    fs::write(
        dir.path().join("pkg/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("pkg/__init__.py"),
        "from .helper import helper\n__all__ = [\"helper\"]\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "python-all-reexports".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:pkg/__init__.py",
        "exports",
        "reexports",
        "symbol:pkg/helper.py::helper",
    ));
}

#[test]
fn test_rust_and_ts_reexports_point_to_target_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::write(dir.path().join("src/helper.rs"), "pub fn helper() {}\n").unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub mod helper;\npub use helper::helper;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/helper.ts"),
        "export function helper() { return 1; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/mod.ts"),
        "export { helper } from './helper';\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "reexports".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:src/lib.rs",
        "exports",
        "reexports",
        "symbol:src/helper.rs::helper",
    ));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:web/mod.ts",
        "exports",
        "reexports",
        "symbol:web/helper.ts::helper",
    ));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:web/mod.ts",
        "imports_symbol",
        "imports_symbol",
        "symbol:web/helper.ts::helper",
    ));
}

#[test]
fn test_wildcard_imports_and_reexports_expand_to_exported_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("pkg")).unwrap();

    fs::write(
        dir.path().join("src/helper.rs"),
        "pub fn a() {}\npub fn b() {}\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub mod helper;\npub use helper::*;\n",
    )
    .unwrap();

    fs::write(
        dir.path().join("web/helper.ts"),
        "export function a() { return 1; }\nexport const b = 2;\n",
    )
    .unwrap();
    fs::write(dir.path().join("web/mod.ts"), "export * from './helper';\n").unwrap();

    fs::write(
        dir.path().join("pkg/helper.py"),
        "def a():\n    return 1\n\ndef _hidden():\n    return 2\n__all__ = [\"a\"]\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("pkg/__init__.py"),
        "from .helper import *\n__all__ = [\"a\"]\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "wildcard-semantics".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:src/lib.rs",
        "imports_symbol",
        "imports_symbol",
        "symbol:src/helper.rs::a",
    ));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:src/lib.rs",
        "exports",
        "reexports",
        "symbol:src/helper.rs::b",
    ));

    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:web/mod.ts",
        "imports_symbol",
        "imports_symbol",
        "symbol:web/helper.ts::a",
    ));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:web/mod.ts",
        "exports",
        "reexports",
        "symbol:web/helper.ts::b",
    ));

    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:pkg/__init__.py",
        "imports_symbol",
        "imports_symbol",
        "symbol:pkg/helper.py::a",
    ));
    assert!(file_has_edge_to_symbol(
        &build.document,
        "file:pkg/__init__.py",
        "exports",
        "reexports",
        "symbol:pkg/helper.py::a",
    ));
    assert!(!file_has_edge_to_symbol(
        &build.document,
        "file:pkg/__init__.py",
        "exports",
        "reexports",
        "symbol:pkg/helper.py::_hidden",
    ));
}

#[test]
fn test_explicit_type_relationship_edges_resolve_across_supported_languages() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/base.ts"),
        "export class Base {}\nexport interface Face {}\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/app.ts"),
            "import { Base as ImportedBase, Face as ImportedFace } from './base';\nclass Child extends ImportedBase implements ImportedFace {}\n",
        )
        .unwrap();

    fs::write(dir.path().join("py/base.py"), "class Base:\n    pass\n").unwrap();
    fs::write(
        dir.path().join("py/app.py"),
        "from .base import Base as ImportedBase\nclass Child(ImportedBase):\n    pass\n",
    )
    .unwrap();

    fs::write(
        dir.path().join("src/base.rs"),
        "pub trait Face {}\npub struct Thing;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "mod base;\nuse crate::base::Face;\nstruct Thing;\nimpl Face for Thing {}\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "type-relationships".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/app.ts::Child",
        "extends",
        "extends",
        "symbol:web/base.ts::Base",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/app.ts::Child",
        "implements",
        "implements",
        "symbol:web/base.ts::Face",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/app.py::Child",
        "extends",
        "extends",
        "symbol:py/base.py::Base",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::Thing#",
        "implements",
        "implements",
        "symbol:src/base.rs::Face",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::Thing#",
        "for_type",
        "for_type",
        "symbol:src/lib.rs::Thing",
    ));
}

#[test]
fn test_call_sites_resolve_to_same_file_and_imported_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("src/util.rs"),
        "pub fn greet() -> String { \"hi\".to_string() }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "mod util;\npub fn run() -> String { util::greet() }\n",
    )
    .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/main.py"),
        "from .helper import helper\ndef execute():\n    return helper()\n",
    )
    .unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/main.ts"),
        "import { util } from './util';\nexport function run() { return util(); }\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "call-sites".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
}

#[test]
fn test_ts_constructor_aliases_resolve_to_class_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(dir.path().join("web/thing.ts"), "export class Thing {}\n").unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { Thing } from './thing';\nconst Ctor = Thing;\nexport function make() { return new Ctor(); }\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("web/member.ts"),
            "import * as ns from './thing';\nconst First = ns.Thing;\nconst Second = First;\nexport function build() { return new Second(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ts-constructor-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::make",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/member.ts::build",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
}

#[test]
fn test_ts_namespace_and_module_alias_member_constructors_resolve_to_class_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(dir.path().join("web/thing.ts"), "export class Thing {}\n").unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './thing';\nexport function direct() { return new ns.Thing(); }\nconst alias = ns;\nexport function top() { return new alias.Thing(); }\nexport function local() { const first = ns; const second = first; return new second.Thing(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ts-namespace-member-constructors".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::direct",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::top",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::local",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
}

#[test]
fn test_ts_default_import_calls_and_constructors_resolve_to_default_export_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export default function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/thing.ts"),
        "export default class Thing {}\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import util from './util';\nimport { default as util_spec } from './util';\nimport Thing from './thing';\nconst first = util;\nconst second = first;\nconst third = util_spec;\nconst Alias = Thing;\nexport function run() { return second(); }\nexport function run_spec() { return third(); }\nexport function make() { return new Alias(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ts-default-import-calls-and-constructors".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_spec",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::make",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
}

#[test]
fn test_ts_default_import_shadowing_does_not_fall_back_and_anonymous_defaults_stay_unresolved() {
    let shadow_dir = tempdir().unwrap();
    fs::create_dir_all(shadow_dir.path().join("web")).unwrap();

    fs::write(
        shadow_dir.path().join("web/util.ts"),
        "export default function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            shadow_dir.path().join("web/main.ts"),
            "import util from './util';\nexport function missing_case() { const util = missing; return util(); }\nexport function expr_case() { const util = true ? missing : util; return util(); }\n",
        )
        .unwrap();

    let shadow_build = build_code_graph(&CodeGraphBuildInput {
        repository_path: shadow_dir.path().to_path_buf(),
        commit_hash: "ts-default-import-shadowing".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &shadow_build.document,
        "symbol:web/main.ts::missing_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &shadow_build.document,
        "symbol:web/main.ts::expr_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));

    let anon_dir = tempdir().unwrap();
    fs::create_dir_all(anon_dir.path().join("web")).unwrap();
    fs::write(
        anon_dir.path().join("web/util.ts"),
        "export default function() { return 42; }\n",
    )
    .unwrap();
    fs::write(
        anon_dir.path().join("web/main.ts"),
        "import util from './util';\nexport function run() { return util(); }\n",
    )
    .unwrap();

    let anon_build = build_code_graph(&CodeGraphBuildInput {
        repository_path: anon_dir.path().to_path_buf(),
        commit_hash: "ts-anonymous-default-import".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &anon_build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
}

#[test]
fn test_ts_reexported_calls_and_constructors_resolve_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export default function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/thing.ts"),
        "export default class Thing {}\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/index.ts"),
            "export { default as util } from './util';\nexport { util as util_alias } from './util';\nexport { default as Thing } from './thing';\nexport * from './util';\n",
        )
        .unwrap();
    fs::write(
        dir.path().join("web/index2.ts"),
        "export { util as util_chain } from './index';\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/ns_main.ts"),
            "import * as ns from './index';\nexport function run_alias_member() { return ns.util_alias(); }\n",
        )
        .unwrap();
    fs::write(
        dir.path().join("web/default_index.ts"),
        "export * from './util';\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { util, Thing } from './index';\nimport { util as util_wildcard } from './index';\nimport { util_chain } from './index2';\nimport util_default from './default_index';\nconst alias = util_chain;\nexport function run() { return util(); }\nexport function run_wildcard() { return util_wildcard(); }\nexport function run_chain() { return alias(); }\nexport function run_default_excluded() { return util_default(); }\nexport function make() { return new Thing(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ts-reexported-calls-and-constructors".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_wildcard",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_chain",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/ns_main.ts::run_alias_member",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_default_excluded",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::make",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/thing.ts::Thing",
    ));
}

#[test]
fn test_js_commonjs_require_calls_resolve_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
        dir.path().join("web/default_util.js"),
        "module.exports = function util() { return 42; };\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/named_util.js"),
        "exports.greet = function greet() { return 42; };\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/object_util.js"),
        "function greet() { return 42; }\nmodule.exports = { greet };\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/object_renamed_util.js"),
        "function greet() { return 42; }\nmodule.exports = { hi: greet };\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/object_inline_util.js"),
        "module.exports = { greet: function greet() { return 42; } };\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/object_class_util.js"),
        "class Thing {}\nmodule.exports = { Alias: Thing };\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.js"),
            "const util = require('./default_util');\nconst { greet } = require('./named_util');\nconst named = require('./named_util');\nconst { greet: object_greet } = require('./object_util');\nconst renamed = require('./object_renamed_util');\nconst inline_named = require('./object_inline_util');\nconst { hi } = require('./object_renamed_util');\nconst { greet: inline_greet } = require('./object_inline_util');\nconst { Alias } = require('./object_class_util');\nconst class_mod = require('./object_class_util');\nexport function run_default() { return util(); }\nexport function run_named() { return greet(); }\nexport function run_member() { return named.greet(); }\nexport function run_object_named() { return object_greet(); }\nexport function run_object_renamed() { return hi(); }\nexport function run_object_inline() { return inline_greet(); }\nexport function run_object_member_renamed() { return renamed.hi(); }\nexport function run_object_member_inline() { return inline_named.greet(); }\nexport function make_object_class() { return new Alias(); }\nexport function make_object_class_member() { return new class_mod.Alias(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "js-commonjs-require-calls".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_default",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/default_util.js::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_named",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/named_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_member",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/named_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_object_named",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_object_renamed",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_renamed_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_object_inline",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_inline_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_object_member_renamed",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_renamed_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::run_object_member_inline",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_inline_util.js::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::make_object_class",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_class_util.js::Thing",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.js::make_object_class_member",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/object_class_util.js::Thing",
    ));
}

#[test]
fn test_python_package_reexported_calls_resolve_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("py/pkg")).unwrap();
    fs::create_dir_all(dir.path().join("py/pkg_wild")).unwrap();

    fs::write(
        dir.path().join("py/pkg/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/pkg/__init__.py"),
        "from .helper import helper\nfrom .helper import helper as alias\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/pkg_wild/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/pkg_wild/__init__.py"),
        "from .helper import *\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .pkg import helper\nfrom .pkg import helper as alias\nfrom .pkg_wild import helper as wild_helper\nfrom . import pkg\ndef run_pkg():\n    helper()\n    return alias()\ndef run_pkg_member_alias():\n    return pkg.alias()\ndef run_pkg_wild():\n    return wild_helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "python-package-reexported-calls".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_pkg",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/pkg/helper.py::helper",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_pkg_member_alias",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/pkg/helper.py::helper",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_pkg_wild",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/pkg_wild/helper.py::helper",
    ));
}

#[test]
fn test_python_wildcard_imports_resolve_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("py/pkg")).unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def greet():\n    return 1\ndef wave():\n    return 2\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/pkg/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/pkg/__init__.py"),
        "from .helper import helper\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .helper import *\nfrom .pkg import *\ndef run_module():\n    greet()\n    return wave()\ndef run_package():\n    return helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "python-wildcard-imports".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_module",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_module",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::wave",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run_package",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/pkg/helper.py::helper",
    ));
}

#[test]
fn test_python_wildcard_imports_respect_export_rules() {
    let underscore_dir = tempdir().unwrap();
    fs::create_dir_all(underscore_dir.path().join("py")).unwrap();

    fs::write(
        underscore_dir.path().join("py/helper.py"),
        "def public():\n    return 1\ndef _hidden():\n    return 2\n",
    )
    .unwrap();
    fs::write(
            underscore_dir.path().join("py/main.py"),
            "from .helper import *\ndef run_public():\n    return public()\ndef run_hidden():\n    return _hidden()\n",
        )
        .unwrap();

    let underscore_build = build_code_graph(&CodeGraphBuildInput {
        repository_path: underscore_dir.path().to_path_buf(),
        commit_hash: "python-wildcard-imports-underscore-rules".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &underscore_build.document,
        "symbol:py/main.py::run_public",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::public",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &underscore_build.document,
        "symbol:py/main.py::run_hidden",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::_hidden",
    ));

    let all_dir = tempdir().unwrap();
    fs::create_dir_all(all_dir.path().join("py")).unwrap();
    fs::write(
        all_dir.path().join("py/helper.py"),
        "__all__ = ['chosen']\n\ndef chosen():\n    return 1\n\ndef extra():\n    return 2\n",
    )
    .unwrap();
    fs::write(
            all_dir.path().join("py/main.py"),
            "from .helper import *\ndef run_chosen():\n    return chosen()\ndef run_extra():\n    return extra()\n",
        )
        .unwrap();

    let all_build = build_code_graph(&CodeGraphBuildInput {
        repository_path: all_dir.path().to_path_buf(),
        commit_hash: "python-wildcard-imports-all-rules".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &all_build.document,
        "symbol:py/main.py::run_chosen",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::chosen",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &all_build.document,
        "symbol:py/main.py::run_extra",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::extra",
    ));
}

#[test]
fn test_rust_import_aliases_and_nested_paths_resolve_to_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src/nested")).unwrap();

    fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
    fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
    fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nuse util::greet as hello;\npub fn run() { hello(); nested::util::wave(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-import-alias-and-nested-paths".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/nested/util.rs::wave",
    ));
}

#[test]
fn test_rust_pub_use_reexports_and_wildcards_resolve_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src/nested")).unwrap();

    fs::write(
        dir.path().join("src/util.rs"),
        "pub fn greet() {}\npub fn wave() {}\n",
    )
    .unwrap();
    fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
    fs::write(dir.path().join("src/nested/util.rs"), "pub fn ping() {}\n").unwrap();
    fs::write(
        dir.path().join("src/barrel1.rs"),
        "pub use crate::util::greet;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/barrel2.rs"),
        "pub use crate::barrel1::greet as hello;\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nmod barrel1;\nmod barrel2;\npub use util::greet;\npub use util::wave as wave_alias;\npub use util::*;\npub use nested::util as util_mod;\npub use nested::{util as util_mod_two};\nuse barrel2::hello;\npub fn run() { greet(); wave(); wave_alias(); util_mod::ping(); util_mod_two::ping(); hello(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-pub-use-reexports-and-wildcards".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::wave",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/nested/util.rs::ping",
    ));
}

#[test]
fn test_rust_module_alias_paths_resolve_to_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src/nested")).unwrap();

    fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
    fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod nested;\nuse nested::util as util_mod;\nuse nested::util::{self as util_mod_two};\nuse nested::{util as util_mod_three};\npub fn run() { util_mod::wave(); util_mod_two::wave(); util_mod_three::wave(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-module-alias-paths".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/nested/util.rs::wave",
    ));
}

#[test]
fn test_rust_function_local_aliases_resolve_to_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src/nested")).unwrap();

    fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
    fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
    fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nuse util::greet;\nuse nested::util as util_mod;\nuse nested::util::{wave as hello};\npub fn run() { let first = greet; let second = first; let wave_alias = util_mod::wave; let hello_alias = hello; second(); wave_alias(); hello_alias(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-function-local-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::greet",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/nested/util.rs::wave",
    ));
}

#[test]
fn test_rust_local_aliases_shadow_imported_names_and_stay_isolated() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();

    fs::write(dir.path().join("src/one.rs"), "pub fn one() {}\n").unwrap();
    fs::write(dir.path().join("src/two.rs"), "pub fn two() {}\n").unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod one;\nmod two;\nuse one::one;\nuse two::two;\npub fn run() { let one = two; one(); }\npub fn first() { let alias = one; alias(); }\npub fn second() { two(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-local-shadowing-and-isolation".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/two.rs::two",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::first",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/one.rs::one",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::second",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/two.rs::two",
    ));
}

#[test]
fn test_rust_unresolved_or_unsupported_local_shadowing_does_not_fall_back() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();

    fs::write(dir.path().join("src/one.rs"), "pub fn one() {}\n").unwrap();
    fs::write(
            dir.path().join("src/lib.rs"),
            "mod one;\nuse one::one;\npub fn missing_case() { let one = missing; one(); }\npub fn expr_case() { let one = if true { missing } else { one }; one(); }\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "rust-local-shadowing-no-fallback".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::missing_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/one.rs::one",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::expr_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/one.rs::one",
    ));
}

#[test]
fn test_alias_cycles_remain_unresolved_without_symbol_edges() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();

    fs::write(
        dir.path().join("web/main.ts"),
        "const a = b;\nconst b = a;\nexport function run() { return a(); }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/main.py"),
        "a = b\nb = a\ndef run():\n    return a()\n",
    )
    .unwrap();
    fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "mod util;\nuse util::greet;\npub fn run() { let a = b; let b = a; a(); }\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "alias-cycles-unresolved".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/main.ts::a",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/main.py::a",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/lib.rs::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:src/util.rs::greet",
    ));
}

#[test]
fn test_interface_and_trait_inheritance_edges_are_emitted() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
        dir.path().join("src/base.rs"),
        "pub trait Parent {}\npub trait Child: Parent {}\n",
    )
    .unwrap();
    fs::write(dir.path().join("src/lib.rs"), "mod base;\n").unwrap();

    fs::write(
        dir.path().join("web/base.ts"),
        "export interface Parent {}\nexport interface Child extends Parent {}\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "interface-trait-inheritance".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/base.ts::Child",
        "extends",
        "extends",
        "symbol:web/base.ts::Parent",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:src/base.rs::Child",
        "extends",
        "extends",
        "symbol:src/base.rs::Parent",
    ));
}

#[test]
fn test_namespace_and_module_member_calls_resolve_to_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/main.ts"),
        "import * as utilns from './util';\nexport function run() { return utilns.util(); }\n",
    )
    .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/main.py"),
        "from . import helper as helper_mod\ndef execute():\n    return helper_mod.helper()\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "member-call-sites".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_top_level_aliases_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nconst alias = util;\nexport function run() { return alias(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/main.py"),
        "from .helper import helper\nalias = helper\ndef execute():\n    return alias()\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "top-level-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_top_level_member_aliases_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst f = utilns.util;\nexport function run() { return f(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod.helper\ndef execute():\n    return alias()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "top-level-member-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_top_level_module_aliases_resolve_member_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nconst alias = ns;\nexport function run() { return alias.util(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod\ndef execute():\n    return alias.helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "top-level-module-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_unresolved_or_unsupported_module_alias_shadowing_does_not_fall_back() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nconst alias = ns;\nexport function missing_case() { const alias = missing; return alias.util(); }\nexport function expr_case() { const alias = true ? ns : ns; return alias.util(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod\ndef missing_case():\n    alias = missing\n    return alias.helper()\ndef expr_case():\n    alias = helper_mod if True else helper_mod\n    return alias.helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "module-alias-shadowing-no-fallback".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::missing_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::expr_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::missing_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::expr_case",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_alias_chains_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nconst first = util;\nconst second = first;\nexport function run() { return second(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\nfirst = helper\nsecond = first\ndef execute():\n    return second()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "alias-chains".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_top_level_member_alias_chains_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst first = utilns.util;\nconst second = first;\nexport function run() { return second(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nfirst = helper_mod.helper\nsecond = first\ndef execute():\n    return second()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "top-level-member-alias-chains".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_mixed_scope_member_aliases_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst top = utilns.util;\nexport function run() { const local = top; return local(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ntop = helper_mod.helper\ndef execute():\n    local = top\n    return local()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "mixed-scope-member-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_local_aliases_shadow_top_level_aliases_of_same_name() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/one.ts"),
        "export function one() { return 1; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/two.ts"),
        "export function two() { return 2; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nimport { two } from './two';\nconst alias = one;\nexport function run() { const alias = two; return alias(); }\n",
        )
        .unwrap();

    fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
    fs::write(dir.path().join("py/two.py"), "def two():\n    return 2\n").unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nfrom .two import two\nalias = one\ndef execute():\n    alias = two\n    return alias()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "alias-shadowing".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/two.ts::two",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/two.py::two",
    ));
}

#[test]
fn test_unresolved_or_unsupported_local_shadowing_does_not_fall_back_to_top_level_aliases() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/one.ts"),
        "export function one() { return 1; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nconst alias = one;\nexport function run_missing() { const alias = missing; return alias(); }\nexport function run_expr() { const alias = true ? one : one; return alias(); }\n",
        )
        .unwrap();

    fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nalias = one\ndef execute_missing():\n    alias = missing\n    return alias()\ndef execute_expr():\n    alias = 1 if True else one\n    return alias()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "shadowing-no-fallback".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_missing",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/one.ts::one",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run_expr",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/one.ts::one",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute_missing",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/one.py::one",
    ));
    assert!(!symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute_expr",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/one.py::one",
    ));
}

#[test]
fn test_local_aliases_are_isolated_to_their_own_enclosing_symbol() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/one.ts"),
        "export function one() { return 1; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("web/two.ts"),
        "export function two() { return 2; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nimport { two } from './two';\nexport function first() { const alias = one; return alias(); }\nexport function second() { return two(); }\n",
        )
        .unwrap();

    fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
    fs::write(dir.path().join("py/two.py"), "def two():\n    return 2\n").unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nfrom .two import two\ndef first():\n    alias = one\n    return alias()\ndef second():\n    return two()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "alias-scope-isolation".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::first",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/one.ts::one",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::second",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/two.ts::two",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::first",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/one.py::one",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::second",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/two.py::two",
    ));
}

#[test]
fn test_function_local_aliases_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nexport function run() { const localAlias = util; return localAlias(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("py/main.py"),
        "from .helper import helper\ndef execute():\n    alias = helper\n    return alias()\n",
    )
    .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_function_local_alias_chains_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nexport function run() { const first = util; const second = first; return second(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\ndef execute():\n    first = helper\n    second = first\n    return second()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-alias-chains".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_function_local_member_aliases_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nexport function run() { const f = utilns.util; return f(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    alias = helper_mod.helper\n    return alias()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-member-aliases".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_function_local_module_alias_chains_resolve_member_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nexport function run() { const first = ns; const second = first; return second.util(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    first = helper_mod\n    second = first\n    return second.helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-module-alias-chains".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_function_local_module_aliases_are_isolated_to_their_own_enclosing_symbol() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nexport function first() { const alias = ns; return alias.util(); }\nexport function second() { return ns.util(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef first():\n    alias = helper_mod\n    return alias.helper()\ndef second():\n    return helper_mod.helper()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-module-alias-isolation".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::first",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::second",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::first",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::second",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_function_local_member_alias_chains_resolve_call_sites_to_underlying_symbols() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();

    fs::write(
        dir.path().join("web/util.ts"),
        "export function util() { return 42; }\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nexport function run() { const first = utilns.util; const second = first; return second(); }\n",
        )
        .unwrap();

    fs::write(
        dir.path().join("py/helper.py"),
        "def helper():\n    return 1\n",
    )
    .unwrap();
    fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    first = helper_mod.helper\n    second = first\n    return second()\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "function-local-member-alias-chains".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:web/main.ts::run",
        "uses_symbol",
        "uses_symbol",
        "symbol:web/util.ts::util",
    ));
    assert!(symbol_has_edge_to_symbol(
        &build.document,
        "symbol:py/main.py::execute",
        "uses_symbol",
        "uses_symbol",
        "symbol:py/helper.py::helper",
    ));
}

#[test]
fn test_nested_symbols_are_captured_and_nested_in_structure() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("pkg")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
            dir.path().join("src/lib.rs"),
            "pub struct Thing;\nimpl Thing { pub fn method(&self) {} }\npub fn top() { fn inner() {} inner(); }\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("pkg/mod.py"),
            "class Thing:\n    def method(self):\n        return 1\n\ndef top():\n    def inner():\n        return 2\n    return inner()\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("web/mod.ts"),
            "export class Thing {\n  method() { return 1; }\n}\nexport function top() {\n  function inner() { return 2; }\n  return inner();\n}\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "nested-symbols".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    let keys = symbol_logical_keys(&build.document);
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:src/lib.rs::Thing::method")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:src/lib.rs::top::inner")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:pkg/mod.py::Thing::method")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:pkg/mod.py::top::inner")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:web/mod.ts::Thing::method")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:web/mod.ts::top::inner")));

    let key_index = logical_key_to_block_id(&build.document);
    let rust_method_id = key_index
        .iter()
        .find(|(key, _)| key.starts_with("symbol:src/lib.rs::Thing::method"))
        .map(|(_, id)| *id)
        .unwrap();
    let rust_parent_id = build
        .document
        .structure
        .iter()
        .find(|(_, children)| children.contains(&rust_method_id))
        .map(|(id, _)| *id)
        .unwrap();

    let rust_parent_key = key_index
        .iter()
        .find(|(_, id)| **id == rust_parent_id)
        .map(|(key, _)| key.as_str())
        .unwrap();

    assert!(rust_parent_key.starts_with("symbol:src/lib.rs::Thing"));
}

#[test]
fn test_ts_js_export_aliases_generators_and_function_like_members_are_captured() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();

    fs::write(
            dir.path().join("src/mod.ts"),
            "function internal() { return 1; }\nexport { internal };\nexport function* gen() { yield 1; }\nexport const arrow = () => 1;\nclass Example {\n  handler = () => 1;\n}\nexport default Example;\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("src/mod.js"),
            "function internalJs() { return 1; }\nexport { internalJs };\nexport function* jsGen() { yield 1; }\nclass JsExample {\n  handler = () => 1;\n}\nexport default JsExample;\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "ts-js-coverage".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    let keys = symbol_logical_keys(&build.document);
    assert!(keys.iter().any(|k| k.starts_with("symbol:src/mod.ts::gen")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:src/mod.ts::Example::handler")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:src/mod.js::jsGen")));
    assert!(keys
        .iter()
        .any(|k| k.starts_with("symbol:src/mod.js::JsExample::handler")));

    assert!(symbol_exported(
        &build.document,
        "symbol:src/mod.ts::internal"
    ));
    assert!(symbol_exported(
        &build.document,
        "symbol:src/mod.ts::Example"
    ));
    assert!(symbol_exported(
        &build.document,
        "symbol:src/mod.js::internalJs"
    ));
    assert!(symbol_exported(
        &build.document,
        "symbol:src/mod.js::JsExample"
    ));

    assert_eq!(
        symbol_kind(&build.document, "symbol:src/mod.ts::arrow").as_deref(),
        Some("function")
    );
    assert_eq!(
        symbol_kind(&build.document, "symbol:src/mod.ts::Example::handler").as_deref(),
        Some("method")
    );
}

#[test]
fn test_file_descriptions_and_symbol_descriptions_are_extracted() {
    let dir = tempdir().unwrap();
    let repo_name = dir
        .path()
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("py")).unwrap();
    fs::create_dir_all(dir.path().join("web")).unwrap();

    fs::write(
            dir.path().join("src/lib.rs"),
            "//! Rust module summary\n\n/// Rust helper description.\npub async fn helper(value: i32) -> i32 { value }\n/// Rust thing description.\npub trait Thing: Send + Sync {}\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("py/mod.py"),
            "\"\"\"Python module summary.\"\"\"\nfrom typing import Protocol, runtime_checkable\n\ndef helper(value: int) -> int:\n    \"\"\"Python helper description.\"\"\"\n    return value\n\n@runtime_checkable\nclass Thing(Protocol):\n    \"\"\"Python thing description.\"\"\"\n    async def generate_answer(self, question: str) -> str:\n        \"\"\"Generate an answer.\"\"\"\n        return question\n\n    @staticmethod\n    def normalize(value: str) -> str:\n        return value\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("web/mod.ts"),
            "/** TS module summary */\n/** TS helper description. */\nexport async function helper(value: number, label: string): Promise<number> { return value; }\n/** TS thing description. */\nexport class Thing extends Base implements Named { public static make(value: number): Thing { return new Thing(); } }\n",
        )
        .unwrap();
    fs::write(
            dir.path().join("web/mod.js"),
            "/** JS helper description. */\nfunction* helper(value, label = 'x') { yield value; }\nmodule.exports = { helper };\n",
        )
        .unwrap();

    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "file-description-and-symbol-descriptions".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();

    assert_eq!(
        repository_content_json_subfield(&build.document, "coderef", "path"),
        Some(json!("."))
    );
    assert_eq!(
        repository_content_json_subfield(&build.document, "coderef", "display"),
        Some(json!(repo_name))
    );
    assert_eq!(
        repository_metadata_custom_subfield(&build.document, META_CODEREF, "path"),
        Some(json!("."))
    );
    assert_eq!(
        directory_content_json_subfield(&build.document, "directory:src", "coderef", "path"),
        Some(json!("src"))
    );
    assert_eq!(
        directory_content_json_subfield(&build.document, "directory:src", "coderef", "display"),
        Some(json!("src"))
    );
    assert_eq!(
        directory_content_json_field(&build.document, "directory:src", "path"),
        None
    );
    assert_eq!(
        directory_metadata_custom_subfield(&build.document, "directory:src", META_CODEREF, "path"),
        Some(json!("src"))
    );
    assert_eq!(
        file_summary(&build.document, "file:src/lib.rs").as_deref(),
        Some("Rust module summary")
    );
    assert_eq!(
        file_content_json_subfield(&build.document, "file:src/lib.rs", "coderef", "path"),
        Some(json!("src/lib.rs"))
    );
    assert_eq!(
        file_content_json_subfield(&build.document, "file:src/lib.rs", "coderef", "display"),
        Some(json!("src/lib.rs"))
    );
    assert_eq!(
        file_content_json_field(&build.document, "file:src/lib.rs", "path"),
        None
    );
    assert_eq!(
        file_metadata_custom_subfield(&build.document, "file:src/lib.rs", META_CODEREF, "display"),
        Some(json!("src/lib.rs"))
    );
    assert_eq!(
        file_summary(&build.document, "file:py/mod.py").as_deref(),
        Some("Python module summary.")
    );
    assert_eq!(
        file_content_json_subfield(&build.document, "file:py/mod.py", "coderef", "path"),
        Some(json!("py/mod.py"))
    );
    assert_eq!(
        file_summary(&build.document, "file:web/mod.ts").as_deref(),
        Some("TS module summary")
    );
    assert_eq!(
        file_content_json_subfield(&build.document, "file:web/mod.ts", "coderef", "path"),
        Some(json!("web/mod.ts"))
    );

    assert_eq!(
        symbol_summary(&build.document, "symbol:src/lib.rs::helper").as_deref(),
        Some("Rust helper description.")
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:src/lib.rs::helper", "description")
            .as_deref(),
        Some("Rust helper description.")
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:src/lib.rs::helper", "helper")
            .as_deref(),
        None
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:src/lib.rs::helper", "inputs"),
        Some(json!([{"name": "value", "type": "i32"}]))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:src/lib.rs::helper",
            "coderef",
            "path"
        ),
        Some(json!("src/lib.rs"))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:src/lib.rs::helper",
            "coderef",
            "display"
        ),
        Some(json!("src/lib.rs#L4"))
    );
    assert_eq!(
        symbol_metadata_custom_subfield(
            &build.document,
            "symbol:src/lib.rs::helper",
            META_CODEREF,
            "display"
        ),
        Some(json!("src/lib.rs#L4"))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:src/lib.rs::helper",
            "coderef",
            "start_line"
        ),
        Some(json!(4))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:src/lib.rs::helper",
            "coderef",
            "end_line"
        ),
        Some(json!(4))
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:src/lib.rs::helper", "path"),
        None
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:src/lib.rs::helper", "line_range"),
        None
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:src/lib.rs::helper", "span"),
        None
    );
    assert_eq!(
        symbol_metadata_custom_field(&build.document, "symbol:src/lib.rs::helper", "path"),
        None
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:src/lib.rs::helper", "output")
            .as_deref(),
        Some("i32")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:src/lib.rs::helper", "modifiers"),
        Some(json!({"async": true, "visibility": "public"}))
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:src/lib.rs::Thing").as_deref(),
        Some("Rust thing description.")
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:src/lib.rs::Thing", "type").as_deref(),
        Some("Send + Sync")
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:py/mod.py::helper").as_deref(),
        Some("Python helper description.")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:py/mod.py::helper", "inputs"),
        Some(json!([{"name": "value", "type": "int"}]))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:py/mod.py::helper",
            "coderef",
            "path"
        ),
        Some(json!("py/mod.py"))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:py/mod.py::helper",
            "coderef",
            "display"
        ),
        Some(json!("py/mod.py#L4-L6"))
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:py/mod.py::helper", "output")
            .as_deref(),
        Some("int")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:py/mod.py::helper", "modifiers"),
        None
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:py/mod.py::Thing").as_deref(),
        Some("Python thing description.")
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:py/mod.py::Thing", "type").as_deref(),
        Some("Protocol")
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:py/mod.py::Thing::generate_answer").as_deref(),
        Some("Generate an answer.")
    );
    assert_eq!(
        symbol_content_json_field(
            &build.document,
            "symbol:py/mod.py::Thing::generate_answer",
            "inputs"
        ),
        Some(json!([
            {"name": "self"},
            {"name": "question", "type": "str"}
        ]))
    );
    assert_eq!(
        symbol_content_string_field(
            &build.document,
            "symbol:py/mod.py::Thing::generate_answer",
            "output"
        )
        .as_deref(),
        Some("str")
    );
    assert_eq!(
        symbol_content_json_field(
            &build.document,
            "symbol:py/mod.py::Thing::generate_answer",
            "modifiers"
        ),
        Some(json!({"async": true}))
    );
    assert_eq!(
        symbol_content_json_field(
            &build.document,
            "symbol:py/mod.py::Thing::normalize",
            "modifiers"
        ),
        Some(json!({"static": true}))
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:web/mod.ts::helper").as_deref(),
        Some("TS helper description.")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:web/mod.ts::helper", "inputs"),
        Some(json!([
            {"name": "value", "type": "number"},
            {"name": "label", "type": "string"}
        ]))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:web/mod.ts::helper",
            "coderef",
            "path"
        ),
        Some(json!("web/mod.ts"))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:web/mod.ts::helper",
            "coderef",
            "display"
        ),
        Some(json!("web/mod.ts#L3"))
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:web/mod.ts::helper", "output")
            .as_deref(),
        Some("Promise<number>")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:web/mod.ts::helper", "modifiers"),
        Some(json!({"async": true}))
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:web/mod.ts::Thing").as_deref(),
        Some("TS thing description.")
    );
    assert_eq!(
        symbol_content_string_field(&build.document, "symbol:web/mod.ts::Thing", "type").as_deref(),
        Some("extends Base implements Named")
    );
    assert_eq!(
        symbol_content_json_field(
            &build.document,
            "symbol:web/mod.ts::Thing::make",
            "modifiers"
        ),
        Some(json!({"static": true, "visibility": "public"}))
    );
    assert_eq!(
        symbol_summary(&build.document, "symbol:web/mod.js::helper").as_deref(),
        Some("JS helper description.")
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:web/mod.js::helper", "inputs"),
        Some(json!([
            {"name": "value"},
            {"name": "label"}
        ]))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:web/mod.js::helper",
            "coderef",
            "path"
        ),
        Some(json!("web/mod.js"))
    );
    assert_eq!(
        symbol_content_json_subfield(
            &build.document,
            "symbol:web/mod.js::helper",
            "coderef",
            "display"
        ),
        Some(json!("web/mod.js#L2"))
    );
    assert_eq!(
        symbol_content_json_field(&build.document, "symbol:web/mod.js::helper", "modifiers"),
        Some(json!({"generator": true}))
    );
}

#[test]
fn test_performance_smoke_medium_fixture() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();

    for i in 0..300usize {
        let mut file = fs::File::create(src.join(format!("m{}.rs", i))).unwrap();
        writeln!(file, "pub fn f{}() {{}}", i).unwrap();
        if i > 0 {
            writeln!(file, "use crate::m{}::f{};", i - 1, i - 1).unwrap();
        }
    }

    let start = std::time::Instant::now();
    let build = build_code_graph(&CodeGraphBuildInput {
        repository_path: dir.path().to_path_buf(),
        commit_hash: "perf-smoke".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .unwrap();
    let elapsed = start.elapsed();

    assert!(build.stats.file_nodes >= 300);
    assert!(elapsed.as_secs_f64() < 3.0, "elapsed: {elapsed:?}");
}

#[test]
fn test_incremental_build_reuses_unchanged_files() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("util.rs"), "pub fn util() -> i32 { 1 }\n").unwrap();
    fs::write(
        src.join("lib.rs"),
        "mod util;\npub fn add(a:i32,b:i32)->i32{util::util()+a+b}\n",
    )
    .unwrap();

    let state_file = dir.path().join("codegraph-state.json");
    let input = CodeGraphIncrementalBuildInput {
        build: CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "incremental-smoke".to_string(),
            config: CodeGraphExtractorConfig::default(),
        },
        state_file: state_file.clone(),
    };

    let first = build_code_graph_incremental(&input).unwrap();
    let first_stats = first.incremental.clone().unwrap();
    assert_eq!(first_stats.rebuilt_files, 2);
    assert_eq!(first_stats.reused_files, 0);
    assert_eq!(
        first_stats.full_rebuild_reason.as_deref(),
        Some("missing_state")
    );
    assert!(state_file.exists());

    let second = build_code_graph_incremental(&input).unwrap();
    let second_stats = second.incremental.clone().unwrap();
    assert_eq!(second_stats.rebuilt_files, 0);
    assert_eq!(second_stats.reused_files, 2);
    assert_eq!(second_stats.full_rebuild_reason, None);
    assert_eq!(
        first.canonical_fingerprint, second.canonical_fingerprint,
        "unchanged incremental rebuild should preserve the graph fingerprint"
    );
}

#[test]
fn test_incremental_build_invalidates_dependents_and_deletions() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("helper.rs"), "pub fn helper() -> i32 { 2 }\n").unwrap();
    fs::write(src.join("util.rs"), "pub fn util() -> i32 { 1 }\n").unwrap();
    fs::write(
        src.join("lib.rs"),
        "mod helper;\nmod util;\npub fn add(a:i32,b:i32)->i32{helper::helper()+util::util()+a+b}\n",
    )
    .unwrap();

    let state_file = dir.path().join("codegraph-state.json");
    let input = CodeGraphIncrementalBuildInput {
        build: CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "incremental-invalidations".to_string(),
            config: CodeGraphExtractorConfig::default(),
        },
        state_file: state_file.clone(),
    };

    let baseline = build_code_graph_incremental(&input).unwrap();
    assert_eq!(baseline.stats.file_nodes, 3);

    fs::write(src.join("util.rs"), "pub fn util() -> i32 { 7 }\n").unwrap();
    let changed = build_code_graph_incremental(&input).unwrap();
    let changed_stats = changed.incremental.clone().unwrap();
    assert_eq!(changed_stats.changed_files, 1);
    assert_eq!(changed_stats.rebuilt_files, 2);
    assert_eq!(changed_stats.reused_files, 1);
    assert_eq!(changed_stats.invalidated_files, 2);

    fs::remove_file(src.join("helper.rs")).unwrap();
    fs::write(
        src.join("lib.rs"),
        "mod util;\npub fn add(a:i32,b:i32)->i32{util::util()+a+b}\n",
    )
    .unwrap();
    let deleted = build_code_graph_incremental(&input).unwrap();
    let deleted_stats = deleted.incremental.clone().unwrap();
    assert_eq!(deleted_stats.deleted_files, 1);
    assert_eq!(deleted_stats.changed_files, 1);
    assert_eq!(deleted_stats.rebuilt_files, 1);
    assert_eq!(deleted_stats.reused_files, 1);
    assert_eq!(deleted.stats.file_nodes, 2);
}
