//! Test case definitions for UCL command benchmarking.

use crate::test_document::ids;
use serde::{Deserialize, Serialize};

/// A single test case for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub command_type: String,
    pub description: String,
    pub prompt: String,
    pub expected_pattern: Option<String>,
    pub validation: ValidationCriteria,
    pub document_id: String,
}

/// Criteria for validating test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCriteria {
    pub must_parse: bool,
    pub must_execute: bool,
    pub expected_command: Option<String>,
    pub target_block_id: Option<String>,
    pub forbidden_patterns: Vec<String>,
}

impl Default for ValidationCriteria {
    fn default() -> Self {
        Self {
            must_parse: true,
            must_execute: true,
            expected_command: None,
            target_block_id: None,
            forbidden_patterns: Vec::new(),
        }
    }
}

/// Generate all test cases for the benchmark suite
pub fn generate_test_cases() -> Vec<TestCase> {
    let mut cases = Vec::new();
    cases.extend(edit_test_cases());
    cases.extend(append_test_cases());
    cases.extend(move_test_cases());
    cases.extend(delete_test_cases());
    cases.extend(link_test_cases());
    cases.extend(snapshot_test_cases());
    cases.extend(transaction_test_cases());
    cases
}

const ML_DOC_ID: &str = "ml_tutorial";
const QUICKSTART_DOC_ID: &str = "quickstart_blog";

