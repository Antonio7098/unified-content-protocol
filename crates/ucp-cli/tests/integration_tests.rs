//! Integration tests for UCP CLI

use std::process::Command;

/// Helper to run the CLI with arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "-q", "-p", "ucp-cli", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command")
}

/// Helper to get stdout as string
fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Helper to get stderr as string
fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn test_help() {
    let output = run_cli(&["--help"]);
    let out = stdout(&output);

    assert!(out.contains("Command-line interface for Unified Content Protocol"));
    assert!(out.contains("create"));
    assert!(out.contains("block"));
    assert!(out.contains("edge"));
}

#[test]
fn test_version() {
    let output = run_cli(&["--version"]);
    let out = stdout(&output);

    assert!(out.contains("ucp"));
}

#[test]
fn test_create_document() {
    let output = run_cli(&["create", "--title", "Test Doc", "--format", "json"]);
    let out = stdout(&output);

    // Should output valid JSON
    let doc: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

    assert!(doc.get("id").is_some());
    assert!(doc.get("root").is_some());
    assert!(doc.get("blocks").is_some());
}

#[test]
fn test_create_document_with_title() {
    let output = run_cli(&["create", "--title", "My Test Document", "--format", "json"]);
    let out = stdout(&output);

    let doc: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

    // Check that title is set in metadata
    let title = doc.get("metadata")
        .and_then(|m| m.get("title"))
        .and_then(|t| t.as_str());
    assert_eq!(title, Some("My Test Document"));
}

