//! Test category definitions for modular benchmark organization.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Identifier for a test category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestCategoryId(pub String);

impl TestCategoryId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TestCategoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TestCategoryId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for TestCategoryId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Standard test categories
pub mod categories {
    use super::TestCategoryId;

    pub fn edit() -> TestCategoryId { TestCategoryId::new("EDIT") }
    pub fn append() -> TestCategoryId { TestCategoryId::new("APPEND") }
    pub fn delete() -> TestCategoryId { TestCategoryId::new("DELETE") }
    pub fn move_block() -> TestCategoryId { TestCategoryId::new("MOVE") }
    pub fn link() -> TestCategoryId { TestCategoryId::new("LINK") }
    pub fn unlink() -> TestCategoryId { TestCategoryId::new("UNLINK") }
    pub fn snapshot() -> TestCategoryId { TestCategoryId::new("SNAPSHOT") }
    pub fn transaction() -> TestCategoryId { TestCategoryId::new("TRANSACTION") }
    pub fn atomic() -> TestCategoryId { TestCategoryId::new("ATOMIC") }

    pub fn all() -> Vec<TestCategoryId> {
        vec![
            edit(),
            append(),
            delete(),
            move_block(),
            link(),
            unlink(),
            snapshot(),
            transaction(),
            atomic(),
        ]
    }
}

/// A test category with metadata and test case generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCategory {
    /// Unique identifier
    pub id: TestCategoryId,
    /// Human-readable name
    pub name: String,
    /// Description of what this category tests
    pub description: String,
    /// UCL command type this category tests
    pub command_type: String,
    /// Tags for filtering/grouping
    pub tags: Vec<String>,
    /// Whether this category is enabled by default
    pub enabled_by_default: bool,
    /// Estimated complexity (1-5)
    pub complexity: u8,
    /// Number of test cases in this category
    pub test_count: u32,
}

impl TestCategory {
    pub fn new(id: impl Into<TestCategoryId>, name: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            command_type: id.0.clone(),
            id,
            name: name.into(),
            description: String::new(),
            tags: Vec::new(),
            enabled_by_default: true,
            complexity: 1,
            test_count: 0,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_complexity(mut self, complexity: u8) -> Self {
        self.complexity = complexity.min(5).max(1);
        self
    }

    pub fn with_test_count(mut self, count: u32) -> Self {
        self.test_count = count;
        self
    }

    pub fn disabled_by_default(mut self) -> Self {
        self.enabled_by_default = false;
        self
    }
}

/// Builder for creating standard test categories
pub fn build_standard_categories() -> Vec<TestCategory> {
    vec![
        TestCategory::new(categories::edit(), "Edit Commands")
            .with_description("Test EDIT commands for modifying block content")
            .with_tags(vec!["content".into(), "modification".into()])
            .with_complexity(2)
            .with_test_count(4),

        TestCategory::new(categories::append(), "Append Commands")
            .with_description("Test APPEND commands for adding new blocks")
            .with_tags(vec!["content".into(), "creation".into()])
            .with_complexity(2)
            .with_test_count(4),

        TestCategory::new(categories::delete(), "Delete Commands")
            .with_description("Test DELETE commands for removing blocks")
            .with_tags(vec!["content".into(), "deletion".into()])
            .with_complexity(2)
            .with_test_count(3),

        TestCategory::new(categories::move_block(), "Move Commands")
            .with_description("Test MOVE commands for relocating blocks")
            .with_tags(vec!["structure".into(), "reorganization".into()])
            .with_complexity(3)
            .with_test_count(3),

        TestCategory::new(categories::link(), "Link Commands")
            .with_description("Test LINK commands for creating relationships")
            .with_tags(vec!["relationships".into(), "edges".into()])
            .with_complexity(2)
            .with_test_count(2),

        TestCategory::new(categories::unlink(), "Unlink Commands")
            .with_description("Test UNLINK commands for removing relationships")
            .with_tags(vec!["relationships".into(), "edges".into()])
            .with_complexity(2)
            .with_test_count(2),

        TestCategory::new(categories::snapshot(), "Snapshot Commands")
            .with_description("Test SNAPSHOT commands for version control")
            .with_tags(vec!["versioning".into(), "state".into()])
            .with_complexity(3)
            .with_test_count(3),

        TestCategory::new(categories::transaction(), "Transaction Commands")
            .with_description("Test transaction control commands (BEGIN, COMMIT, ROLLBACK)")
            .with_tags(vec!["transactions".into(), "atomicity".into()])
            .with_complexity(4)
            .with_test_count(1),

        TestCategory::new(categories::atomic(), "Atomic Commands")
            .with_description("Test ATOMIC blocks for grouped operations")
            .with_tags(vec!["transactions".into(), "atomicity".into()])
            .with_complexity(4)
            .with_test_count(1),
    ]
}
