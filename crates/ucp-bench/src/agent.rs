//! LLM Agent for executing UCL commands.

use crate::documents::{DocumentDefinition, DocumentRegistry, DocumentDetailPayload, DOCUMENTS};
use crate::metrics::{ErrorCategory, TestResult};
use crate::provider::{CompletionRequest, LlmProvider, Message};
use crate::test_cases::TestCase;
use crate::test_document;
use std::sync::Arc;
use ucl_parser::Parser;
use ucm_core::Document;
use ucm_engine::Engine;

/// System prompt for the UCL agent
const SYSTEM_PROMPT: &str = r#"You are a UCL (Unified Content Language) command generator. Your task is to generate valid UCL commands to manipulate documents.

## UCL Command Reference

### EDIT - Modify block content
```
EDIT <block_id> SET <path> = <value>
EDIT <block_id> SET <path> += <value>
```
Note: <path> is a property name like `text` or `content`. <value> must be a quoted string.

### APPEND - Add new blocks
```
APPEND <parent_id> <content_type> :: <content>
APPEND <parent_id> <content_type> WITH label = "name" :: <content>
APPEND <parent_id> <content_type> AT <index> :: <content>
```
Content types: text, code, table, math, json, media, binary, composite
IMPORTANT: WITH and AT modifiers must come BEFORE the :: separator, not after.

### MOVE - Relocate blocks
```
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```
IMPORTANT: Do NOT combine TO with BEFORE/AFTER. Use either "TO <parent>" OR "BEFORE <sibling>" OR "AFTER <sibling>".

### DELETE - Remove blocks
```
DELETE <block_id>
DELETE <block_id> CASCADE
DELETE <block_id> PRESERVE_CHILDREN
```

### LINK/UNLINK - Manage relationships
```
LINK <source_id> <edge_type> <target_id>
UNLINK <source_id> <edge_type> <target_id>
```
Edge types: references, elaborates, summarizes, contradicts, supports, requires, parent_of

### SNAPSHOT - Version control
```
SNAPSHOT CREATE "name"
SNAPSHOT CREATE "name" WITH description = "desc"
SNAPSHOT RESTORE "name"
SNAPSHOT LIST
SNAPSHOT DELETE "name"
```
IMPORTANT: Description requires `WITH description = "..."` syntax, NOT just two strings.

### TRANSACTION - Atomic operations
```
BEGIN TRANSACTION
BEGIN TRANSACTION "name"
COMMIT
ROLLBACK
ATOMIC { <commands> }
```

## Rules
1. Output ONLY the UCL command(s), no explanations or markdown
2. Use exact block IDs as provided
3. String values must be quoted with double quotes
4. Block IDs have format: blk_XXXXXXXXXXXX (12 hex chars)
5. Do NOT use operators like + for string concatenation - just provide the full value

"#;

/// Agent that generates and executes UCL commands
pub struct BenchmarkAgent {
    provider: Arc<dyn LlmProvider>,
    document: Document,
    engine: Engine,
    /// If true, actually execute commands and validate document changes
    execute_commands: bool,
    document_definition: DocumentDefinition,
}

impl BenchmarkAgent {
    pub fn new(
        provider: Arc<dyn LlmProvider>,
        document_definition: DocumentDefinition,
        execute_commands: bool,
    ) -> Self {
        let document = document_definition.build();
        let engine = Engine::new();
        Self {
            provider,
            document,
            engine,
            execute_commands,
            document_definition,
        }
    }

    pub fn with_document(
        provider: Arc<dyn LlmProvider>,
        document: Document,
        document_definition: DocumentDefinition,
        execute_commands: bool,
    ) -> Self {
        let engine = Engine::new();
        Self {
            provider,
            document,
            engine,
            execute_commands,
            document_definition,
        }
    }

