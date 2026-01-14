//! Dynamic prompt builder for LLM agents.
//!
//! Builds prompts based on specified capabilities so LLMs generate valid UCL.

use std::collections::HashSet;

/// UCL command capabilities that can be enabled for an agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UclCapability {
    /// EDIT command - modify block content
    Edit,
    /// APPEND command - add new blocks
    Append,
    /// MOVE command - relocate blocks
    Move,
    /// DELETE command - remove blocks
    Delete,
    /// LINK/UNLINK commands - manage relationships
    Link,
    /// SNAPSHOT commands - version control
    Snapshot,
    /// TRANSACTION/ATOMIC commands - atomic operations
    Transaction,
}

impl UclCapability {
    /// Get all available capabilities
    pub fn all() -> Vec<Self> {
        vec![
            Self::Edit,
            Self::Append,
            Self::Move,
            Self::Delete,
            Self::Link,
            Self::Snapshot,
            Self::Transaction,
        ]
    }

    /// Get the command name(s) for this capability
    pub fn command_names(&self) -> Vec<&'static str> {
        match self {
            Self::Edit => vec!["EDIT"],
            Self::Append => vec!["APPEND"],
            Self::Move => vec!["MOVE"],
            Self::Delete => vec!["DELETE"],
            Self::Link => vec!["LINK", "UNLINK"],
            Self::Snapshot => vec!["SNAPSHOT"],
            Self::Transaction => vec!["BEGIN", "COMMIT", "ROLLBACK", "ATOMIC"],
        }
    }

    /// Get documentation for this capability
    pub fn documentation(&self) -> &'static str {
        match self {
            Self::Edit => EDIT_DOC,
            Self::Append => APPEND_DOC,
            Self::Move => MOVE_DOC,
            Self::Delete => DELETE_DOC,
            Self::Link => LINK_DOC,
            Self::Snapshot => SNAPSHOT_DOC,
            Self::Transaction => TRANSACTION_DOC,
        }
    }
}

const EDIT_DOC: &str = r#"### EDIT - Modify block content
```
EDIT <block_id> SET <path> = <value>
EDIT <block_id> SET <path> += <value>
```
Note: <path> is a property name like `text` or `content`. <value> must be a quoted string."#;

const APPEND_DOC: &str = r#"### APPEND - Add new blocks
```
APPEND <parent_id> <content_type> :: <content>
APPEND <parent_id> <content_type> WITH label = "name" :: <content>
APPEND <parent_id> <content_type> AT <index> :: <content>
```
Content types: text, code, table, math, json, media, binary, composite
IMPORTANT: WITH and AT modifiers must come BEFORE the :: separator, not after."#;

const MOVE_DOC: &str = r#"### MOVE - Relocate blocks
```
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```
IMPORTANT: Do NOT combine TO with BEFORE/AFTER. Use either "TO <parent>" OR "BEFORE <sibling>" OR "AFTER <sibling>"."#;

const DELETE_DOC: &str = r#"### DELETE - Remove blocks
```
DELETE <block_id>
DELETE <block_id> CASCADE
DELETE <block_id> PRESERVE_CHILDREN
```"#;

const LINK_DOC: &str = r#"### LINK/UNLINK - Manage relationships
```
LINK <source_id> <edge_type> <target_id>
UNLINK <source_id> <edge_type> <target_id>
```
Edge types: references, elaborates, summarizes, contradicts, supports, requires, parent_of"#;

const SNAPSHOT_DOC: &str = r#"### SNAPSHOT - Version control
```
SNAPSHOT CREATE "name"
SNAPSHOT CREATE "name" WITH description = "desc"
SNAPSHOT RESTORE "name"
SNAPSHOT LIST
SNAPSHOT DELETE "name"
```
IMPORTANT: Description requires `WITH description = "..."` syntax, NOT just two strings."#;

const TRANSACTION_DOC: &str = r#"### TRANSACTION - Atomic operations
```
BEGIN TRANSACTION
BEGIN TRANSACTION "name"
COMMIT
ROLLBACK
ATOMIC { <commands> }
```"#;