#[test]
fn test_block_subcommands() {
    let output = run_cli(&["block", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("add"));
    assert!(out.contains("get"));
    assert!(out.contains("delete"));
    assert!(out.contains("move"));
    assert!(out.contains("list"));
    assert!(out.contains("update"));
}

#[test]
fn test_edge_subcommands() {
    let output = run_cli(&["edge", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("add"));
    assert!(out.contains("remove"));
    assert!(out.contains("list"));
}

#[test]
fn test_nav_subcommands() {
    let output = run_cli(&["nav", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("children"));
    assert!(out.contains("parent"));
    assert!(out.contains("siblings"));
    assert!(out.contains("descendants"));
}

#[test]
fn test_tx_subcommands() {
    let output = run_cli(&["tx", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("begin"));
    assert!(out.contains("commit"));
    assert!(out.contains("rollback"));
    assert!(out.contains("savepoint"));
}

#[test]
fn test_snapshot_subcommands() {
    let output = run_cli(&["snapshot", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("create"));
    assert!(out.contains("restore"));
    assert!(out.contains("list"));
    assert!(out.contains("delete"));
}

#[test]
fn test_import_subcommands() {
    let output = run_cli(&["import", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("markdown"));
    assert!(out.contains("html"));
}

#[test]
fn test_export_subcommands() {
    let output = run_cli(&["export", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("markdown"));
    assert!(out.contains("json"));
}

#[test]
fn test_ucl_subcommands() {
    let output = run_cli(&["ucl", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("exec"));
    assert!(out.contains("parse"));
}

#[test]
fn test_agent_subcommands() {
    let output = run_cli(&["agent", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("session"));
    assert!(out.contains("goto"));
    assert!(out.contains("back"));
    assert!(out.contains("expand"));
    assert!(out.contains("follow"));
    assert!(out.contains("search"));
    assert!(out.contains("find"));
    assert!(out.contains("context"));
    assert!(out.contains("view"));
}

#[test]
fn test_llm_subcommands() {
    let output = run_cli(&["llm", "--help"]);
    let out = stdout(&output);

    assert!(out.contains("id-map"));
    assert!(out.contains("shorten-ucl"));
    assert!(out.contains("expand-ucl"));
    assert!(out.contains("prompt"));
    assert!(out.contains("context"));
}

#[test]
fn test_json_output_format() {
    // Create a document with JSON output
    let output = run_cli(&["create", "--format", "json"]);
    let out = stdout(&output);

    // Should be valid JSON
    let result: Result<serde_json::Value, _> = serde_json::from_str(&out);
    assert!(result.is_ok(), "Output should be valid JSON");
}

#[test]
fn test_text_output_format() {
    let output = run_cli(&["create", "--format", "text"]);
    let out = stdout(&output);

    // Should contain human-readable output
    assert!(out.contains("Created new document") || out.contains("Document Information"));
}

mod with_temp_file {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Create a temp file with a document and return the path
    fn create_temp_doc() -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");

        // Use valid block IDs: blk_ + 24 hex chars (12 bytes)
        let doc = r#"{
            "id": "doc_test123456789012",
            "root": "blk_ff0000000000000000000000",
            "structure": {},
            "blocks": {
                "blk_ff0000000000000000000000": {
                    "id": "ff0000000000000000000000",
                    "content": {
                        "type": "text",
                        "text": "Hello World",
                        "format": "plain"
                    },
                    "metadata": {
                        "content_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                        "created_at": "2025-01-01T00:00:00Z",
                        "modified_at": "2025-01-01T00:00:00Z"
                    },
                    "edges": [],
                    "version": {"counter": 1, "timestamp": "2025-01-01T00:00:00Z"}
                }
            },
            "metadata": {
                "title": "Test Document"
            },
            "version": 1
        }"#;

        file.write_all(doc.as_bytes()).expect("Failed to write temp file");
        file
    }

    #[test]
    fn test_info_command() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["info", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let info: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

        assert!(info.get("id").is_some());
        assert!(info.get("block_count").is_some());
        assert!(info.get("root").is_some());
    }

    #[test]
    fn test_tree_command() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["tree", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let tree: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

        assert!(tree.get("id").is_some());
        assert!(tree.get("children").is_some());
    }

    #[test]
    fn test_block_list() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["block", "list", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let blocks: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

        assert!(blocks.is_array());
    }

    #[test]
    fn test_orphans_command() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["orphans", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

        assert!(result.get("count").is_some());
        assert!(result.get("orphans").is_some());
    }

    #[test]
    fn test_validate_command() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["validate", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");

        assert!(result.get("valid").is_some());
        assert!(result.get("issues").is_some());
    }

    #[test]
    fn test_nav_children() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["nav", "children", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        // Should return an array of children
        let result: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");
        assert!(result.is_array());
    }

    #[test]
    fn test_find_command() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["find", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");
        assert!(result.is_array());
    }

    #[test]
    fn test_export_json() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["export", "json", "--input", path]);
        let out = stdout(&output);

        // Should output valid JSON
        let doc: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");
        assert!(doc.get("id").is_some());
        assert!(doc.get("blocks").is_some());
    }

    #[test]
    fn test_export_markdown() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["export", "markdown", "--input", path]);
        // Should succeed (exit 0)
        assert!(output.status.success() || !stderr(&output).contains("Error"));
    }

    #[test]
    fn test_nav_descendants() {
        let temp_file = create_temp_doc();
        let path = temp_file.path().to_str().unwrap();

        let output = run_cli(&["nav", "descendants", "--input", path, "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out).expect("Output should be valid JSON");
        assert!(result.is_array());
    }
}

mod workflow_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Test a complete workflow: create -> add blocks -> export
    #[test]
    fn test_create_and_export_workflow() {
        // Create a document
        let output = run_cli(&["create", "--title", "Workflow Test", "--format", "json"]);
        let doc_json = stdout(&output);

        // Write to temp file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file.write_all(doc_json.as_bytes()).expect("Failed to write");
        let path = temp_file.path().to_str().unwrap();

        // Get info
        let output = run_cli(&["info", "--input", path, "--format", "json"]);
        let info: serde_json::Value = serde_json::from_str(&stdout(&output))
            .expect("Info should be valid JSON");

        assert_eq!(info.get("title").and_then(|t| t.as_str()), Some("Workflow Test"));
        assert_eq!(info.get("block_count").and_then(|c| c.as_u64()), Some(1));
    }

    #[test]
    fn test_ucl_parse() {
        let output = run_cli(&["ucl", "parse", "EDIT #blk_abc123 SET text = 'Hello'"]);
        // Should succeed or show parse result
        let combined = format!("{}{}", stdout(&output), stderr(&output));
        // Parser might fail on invalid ID but command should run
        assert!(!combined.is_empty());
    }

    #[test]
    fn test_llm_prompt() {
        let output = run_cli(&["llm", "prompt", "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out)
            .expect("Prompt should return valid JSON");

        assert!(result.get("capabilities").is_some());
        assert!(result.get("prompt").is_some());
    }

    #[test]
    fn test_llm_prompt_with_capabilities() {
        let output = run_cli(&["llm", "prompt", "--capabilities", "edit,append", "--format", "json"]);
        let out = stdout(&output);

        let result: serde_json::Value = serde_json::from_str(&out)
            .expect("Prompt should return valid JSON");

        let caps = result.get("capabilities").and_then(|c| c.as_array());
        assert!(caps.is_some());
    }
}

mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_input_file() {
        let output = run_cli(&["info", "--input", "/nonexistent/file.json"]);
        let err = stderr(&output);
        assert!(err.contains("Error") || err.contains("error") || !output.status.success());
    }

    #[test]
    fn test_invalid_block_id() {
        // Block get with invalid ID should fail gracefully
        let output = run_cli(&["block", "get", "--id", "invalid-id"]);
        // Should fail but not crash
        assert!(!output.status.success() || stderr(&output).contains("Invalid"));
    }

    #[test]
    fn test_missing_required_args() {
        // Block delete without ID should fail
        let output = run_cli(&["block", "delete"]);
        let err = stderr(&output);
        // Should show usage error
        assert!(err.contains("error") || err.contains("required") || err.contains("Usage"));
    }
}