    /// Run a single test case and return the result
    pub async fn run_test(&mut self, test_case: &TestCase) -> TestResult {
        let model = self.provider.model_id().to_string();
        let provider_id = self.provider.provider_id().to_string();

        // Build the prompt
        let user_prompt = format!(
            "{}\n\n## Document Structure\n{}\n\n## Task\n{}\n\nGenerate the UCL command:",
            test_case.description,
            test_document::document_description(),
            test_case.prompt
        );

        // Store full prompt for debugging
        let full_prompt = format!("=== SYSTEM ===\n{}\n\n=== USER ===\n{}", SYSTEM_PROMPT, user_prompt);

        let request = CompletionRequest::new(vec![
            Message::system(SYSTEM_PROMPT),
            Message::user(&user_prompt),
        ])
        .with_max_tokens(1024)
        .with_temperature(0.0);

        // Call LLM
        let response = match self.provider.complete(request).await {
            Ok(r) => r,
            Err(e) => {
                let category = match &e {
                    crate::provider::ProviderError::RateLimited { .. } => ErrorCategory::RateLimitError,
                    crate::provider::ProviderError::Timeout { .. } => ErrorCategory::TimeoutError,
                    _ => ErrorCategory::ProviderError,
                };
                return TestResult::failure(
                    &test_case.id,
                    &test_case.command_type,
                    &model,
                    &provider_id,
                    category,
                    e.to_string(),
                );
            }
        };

        // Calculate cost
        let pricing = self.provider.pricing();
        let cost_usd = pricing.calculate_cost(response.input_tokens, response.output_tokens);

        // Store raw response for debugging
        let raw_response = response.content.clone();

        // Extract UCL from response
        let generated_ucl = extract_ucl(&response.content);

        // Create base result
        let mut result = TestResult::success(
            &test_case.id,
            &test_case.command_type,
            &model,
            &provider_id,
        );
        result.latency_ms = response.latency_ms;
        result.input_tokens = response.input_tokens;
        result.output_tokens = response.output_tokens;
        result.cost_usd = cost_usd;
        result.generated_ucl = generated_ucl.clone();
        result.expected_pattern = test_case.expected_pattern.clone();
        result.debug_prompt = Some(full_prompt.clone());
        result.debug_raw_response = Some(raw_response);

        // Validate: Parse
        let parse_result = Parser::new(&generated_ucl).parse_commands_only();
        match parse_result {
            Ok(commands) => {
                result.parse_success = true;

                // Validate: Execute
                if test_case.validation.must_execute && !commands.is_empty() {
                    if self.execute_commands {
                        // Skip execution for TRANSACTION/ATOMIC commands - they're validated at parse level
                        let skip_execution = matches!(
                            test_case.command_type.as_str(),
                            "TRANSACTION" | "ATOMIC"
                        ) || commands.iter().any(|c| matches!(
                            c,
                            ucl_parser::Command::Transaction(_) | 
                            ucl_parser::Command::Atomic(_) |
                            ucl_parser::Command::Snapshot(ucl_parser::SnapshotCommand::List) |
                            ucl_parser::Command::Snapshot(ucl_parser::SnapshotCommand::Diff { .. })
                        ));
                        
                        if skip_execution {
                            // These commands don't modify document state - just validate parsing
                            result.execute_success = true;
                        } else {
                            // Actually execute commands and validate document changes
                            match self.execute_and_validate(&commands, test_case) {
                                Ok(()) => {
                                    result.execute_success = true;
                                }
                                Err(e) => {
                                    result.execute_success = false;
                                    result.error_category = Some(ErrorCategory::ExecutionError);
                                    result.error_message = Some(format!("Execution error: {}", e));
                                }
                            }
                        }
                    } else {
                        // Just check that parsing succeeded (mock execution)
                        result.execute_success = true;
                    }
                }

                // Validate: Pattern match
                if let Some(ref pattern) = test_case.expected_pattern {
                    let re = regex::Regex::new(pattern).ok();
                    if let Some(re) = re {
                        if re.is_match(&generated_ucl) {
                            result.semantic_score = 1.0;
                        } else {
                            result.semantic_score = 0.5;
                        }
                    }
                }

                // Check forbidden patterns
                for forbidden in &test_case.validation.forbidden_patterns {
                    if generated_ucl.contains(forbidden) {
                        result.semantic_score *= 0.5;
                    }
                }
            }
            Err(e) => {
                result.parse_success = false;
                result.execute_success = false;
                result.semantic_score = 0.0;
                result.error_category = Some(ErrorCategory::ParseError);
                result.error_message = Some(format!("Parse error: {:?}", e));
            }
        }

        // Efficiency score based on command conciseness
        result.efficiency_score = calculate_efficiency(&generated_ucl);

        result
    }