fn edit_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "edit_simple_text_001".into(),
            command_type: "EDIT".into(),
            description: "Simple text content edit".into(),
            prompt: format!(
                "Edit block {} to change its text content to 'Updated introduction hook.'",
                ids::intro_hook()
            ),
            expected_pattern: Some(r"EDIT\s+blk_000000000011\s+SET".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("EDIT".into()),
                target_block_id: Some(ids::intro_hook().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "edit_json_path_002".into(),
            command_type: "EDIT".into(),
            description: "Edit JSON using path expression".into(),
            prompt: format!(
                "Edit block {} to change the version field to '2.0'",
                ids::metadata()
            ),
            expected_pattern: Some(r"EDIT\s+blk_000000000001.*version".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("EDIT".into()),
                target_block_id: Some(ids::metadata().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "edit_code_block_003".into(),
            command_type: "EDIT".into(),
            description: "Edit code block content".into(),
            prompt: format!(
                "Replace the content of code block {} with '# ML Setup\\nimport pandas as pd'",
                ids::section1_code()
            ),
            expected_pattern: Some(r"EDIT\s+blk_000000000024".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("EDIT".into()),
                target_block_id: Some(ids::section1_code().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "edit_append_text_004".into(),
            command_type: "EDIT".into(),
            description: "Append text to existing content".into(),
            prompt: format!(
                "Append ' Learn more in the next section.' to the text in block {}",
                ids::intro_thesis()
            ),
            expected_pattern: Some(r"EDIT\s+blk_000000000013.*\+=".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("EDIT".into()),
                target_block_id: Some(ids::intro_thesis().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn append_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "append_text_block_001".into(),
            command_type: "APPEND".into(),
            description: "Append a new text block".into(),
            prompt: format!(
                "Add a new text block with content 'Additional context paragraph.' as a child of block {}",
                ids::intro()
            ),
            expected_pattern: Some(r"APPEND\s+blk_000000000010\s+text".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("APPEND".into()),
                target_block_id: Some(ids::intro().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "append_code_block_002".into(),
            command_type: "APPEND".into(),
            description: "Append a code block with language".into(),
            prompt: format!(
                "Add a new Python code block with 'print(\"Hello ML\")' to section {}",
                ids::section1()
            ),
            expected_pattern: Some(r"APPEND\s+blk_000000000021\s+code".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("APPEND".into()),
                target_block_id: Some(ids::section1().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "append_at_index_003".into(),
            command_type: "APPEND".into(),
            description: "Append block at specific index".into(),
            prompt: format!(
                "Add a new text block 'Important note:' as the first child (index 0) of block {}",
                ids::section1()
            ),
            expected_pattern: Some(r"APPEND.*AT\s+0".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("APPEND".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "append_with_label_004".into(),
            command_type: "APPEND".into(),
            description: "Append block with label property".into(),
            prompt: format!(
                "Add a new text block 'Summary point' with label='key-takeaway' to {}",
                ids::conclusion()
            ),
            expected_pattern: Some(r"APPEND.*label".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("APPEND".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn move_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "move_to_parent_001".into(),
            command_type: "MOVE".into(),
            description: "Move block to new parent".into(),
            prompt: format!(
                "Move block {} to be a child of {}",
                ids::section3_list(), ids::section1()
            ),
            expected_pattern: Some(r"MOVE\s+blk_000000000043\s+TO\s+blk_000000000021".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("MOVE".into()),
                target_block_id: Some(ids::section3_list().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "move_before_sibling_002".into(),
            command_type: "MOVE".into(),
            description: "Move block before sibling".into(),
            prompt: format!(
                "Move block {} to be before block {}",
                ids::section1_table(), ids::section1_code()
            ),
            expected_pattern: Some(r"MOVE.*BEFORE".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("MOVE".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "move_after_sibling_003".into(),
            command_type: "MOVE".into(),
            description: "Move block after sibling".into(),
            prompt: format!(
                "Move block {} to be after block {}",
                ids::intro_hook(), ids::intro_thesis()
            ),
            expected_pattern: Some(r"MOVE.*AFTER".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("MOVE".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn delete_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "delete_single_001".into(),
            command_type: "DELETE".into(),
            description: "Delete a single block".into(),
            prompt: format!(
                "Delete the block {}",
                ids::section3_list()
            ),
            expected_pattern: Some(r"DELETE\s+blk_000000000043".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("DELETE".into()),
                target_block_id: Some(ids::section3_list().to_string()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "delete_cascade_002".into(),
            command_type: "DELETE".into(),
            description: "Delete block with CASCADE".into(),
            prompt: format!(
                "Delete block {} and all its children (cascade)",
                ids::section2()
            ),
            expected_pattern: Some(r"DELETE.*CASCADE".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("DELETE".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "delete_preserve_003".into(),
            command_type: "DELETE".into(),
            description: "Delete block preserving children".into(),
            prompt: format!(
                "Delete block {} but preserve its children (move them to parent)",
                ids::section1()
            ),
            expected_pattern: Some(r"DELETE.*PRESERVE".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("DELETE".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn link_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "link_references_001".into(),
            command_type: "LINK".into(),
            description: "Create a references link".into(),
            prompt: format!(
                "Create a 'references' link from {} to {}",
                ids::section2_math(), ids::references()
            ),
            expected_pattern: Some(r"LINK\s+blk_000000000033\s+references\s+blk_000000000060".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("LINK".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "link_elaborates_002".into(),
            command_type: "LINK".into(),
            description: "Create an elaborates link".into(),
            prompt: format!(
                "Create an 'elaborates' link from {} to {}",
                ids::section1_code(), ids::section1_para()
            ),
            expected_pattern: Some(r"LINK.*elaborates".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("LINK".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "unlink_001".into(),
            command_type: "UNLINK".into(),
            description: "Remove an existing link".into(),
            prompt: format!(
                "Remove the 'references' link from {} to {}",
                ids::section2_math(), ids::references()
            ),
            expected_pattern: Some(r"UNLINK".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("UNLINK".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn snapshot_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "snapshot_create_001".into(),
            command_type: "SNAPSHOT".into(),
            description: "Create a named snapshot".into(),
            prompt: "Create a snapshot named 'before-edit' with description 'State before major edits'".into(),
            expected_pattern: Some(r#"SNAPSHOT\s+CREATE\s+"before-edit""#.into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("SNAPSHOT".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "snapshot_restore_002".into(),
            command_type: "SNAPSHOT".into(),
            description: "Restore from snapshot".into(),
            prompt: "Restore the document to snapshot 'before-edit'".into(),
            expected_pattern: Some(r#"SNAPSHOT\s+RESTORE\s+"before-edit""#.into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("SNAPSHOT".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "snapshot_list_003".into(),
            command_type: "SNAPSHOT".into(),
            description: "List all snapshots".into(),
            prompt: "List all available snapshots".into(),
            expected_pattern: Some(r"SNAPSHOT\s+LIST".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("SNAPSHOT".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

fn transaction_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            id: "transaction_atomic_001".into(),
            command_type: "ATOMIC".into(),
            description: "Atomic block with multiple commands".into(),
            prompt: format!(
                "Create an atomic block that: 1) edits {} to say 'New hook', 2) appends a text block 'Follow up' to {}",
                ids::intro_hook(), ids::intro()
            ),
            expected_pattern: Some(r"ATOMIC\s*\{".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                expected_command: Some("ATOMIC".into()),
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
        TestCase {
            id: "transaction_begin_commit_002".into(),
            command_type: "TRANSACTION".into(),
            description: "Begin and commit transaction".into(),
            prompt: "Begin a transaction named 'bulk-update', then commit it".into(),
            expected_pattern: Some(r"BEGIN\s+TRANSACTION".into()),
            validation: ValidationCriteria {
                must_parse: true,
                must_execute: true,
                ..Default::default()
            },
            document_id: ML_DOC_ID.into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_cases() {
        let cases = generate_test_cases();
        assert!(cases.len() >= 20);

        let command_types: Vec<_> = cases.iter().map(|c| c.command_type.as_str()).collect();
        assert!(command_types.contains(&"EDIT"));
        assert!(command_types.contains(&"APPEND"));
        assert!(command_types.contains(&"MOVE"));
        assert!(command_types.contains(&"DELETE"));
        assert!(command_types.contains(&"LINK"));
        assert!(command_types.contains(&"SNAPSHOT"));
    }

    #[test]
    fn test_unique_ids() {
        let cases = generate_test_cases();
        let ids: std::collections::HashSet<_> = cases.iter().map(|c| &c.id).collect();
        assert_eq!(ids.len(), cases.len(), "All test case IDs must be unique");
    }
}
