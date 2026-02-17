use std::path::PathBuf;

use ucp_api::{
    build_code_graph, canonical_fingerprint, validate_code_graph_profile, CodeGraphBuildInput,
    CodeGraphBuildStatus, CodeGraphExtractorConfig, CodeGraphSeverity, PortableDocument,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn single_language_fixture_builds_valid_profile() {
    let result = build_code_graph(&CodeGraphBuildInput {
        repository_path: fixture_path("single-language-rust"),
        commit_hash: "fixture-single".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .expect("build should succeed");

    assert!(result.stats.file_nodes >= 2);
    assert!(result.stats.symbol_nodes >= 2);
    assert_eq!(result.stats.languages.get("rust").copied(), Some(2));

    let validation = validate_code_graph_profile(&result.document);
    assert!(
        validation.valid,
        "diagnostics: {:?}",
        validation.diagnostics
    );
    assert!(matches!(
        result.status,
        CodeGraphBuildStatus::Success | CodeGraphBuildStatus::PartialSuccess
    ));
}

#[test]
fn multi_language_fixture_tracks_languages_and_determinism() {
    let input = CodeGraphBuildInput {
        repository_path: fixture_path("multi-language"),
        commit_hash: "fixture-multi".to_string(),
        config: CodeGraphExtractorConfig::default(),
    };

    let first = build_code_graph(&input).expect("first build");
    let second = build_code_graph(&input).expect("second build");

    assert_eq!(first.canonical_fingerprint, second.canonical_fingerprint);
    assert!(first.stats.languages.contains_key("rust"));
    assert!(first.stats.languages.contains_key("python"));
    assert!(first.stats.languages.contains_key("typescript"));

    let fp_first = canonical_fingerprint(&first.document).unwrap();
    let fp_second = canonical_fingerprint(&second.document).unwrap();
    assert_eq!(fp_first, fp_second);
}

#[test]
fn edge_case_fixture_reports_unresolved_import_diagnostics() {
    let result = build_code_graph(&CodeGraphBuildInput {
        repository_path: fixture_path("edge-cases"),
        commit_hash: "fixture-edge".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .expect("build should succeed with diagnostics");

    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.code == "CG2006" && d.severity == CodeGraphSeverity::Warning));
    assert!(matches!(
        result.status,
        CodeGraphBuildStatus::PartialSuccess | CodeGraphBuildStatus::FailedValidation
    ));
}

#[test]
fn portable_document_roundtrip_for_fixture_preserves_fingerprint() {
    let result = build_code_graph(&CodeGraphBuildInput {
        repository_path: fixture_path("single-language-rust"),
        commit_hash: "fixture-roundtrip".to_string(),
        config: CodeGraphExtractorConfig::default(),
    })
    .expect("build should succeed");

    let portable = PortableDocument::from_document(&result.document);
    let encoded = serde_json::to_string_pretty(&portable).expect("encode portable");
    let decoded: PortableDocument = serde_json::from_str(&encoded).expect("decode portable");
    let rebuilt = decoded.to_document().expect("to document");

    let original_fp = canonical_fingerprint(&result.document).expect("fingerprint original");
    let rebuilt_fp = canonical_fingerprint(&rebuilt).expect("fingerprint rebuilt");

    assert_eq!(original_fp, rebuilt_fp);
}