    /// Execute commands and validate document changes
    fn execute_and_validate(
        &mut self,
        commands: &[ucl_parser::Command],
        test_case: &TestCase,
    ) -> Result<(), String> {
        // Store initial document state for comparison
        let initial_block_count = self.document.blocks.len();
        
        // Execute each command
        for command in commands {
            // Convert UCL command to engine operation
            let operation = match self.command_to_operation(command) {
                Ok(op) => op,
                Err(e) => return Err(format!("Failed to convert command: {}", e)),
            };
            
            // Execute the operation
            if let Err(e) = self.engine.execute(&mut self.document, operation) {
                return Err(format!("Engine execution failed: {:?}", e));
            }
        }
        
        // Validate based on test case criteria
        if let Some(ref target_id) = test_case.validation.target_block_id {
            // Check that the target block exists (for non-DELETE operations)
            if test_case.command_type != "DELETE" {
                use std::str::FromStr;
                if let Ok(block_id) = ucm_core::BlockId::from_str(target_id) {
                    if self.document.get_block(&block_id).is_none() {
                        return Err(format!("Target block {} not found after execution", target_id));
                    }
                }
            }
        }
        
        // For APPEND, verify block count increased
        if test_case.command_type == "APPEND" {
            let new_block_count = self.document.blocks.len();
            if new_block_count <= initial_block_count {
                return Err("APPEND did not increase block count".into());
            }
        }
        
        // For DELETE, verify block count decreased or stayed same (CASCADE might delete multiple)
        if test_case.command_type == "DELETE" {
            let new_block_count = self.document.blocks.len();
            if new_block_count > initial_block_count {
                return Err("DELETE should not increase block count".into());
            }
        }
        
        Ok(())
    }
    
    /// Convert a UCL command to an engine operation
    fn command_to_operation(&self, command: &ucl_parser::Command) -> Result<ucm_engine::Operation, String> {
        use ucl_parser::Command;
        use ucm_engine::{Operation, EditOperator};
        use ucm_core::{BlockId, Content, EdgeType};
        use std::str::FromStr;
        
        // Helper to parse block ID from string
        let parse_block_id = |s: &str| -> Result<BlockId, String> {
            BlockId::from_str(s).map_err(|e| format!("Invalid block ID '{}': {}", s, e))
        };
        
        match command {
            Command::Edit(edit) => {
                let block_id = parse_block_id(&edit.block_id)?;
                let operator = match edit.operator {
                    ucl_parser::Operator::Set => EditOperator::Set,
                    ucl_parser::Operator::Append => EditOperator::Append,
                    ucl_parser::Operator::Remove => EditOperator::Remove,
                    _ => EditOperator::Set, // Default for other operators
                };
                let value = match &edit.value {
                    ucl_parser::Value::String(s) => serde_json::Value::String(s.clone()),
                    ucl_parser::Value::Number(n) => serde_json::json!(n),
                    ucl_parser::Value::Bool(b) => serde_json::Value::Bool(*b),
                    ucl_parser::Value::Null => serde_json::Value::Null,
                    ucl_parser::Value::Array(arr) => serde_json::json!(arr),
                    ucl_parser::Value::Object(obj) => serde_json::json!(obj),
                    ucl_parser::Value::BlockRef(s) => serde_json::Value::String(s.clone()),
                };
                Ok(Operation::Edit {
                    block_id,
                    path: format!("{:?}", edit.path),
                    value,
                    operator,
                })
            }
            Command::Append(append) => {
                let parent_id = parse_block_id(&append.parent_id)?;
                let text = ucm_core::Text {
                    text: append.content.clone(),
                    format: ucm_core::TextFormat::Plain,
                };
                let content = Content::Text(text);
                let label = append.properties.get("label").and_then(|v| {
                    if let ucl_parser::Value::String(s) = v { Some(s.clone()) } else { None }
                });
                Ok(Operation::Append {
                    parent_id,
                    content,
                    label,
                    tags: Vec::new(),
                    semantic_role: None,
                    index: append.index,
                })
            }
            Command::Move(mv) => {
                let block_id = parse_block_id(&mv.block_id)?;
                let (new_parent, index) = match &mv.target {
                    ucl_parser::MoveTarget::ToParent { parent_id, index } => {
                        (parse_block_id(parent_id)?, *index)
                    }
                    ucl_parser::MoveTarget::Before { sibling_id } => {
                        // For BEFORE/AFTER, we'd need to look up the sibling's parent
                        // For now, use the sibling_id as target (engine should handle this)
                        (parse_block_id(sibling_id)?, None)
                    }
                    ucl_parser::MoveTarget::After { sibling_id } => {
                        (parse_block_id(sibling_id)?, None)
                    }
                };
                Ok(Operation::Move {
                    block_id,
                    new_parent,
                    index,
                })
            }
            Command::Delete(del) => {
                let block_id = del.block_id.as_ref()
                    .ok_or_else(|| "DELETE command missing block_id".to_string())?;
                let block_id = parse_block_id(block_id)?;
                Ok(Operation::Delete {
                    block_id,
                    cascade: del.cascade,
                    preserve_children: del.preserve_children,
                })
            }
            Command::Link(link) => {
                let source = parse_block_id(&link.source_id)?;
                let target = parse_block_id(&link.target_id)?;
                let edge_type = EdgeType::from_str(&link.edge_type)
                    .unwrap_or(EdgeType::References);
                Ok(Operation::Link {
                    source,
                    edge_type,
                    target,
                    metadata: None,
                })
            }
            Command::Unlink(unlink) => {
                let source = parse_block_id(&unlink.source_id)?;
                let target = parse_block_id(&unlink.target_id)?;
                let edge_type = EdgeType::from_str(&unlink.edge_type)
                    .unwrap_or(EdgeType::References);
                Ok(Operation::Unlink {
                    source,
                    edge_type,
                    target,
                })
            }
            Command::Snapshot(snap) => {
                match snap {
                    ucl_parser::SnapshotCommand::Create { name, description } => {
                        Ok(Operation::CreateSnapshot {
                            name: name.clone(),
                            description: description.clone(),
                        })
                    }
                    ucl_parser::SnapshotCommand::Restore { name } => {
                        Ok(Operation::RestoreSnapshot { name: name.clone() })
                    }
                    _ => Err("Unsupported snapshot operation".into()),
                }
            }
            Command::Transaction(_) | Command::Atomic(_) => {
                // Transaction commands don't map directly to single operations
                // For benchmarking, we just validate they parse correctly
                Err("Transaction/Atomic commands validated at parse level only".into())
            }
            _ => Err(format!("Unsupported command type: {:?}", command)),
        }
    }

