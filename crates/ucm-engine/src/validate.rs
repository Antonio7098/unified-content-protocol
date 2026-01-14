//! Validation pipeline for documents and operations.

use ucm_core::{
    Block, BlockId, Document, Error, ErrorCode, Result, ValidationIssue, ValidationSeverity,
};

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            issues: Vec::new(),
        }
    }

    pub fn invalid(issues: Vec<ValidationIssue>) -> Self {
        let has_errors = issues
            .iter()
            .any(|i| i.severity == ValidationSeverity::Error);
        Self {
            valid: !has_errors,
            issues,
        }
    }

    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .collect()
    }

    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .collect()
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.issues.extend(other.issues);
        self.valid = self.valid && other.valid;
    }
}

/// Resource limits for validation
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_document_size: usize,
    pub max_block_count: usize,
    pub max_block_size: usize,
    pub max_nesting_depth: usize,
    pub max_edges_per_block: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_document_size: 50 * 1024 * 1024, // 50MB
            max_block_count: 100_000,
            max_block_size: 5 * 1024 * 1024, // 5MB
            max_nesting_depth: 50,
            max_edges_per_block: 1000,
        }
    }
}

/// Validation pipeline
#[derive(Debug, Clone)]
pub struct ValidationPipeline {
    limits: ResourceLimits,
}

impl ValidationPipeline {
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Validate a document
    pub fn validate_document(&self, doc: &Document) -> ValidationResult {
        let mut result = ValidationResult::valid();

        // Check block count
        if doc.block_count() > self.limits.max_block_count {
            result.issues.push(ValidationIssue::error(
                ErrorCode::E400DocumentSizeExceeded,
                format!(
                    "Document has {} blocks, maximum is {}",
                    doc.block_count(),
                    self.limits.max_block_count
                ),
            ));
            result.valid = false;
        }

        // Validate structure
        result.merge(self.validate_structure(doc));

        // Validate each block
        for block in doc.blocks.values() {
            result.merge(self.validate_block(block, doc));
        }

        // Check for orphans (warning)
        let orphans = doc.find_orphans();
        for orphan in orphans {
            result.issues.push(ValidationIssue::warning(
                ErrorCode::E203OrphanedBlock,
                format!("Block {} is unreachable from root", orphan),
            ));
        }

        result
    }

    /// Validate document structure
    fn validate_structure(&self, doc: &Document) -> ValidationResult {
        let mut issues = Vec::new();

        // Check for cycles
        if self.has_cycles(doc) {
            issues.push(ValidationIssue::error(
                ErrorCode::E201CycleDetected,
                "Document structure contains a cycle",
            ));
        }

        // Check nesting depth
        let max_depth = self.compute_max_depth(doc);
        if max_depth > self.limits.max_nesting_depth {
            issues.push(ValidationIssue::error(
                ErrorCode::E403NestingDepthExceeded,
                format!(
                    "Maximum nesting depth is {}, document has {}",
                    self.limits.max_nesting_depth, max_depth
                ),
            ));
        }

        // Check that all referenced blocks exist
        for (parent, children) in &doc.structure {
            if !doc.blocks.contains_key(parent) {
                issues.push(ValidationIssue::error(
                    ErrorCode::E001BlockNotFound,
                    format!("Structure references non-existent block {}", parent),
                ));
            }
            for child in children {
                if !doc.blocks.contains_key(child) {
                    issues.push(ValidationIssue::error(
                        ErrorCode::E001BlockNotFound,
                        format!("Structure references non-existent child block {}", child),
                    ));
                }
            }
        }

        ValidationResult::invalid(issues)
    }

    /// Validate a single block
    fn validate_block(&self, block: &Block, doc: &Document) -> ValidationResult {
        let mut issues = Vec::new();

        // Check block size
        let size = block.size_bytes();
        if size > self.limits.max_block_size {
            issues.push(ValidationIssue::error(
                ErrorCode::E402BlockSizeExceeded,
                format!(
                    "Block {} has size {} bytes, maximum is {}",
                    block.id, size, self.limits.max_block_size
                ),
            ));
        }

        // Check edge count
        if block.edges.len() > self.limits.max_edges_per_block {
            issues.push(ValidationIssue::error(
                ErrorCode::E404EdgeCountExceeded,
                format!(
                    "Block {} has {} edges, maximum is {}",
                    block.id,
                    block.edges.len(),
                    self.limits.max_edges_per_block
                ),
            ));
        }

        // Check edge targets exist
        for edge in &block.edges {
            if !doc.blocks.contains_key(&edge.target) {
                issues.push(ValidationIssue::error(
                    ErrorCode::E001BlockNotFound,
                    format!(
                        "Block {} has edge to non-existent block {}",
                        block.id, edge.target
                    ),
                ));
            }
        }

        ValidationResult::invalid(issues)
    }

    /// Check for cycles in document structure
    fn has_cycles(&self, doc: &Document) -> bool {
        use std::collections::HashSet;

        fn dfs(
            node: &BlockId,
            structure: &std::collections::HashMap<BlockId, Vec<BlockId>>,
            visited: &mut HashSet<BlockId>,
            rec_stack: &mut HashSet<BlockId>,
        ) -> bool {
            visited.insert(*node);
            rec_stack.insert(*node);

            if let Some(children) = structure.get(node) {
                for child in children {
                    if !visited.contains(child) {
                        if dfs(child, structure, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(child) {
                        return true;
                    }
                }
            }

            rec_stack.remove(node);
            false
        }

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        dfs(&doc.root, &doc.structure, &mut visited, &mut rec_stack)
    }

    /// Compute maximum nesting depth
    fn compute_max_depth(&self, doc: &Document) -> usize {
        fn depth_from(
            node: &BlockId,
            structure: &std::collections::HashMap<BlockId, Vec<BlockId>>,
            current_depth: usize,
        ) -> usize {
            let children = structure.get(node);
            match children {
                None => current_depth,
                Some(v) if v.is_empty() => current_depth,
                Some(children) => children
                    .iter()
                    .map(|c| depth_from(c, structure, current_depth + 1))
                    .max()
                    .unwrap_or(current_depth),
            }
        }

        depth_from(&doc.root, &doc.structure, 1)
    }

    /// Validate a block ID format
    pub fn validate_block_id(&self, id: &str) -> Result<BlockId> {
        id.parse::<BlockId>().map_err(|_| {
            Error::new(
                ErrorCode::E002InvalidBlockId,
                format!("Invalid block ID: {}", id),
            )
        })
    }
}

impl Default for ValidationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::Content;

    #[test]
    fn test_valid_document() {
        let validator = ValidationPipeline::new();
        let doc = Document::create();

        let result = validator.validate_document(&doc);
        assert!(result.valid);
    }

    #[test]
    fn test_orphan_detection() {
        let validator = ValidationPipeline::new();
        let mut doc = Document::create();

        let root = doc.root;
        let id = doc
            .add_block(Block::new(Content::text("Test"), None), &root)
            .unwrap();
        doc.remove_from_structure(&id);

        let result = validator.validate_document(&doc);
        assert!(result.valid); // Orphans are warnings, not errors
        assert!(!result.warnings().is_empty());
    }

    #[test]
    fn test_block_size_limit() {
        let validator = ValidationPipeline::with_limits(ResourceLimits {
            max_block_size: 10, // Very small limit
            ..Default::default()
        });

        let mut doc = Document::create();
        let root = doc.root;
        doc.add_block(
            Block::new(Content::text("This is longer than 10 bytes"), None),
            &root,
        )
        .unwrap();

        let result = validator.validate_document(&doc);
        assert!(!result.valid);
    }
}
