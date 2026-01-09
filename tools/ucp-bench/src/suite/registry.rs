//! Test registry for managing test cases and categories.

use super::category::{build_standard_categories, TestCategory, TestCategoryId};
use crate::test_cases::{generate_test_cases, TestCase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry of all available test categories and cases
#[derive(Debug, Clone)]
pub struct TestRegistry {
    /// All registered categories
    categories: HashMap<TestCategoryId, TestCategory>,
    /// Test cases grouped by category
    test_cases: HashMap<TestCategoryId, Vec<TestCase>>,
}

impl TestRegistry {
    /// Create a new registry with standard categories and test cases
    pub fn new() -> Self {
        let mut registry = Self {
            categories: HashMap::new(),
            test_cases: HashMap::new(),
        };

        // Register standard categories
        for category in build_standard_categories() {
            registry.categories.insert(category.id.clone(), category);
        }

        // Load and categorize test cases
        let all_cases = generate_test_cases();
        for case in all_cases {
            let category_id = TestCategoryId::new(&case.command_type);
            registry
                .test_cases
                .entry(category_id)
                .or_insert_with(Vec::new)
                .push(case);
        }

        // Update test counts
        for (id, cases) in &registry.test_cases {
            if let Some(category) = registry.categories.get_mut(id) {
                category.test_count = cases.len() as u32;
            }
        }

        registry
    }

    /// Get all registered categories
    pub fn categories(&self) -> impl Iterator<Item = &TestCategory> {
        self.categories.values()
    }

    /// Get a specific category
    pub fn get_category(&self, id: &TestCategoryId) -> Option<&TestCategory> {
        self.categories.get(id)
    }

    /// Get test cases for a category
    pub fn get_tests(&self, category_id: &TestCategoryId) -> Option<&Vec<TestCase>> {
        self.test_cases.get(category_id)
    }

    /// Get all test cases for multiple categories
    pub fn get_tests_for_categories(&self, category_ids: &[TestCategoryId]) -> Vec<&TestCase> {
        category_ids
            .iter()
            .filter_map(|id| self.test_cases.get(id))
            .flatten()
            .collect()
    }

    /// Get all test cases
    pub fn all_tests(&self) -> Vec<&TestCase> {
        self.test_cases.values().flatten().collect()
    }

    /// Find test case by ID
    pub fn find_test(&self, test_id: &str) -> Option<TestCase> {
        for cases in self.test_cases.values() {
            if let Some(case) = cases.iter().find(|c| c.id == test_id) {
                return Some(case.clone());
            }
        }
        None
    }

    /// Get total number of test cases
    pub fn total_test_count(&self) -> usize {
        self.test_cases.values().map(|v| v.len()).sum()
    }

    /// Register a custom category
    pub fn register_category(&mut self, category: TestCategory) {
        self.categories.insert(category.id.clone(), category);
    }

    /// Register custom test cases for a category
    pub fn register_tests(&mut self, category_id: TestCategoryId, cases: Vec<TestCase>) {
        self.test_cases.insert(category_id, cases);
    }

    /// Get a summary of the registry
    pub fn summary(&self) -> RegistrySummary {
        RegistrySummary {
            total_categories: self.categories.len(),
            total_tests: self.total_test_count(),
            categories: self
                .categories
                .values()
                .map(|c| CategorySummary {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    test_count: c.test_count,
                    complexity: c.complexity,
                })
                .collect(),
        }
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of the test registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySummary {
    pub total_categories: usize,
    pub total_tests: usize,
    pub categories: Vec<CategorySummary>,
}

/// Summary of a single category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub id: TestCategoryId,
    pub name: String,
    pub test_count: u32,
    pub complexity: u8,
}
