use tree_sitter::{Language, Parser};

use crate::model::*;

use super::{
    extract_file_description,
    languages::{python::analyze_python_tree, rust::analyze_rust_tree, ts_js::analyze_ts_tree},
};

pub(super) fn analyze_file(path: &str, source: &str, language: CodeLanguage) -> FileAnalysis {
    let mut analysis = FileAnalysis {
        file_description: extract_file_description(source, language),
        ..Default::default()
    };
    let mut parser = Parser::new();
    let tree_sitter_language = language_for(language);
    if parser.set_language(&tree_sitter_language).is_err() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::error(
                "CG2010",
                format!(
                    "failed to initialize tree-sitter parser for {}",
                    language.as_str()
                ),
            )
            .with_path(path.to_string()),
        );
        return analysis;
    }

    let Some(tree) = parser.parse(source, None) else {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::error("CG2011", "tree-sitter returned no parse tree")
                .with_path(path.to_string()),
        );
        return analysis;
    };

    let root = tree.root_node();
    if root.has_error() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::warning(
                "CG2002",
                "tree-sitter parser reported syntax errors; extraction continues",
            )
            .with_path(path.to_string()),
        );
    }

    match language {
        CodeLanguage::Rust => analyze_rust_tree(source, root, &mut analysis),
        CodeLanguage::Python => analyze_python_tree(path, source, root, &mut analysis),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            analyze_ts_tree(source, root, &mut analysis)
        }
    }

    if analysis.symbols.is_empty() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::info("CG2001", format!("no symbols extracted for {}", path))
                .with_path(path.to_string()),
        );
    }

    analysis
}

pub(super) fn language_for(language: CodeLanguage) -> Language {
    match language {
        CodeLanguage::Rust => tree_sitter_rust::LANGUAGE.into(),
        CodeLanguage::Python => tree_sitter_python::LANGUAGE.into(),
        CodeLanguage::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        CodeLanguage::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
    }
}

pub(super) fn is_python_package_init(path: &str) -> bool {
    path == "__init__.py" || path.ends_with("/__init__.py")
}