    /// Reset the document to initial state
    pub fn reset(&mut self) {
        self.document = self.document_definition.build();
        self.engine = Engine::new();
    }

    /// Get current document state
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// Execute commands for playground (without test case validation)
    pub fn execute_and_validate_for_playground(&mut self, commands: &[ucl_parser::Command]) -> Result<(), String> {
        for command in commands {
            let operation = match self.command_to_operation(command) {
                Ok(op) => op,
                Err(e) => return Err(format!("Failed to convert command: {}", e)),
            };

            if let Err(e) = self.engine.execute(&mut self.document, operation) {
                return Err(format!("Engine execution failed: {:?}", e));
            }
        }
        Ok(())
    }
}

/// Extract UCL commands from LLM response (handles markdown code blocks)
pub fn extract_ucl(response: &str) -> String {
    let response = response.trim();

    // Check for code block
    if response.contains("```") {
        let lines: Vec<&str> = response.lines().collect();
        let mut in_block = false;
        let mut ucl_lines = Vec::new();

        for line in lines {
            if line.starts_with("```") {
                if in_block {
                    break;
                }
                in_block = true;
                continue;
            }
            if in_block {
                ucl_lines.push(line);
            }
        }

        if !ucl_lines.is_empty() {
            return ucl_lines.join("\n");
        }
    }

    // Return as-is if no code block found
    response.to_string()
}

/// Calculate efficiency score based on command structure
fn calculate_efficiency(ucl: &str) -> f32 {
    let len = ucl.len();
    if len == 0 {
        return 0.0;
    }

    // Penalize overly verbose commands
    let base_score: f32 = if len < 100 {
        1.0
    } else if len < 200 {
        0.9
    } else if len < 500 {
        0.7
    } else {
        0.5
    };

    // Bonus for using proper UCL structure
    let structure_bonus: f32 = if ucl.contains("ATOMIC") || ucl.contains("BEGIN") {
        0.0 // Complex operations get no penalty
    } else {
        0.0
    };

    (base_score + structure_bonus).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::MockProvider;

    #[test]
    fn test_extract_ucl_plain() {
        let response = r#"EDIT blk_abc SET text = "hello""#;
        assert_eq!(extract_ucl(response), response);
    }

    #[test]
    fn test_extract_ucl_code_block() {
        let response = r#"Here's the command:

```ucl
EDIT blk_abc SET text = "hello"
```

This will update the text."#;
        assert_eq!(extract_ucl(response), r#"EDIT blk_abc SET text = "hello""#);
    }

    #[tokio::test]
    async fn test_agent_run() {
        let provider = Arc::new(
            MockProvider::new("mock")
                .with_responses(vec![r#"EDIT blk_000000000011 SET text = "updated""#.into()])
        );
        let mut agent = BenchmarkAgent::new(provider, false);

        let test_case = TestCase {
            id: "test_001".into(),
            command_type: "EDIT".into(),
            description: "Test edit".into(),
            prompt: "Edit the intro hook".into(),
            expected_pattern: Some(r"EDIT\s+blk_000000000011".into()),
            validation: crate::test_cases::ValidationCriteria::default(),
        };

        let result = agent.run_test(&test_case).await;
        assert!(result.parse_success);
    }
}