/// Builder for constructing LLM prompts with specific capabilities
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    capabilities: HashSet<UclCapability>,
    system_context: Option<String>,
    task_context: Option<String>,
    rules: Vec<String>,
    use_short_ids: bool,
}

impl PromptBuilder {
    /// Create a new prompt builder with no capabilities
    pub fn new() -> Self {
        Self {
            capabilities: HashSet::new(),
            system_context: None,
            task_context: None,
            rules: Vec::new(),
            use_short_ids: false,
        }
    }

    /// Create a builder with all capabilities enabled
    pub fn with_all_capabilities() -> Self {
        let mut builder = Self::new();
        for cap in UclCapability::all() {
            builder.capabilities.insert(cap);
        }
        builder
    }

    /// Add a single capability
    pub fn with_capability(mut self, cap: UclCapability) -> Self {
        self.capabilities.insert(cap);
        self
    }

    /// Add multiple capabilities
    pub fn with_capabilities(mut self, caps: impl IntoIterator<Item = UclCapability>) -> Self {
        self.capabilities.extend(caps);
        self
    }

    /// Remove a capability
    pub fn without_capability(mut self, cap: UclCapability) -> Self {
        self.capabilities.remove(&cap);
        self
    }

    /// Set custom system context (prepended to prompt)
    pub fn with_system_context(mut self, context: impl Into<String>) -> Self {
        self.system_context = Some(context.into());
        self
    }

    /// Set task-specific context
    pub fn with_task_context(mut self, context: impl Into<String>) -> Self {
        self.task_context = Some(context.into());
        self
    }

    /// Add a custom rule
    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }

    /// Enable short ID mode (for token efficiency)
    pub fn with_short_ids(mut self, enabled: bool) -> Self {
        self.use_short_ids = enabled;
        self
    }

    /// Build the system prompt
    pub fn build_system_prompt(&self) -> String {
        let mut parts = Vec::new();

        // System context
        if let Some(ref ctx) = self.system_context {
            parts.push(ctx.clone());
        } else {
            parts.push(self.default_system_context());
        }

        // Command reference header
        parts.push("\n## UCL Command Reference\n".to_string());

        // Add documentation for each enabled capability
        for cap in &self.capabilities {
            parts.push(cap.documentation().to_string());
            parts.push(String::new());
        }

        // Rules section
        parts.push("## Rules".to_string());

        // Default rules
        let default_rules = self.default_rules();
        for (i, rule) in default_rules.iter().enumerate() {
            parts.push(format!("{}. {}", i + 1, rule));
        }

        // Custom rules
        let offset = default_rules.len();
        for (i, rule) in self.rules.iter().enumerate() {
            parts.push(format!("{}. {}", offset + i + 1, rule));
        }

        parts.join("\n")
    }

    /// Build a complete prompt with document context
    pub fn build_prompt(&self, document_description: &str, task: &str) -> String {
        let mut parts = Vec::new();

        // Task context if provided
        if let Some(ref ctx) = self.task_context {
            parts.push(ctx.clone());
        }

        // Document structure
        parts.push("## Document Structure".to_string());
        parts.push(document_description.to_string());

        // Task
        parts.push("\n## Task".to_string());
        parts.push(task.to_string());

        // Instruction
        parts.push("\nGenerate the UCL command:".to_string());

        parts.join("\n")
    }

    fn default_system_context(&self) -> String {
        let caps: Vec<_> = self
            .capabilities
            .iter()
            .flat_map(|c| c.command_names())
            .collect();

        format!(
            "You are a UCL (Unified Content Language) command generator. \
            Your task is to generate valid UCL commands to manipulate documents.\n\n\
            Available commands: {}",
            caps.join(", ")
        )
    }

    fn default_rules(&self) -> Vec<&'static str> {
        let mut rules = vec![
            "Output ONLY the UCL command(s), no explanations or markdown",
            "Use exact block IDs as provided",
            "String values must be quoted with double quotes",
        ];

        if self.use_short_ids {
            rules.push("Block IDs are short numeric IDs (1, 2, 3, etc.)");
        } else {
            rules.push("Block IDs have format: blk_XXXXXXXXXXXX (12 hex chars)");
        }

        rules.push(
            "Do NOT use operators like + for string concatenation - just provide the full value",
        );

        rules
    }

    /// Get the list of enabled capabilities
    pub fn capabilities(&self) -> impl Iterator<Item = &UclCapability> {
        self.capabilities.iter()
    }

    /// Check if a capability is enabled
    pub fn has_capability(&self, cap: UclCapability) -> bool {
        self.capabilities.contains(&cap)
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::with_all_capabilities()
    }
}

/// Preset prompt configurations for common use cases
pub mod presets {
    use super::*;

    /// Basic editing only (EDIT, APPEND, DELETE)
    pub fn basic_editing() -> PromptBuilder {
        PromptBuilder::new()
            .with_capability(UclCapability::Edit)
            .with_capability(UclCapability::Append)
            .with_capability(UclCapability::Delete)
    }

    /// Structure manipulation (MOVE, LINK)
    pub fn structure_manipulation() -> PromptBuilder {
        PromptBuilder::new()
            .with_capability(UclCapability::Move)
            .with_capability(UclCapability::Link)
    }

    /// Full document editing (all except transactions)
    pub fn full_editing() -> PromptBuilder {
        PromptBuilder::new()
            .with_capability(UclCapability::Edit)
            .with_capability(UclCapability::Append)
            .with_capability(UclCapability::Move)
            .with_capability(UclCapability::Delete)
            .with_capability(UclCapability::Link)
    }

    /// Version control focused
    pub fn version_control() -> PromptBuilder {
        PromptBuilder::new()
            .with_capability(UclCapability::Snapshot)
            .with_capability(UclCapability::Transaction)
    }

    /// Token-efficient mode with short IDs
    pub fn token_efficient() -> PromptBuilder {
        PromptBuilder::with_all_capabilities().with_short_ids(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder_basic() {
        let builder = PromptBuilder::new().with_capability(UclCapability::Edit);

        let prompt = builder.build_system_prompt();
        assert!(prompt.contains("EDIT"));
        assert!(!prompt.contains("APPEND"));
    }

    #[test]
    fn test_prompt_builder_all_capabilities() {
        let builder = PromptBuilder::with_all_capabilities();
        let prompt = builder.build_system_prompt();

        assert!(prompt.contains("EDIT"));
        assert!(prompt.contains("APPEND"));
        assert!(prompt.contains("MOVE"));
        assert!(prompt.contains("DELETE"));
        assert!(prompt.contains("LINK"));
        assert!(prompt.contains("SNAPSHOT"));
        assert!(prompt.contains("ATOMIC"));
    }

    #[test]
    fn test_prompt_builder_custom_rules() {
        let builder = PromptBuilder::new()
            .with_capability(UclCapability::Edit)
            .with_rule("Always use lowercase for labels");

        let prompt = builder.build_system_prompt();
        assert!(prompt.contains("Always use lowercase for labels"));
    }

    #[test]
    fn test_prompt_builder_short_ids() {
        let builder = PromptBuilder::new()
            .with_capability(UclCapability::Edit)
            .with_short_ids(true);

        let prompt = builder.build_system_prompt();
        assert!(prompt.contains("short numeric IDs"));
    }

    #[test]
    fn test_presets() {
        let basic = presets::basic_editing();
        assert!(basic.has_capability(UclCapability::Edit));
        assert!(basic.has_capability(UclCapability::Append));
        assert!(basic.has_capability(UclCapability::Delete));
        assert!(!basic.has_capability(UclCapability::Move));

        let full = presets::full_editing();
        assert!(full.has_capability(UclCapability::Move));
        assert!(!full.has_capability(UclCapability::Transaction));
    }

    #[test]
    fn test_build_complete_prompt() {
        let builder = presets::basic_editing();

        let doc_desc = "Document with blocks: [1] Title, [2] Paragraph";
        let task = "Edit block 2 to say 'Hello World'";

        let prompt = builder.build_prompt(doc_desc, task);

        assert!(prompt.contains("Document Structure"));
        assert!(prompt.contains(doc_desc));
        assert!(prompt.contains("Task"));
        assert!(prompt.contains(task));
    }
}
