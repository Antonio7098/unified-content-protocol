//! State management for CLI sessions
//!
//! This module provides state persistence for agent sessions, transactions,
//! and snapshots across CLI invocations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ucm_core::{BlockId, Document};

use crate::output::DocumentJson;

/// Complete CLI state that can be serialized with the document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliState {
    /// Active agent sessions
    #[serde(default)]
    pub sessions: HashMap<String, AgentSessionState>,

    /// Snapshots
    #[serde(default)]
    pub snapshots: Vec<SnapshotInfo>,

    /// Transaction state (if in a transaction)
    #[serde(default)]
    pub transaction: Option<TransactionState>,
}

impl CliState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Serializable agent session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSessionState {
    pub id: String,
    pub name: Option<String>,
    pub current_block: Option<String>,
    pub history: Vec<String>,
    pub context_blocks: Vec<String>,
    pub state: String,
    pub created_at: String,
}

impl AgentSessionState {
    pub fn new(id: String, name: Option<String>, start_block: Option<BlockId>) -> Self {
        Self {
            id,
            name,
            current_block: start_block.map(|b| b.to_string()),
            history: Vec::new(),
            context_blocks: Vec::new(),
            state: "active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn goto(&mut self, block_id: &BlockId) {
        if let Some(current) = &self.current_block {
            self.history.push(current.clone());
        }
        self.current_block = Some(block_id.to_string());
    }

    pub fn back(&mut self, steps: usize) -> Option<BlockId> {
        use std::str::FromStr;
        for _ in 0..steps {
            if let Some(prev) = self.history.pop() {
                self.current_block = Some(prev);
            }
        }
        self.current_block
            .as_ref()
            .and_then(|s| BlockId::from_str(s).ok())
    }

    pub fn add_to_context(&mut self, block_id: &BlockId) {
        let id_str = block_id.to_string();
        if !self.context_blocks.contains(&id_str) {
            self.context_blocks.push(id_str);
        }
    }

    pub fn remove_from_context(&mut self, block_id: &BlockId) {
        let id_str = block_id.to_string();
        self.context_blocks.retain(|b| b != &id_str);
    }

    pub fn clear_context(&mut self) {
        self.context_blocks.clear();
    }
}

/// Serializable snapshot info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub block_count: usize,
    pub document_json: String,
}

impl SnapshotInfo {
    pub fn create(
        name: String,
        description: Option<String>,
        doc: &Document,
    ) -> anyhow::Result<Self> {
        let doc_json = DocumentJson::from_document(doc);
        Ok(Self {
            name,
            description,
            created_at: chrono::Utc::now().to_rfc3339(),
            block_count: doc.block_count(),
            document_json: serde_json::to_string(&doc_json)?,
        })
    }

    pub fn restore(&self) -> anyhow::Result<Document> {
        let doc_json: DocumentJson = serde_json::from_str(&self.document_json)?;
        doc_json.to_document()
    }
}

/// Transaction state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionState {
    pub name: Option<String>,
    pub started_at: String,
    pub savepoints: Vec<SavepointInfo>,
    /// Document state at transaction start
    pub original_document: String,
}

impl TransactionState {
    pub fn new(name: Option<String>, doc: &Document) -> anyhow::Result<Self> {
        let doc_json = DocumentJson::from_document(doc);
        Ok(Self {
            name,
            started_at: chrono::Utc::now().to_rfc3339(),
            savepoints: Vec::new(),
            original_document: serde_json::to_string(&doc_json)?,
        })
    }

    pub fn create_savepoint(&mut self, name: String, doc: &Document) -> anyhow::Result<()> {
        let doc_json = DocumentJson::from_document(doc);
        self.savepoints.push(SavepointInfo {
            name,
            created_at: chrono::Utc::now().to_rfc3339(),
            document_json: serde_json::to_string(&doc_json)?,
        });
        Ok(())
    }

    pub fn rollback_to_savepoint(&self, name: &str) -> anyhow::Result<Option<Document>> {
        for savepoint in self.savepoints.iter().rev() {
            if savepoint.name == name {
                let doc_json: DocumentJson = serde_json::from_str(&savepoint.document_json)?;
                return Ok(Some(doc_json.to_document()?));
            }
        }
        Ok(None)
    }

    pub fn get_original_document(&self) -> anyhow::Result<Document> {
        let doc_json: DocumentJson = serde_json::from_str(&self.original_document)?;
        doc_json.to_document()
    }
}

/// Savepoint info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavepointInfo {
    pub name: String,
    pub created_at: String,
    pub document_json: String,
}

/// Document with CLI state - stored as separate JSON fields
#[derive(Debug, Clone)]
pub struct StatefulDocument {
    pub document: Document,
    pub cli_state: CliState,
}

/// JSON representation for stateful document
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatefulDocumentJson {
    #[serde(flatten)]
    document: DocumentJson,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cli_state: Option<CliState>,
}

impl StatefulDocument {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            cli_state: CliState::new(),
        }
    }

    pub fn from_document(document: Document) -> Self {
        Self {
            document,
            cli_state: CliState::new(),
        }
    }

    pub fn state(&self) -> &CliState {
        &self.cli_state
    }

    pub fn state_mut(&mut self) -> &mut CliState {
        &mut self.cli_state
    }
}

/// Read a stateful document from file or stdin
pub fn read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> {
    let json = if let Some(path) = input {
        std::fs::read_to_string(&path)?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Try to parse as StatefulDocumentJson first
    if let Ok(parsed) = serde_json::from_str::<StatefulDocumentJson>(&json) {
        let cli_state = parsed.cli_state.unwrap_or_default();
        let doc = parsed.document.to_document()?;
        return Ok(StatefulDocument {
            document: doc,
            cli_state,
        });
    }

    // Try as plain DocumentJson
    if let Ok(doc_json) = serde_json::from_str::<DocumentJson>(&json) {
        let doc = doc_json.to_document()?;
        return Ok(StatefulDocument::from_document(doc));
    }

    Err(anyhow::anyhow!("Failed to parse document JSON"))
}

/// Write a stateful document to file or stdout
pub fn write_stateful_document(
    doc: &StatefulDocument,
    output: Option<String>,
) -> anyhow::Result<()> {
    let doc_json = DocumentJson::from_document(&doc.document);

    // Create combined JSON
    let stateful_json = StatefulDocumentJson {
        document: doc_json,
        cli_state: if doc.cli_state.sessions.is_empty()
            && doc.cli_state.snapshots.is_empty()
            && doc.cli_state.transaction.is_none()
        {
            None
        } else {
            Some(doc.cli_state.clone())
        },
    };

    let json = serde_json::to_string_pretty(&stateful_json)?;

    if let Some(path) = output {
        std::fs::write(&path, &json)?;
    } else {
        println!("{}", json);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::Document;

    #[test]
    fn test_cli_state_new() {
        let state = CliState::new();
        assert!(state.sessions.is_empty());
        assert!(state.snapshots.is_empty());
        assert!(state.transaction.is_none());
    }

    #[test]
    fn test_agent_session_goto() {
        let mut session = AgentSessionState::new("test-session".to_string(), None, None);
        let block_id = ucm_core::BlockId::root();

        session.goto(&block_id);

        assert_eq!(session.current_block, Some(block_id.to_string()));
        assert_eq!(session.history.len(), 0); // First goto doesn't add to history
    }

    #[test]
    fn test_agent_session_back() {
        let mut session = AgentSessionState::new("test-session".to_string(), None, None);
        let block1 = ucm_core::BlockId::root();
        let block2 = ucm_core::BlockId::from_hex("aabbccddeeff001122334455").unwrap();

        session.goto(&block1);
        session.goto(&block2);

        let result = session.back(1);
        assert_eq!(result, Some(block1.clone()));
        assert_eq!(session.current_block, Some(block1.to_string()));
    }

    #[test]
    fn test_agent_session_back_empty() {
        let mut session = AgentSessionState::new("test-session".to_string(), None, None);
        let result = session.back(1);
        assert!(result.is_none());
    }

    #[test]
    fn test_agent_session_context() {
        let mut session = AgentSessionState::new("test-session".to_string(), None, None);
        let block1 = ucm_core::BlockId::root();
        let block2 = ucm_core::BlockId::from_hex("aabbccddeeff001122334455").unwrap();

        session.add_to_context(&block1);
        session.add_to_context(&block2);

        assert_eq!(session.context_blocks.len(), 2);

        session.remove_from_context(&block1);
        assert_eq!(session.context_blocks.len(), 1);
        assert!(!session.context_blocks.contains(&block1.to_string()));
        assert!(session.context_blocks.contains(&block2.to_string()));
    }

    #[test]
    fn test_snapshot_info_create_restore() {
        let doc = Document::create();
        let snapshot = SnapshotInfo::create(
            "test-snapshot".to_string(),
            Some("Test description".to_string()),
            &doc,
        )
        .expect("Should create snapshot");

        assert_eq!(snapshot.name, "test-snapshot");
        assert_eq!(snapshot.description, Some("Test description".to_string()));
        assert_eq!(snapshot.block_count, doc.block_count());

        let restored = snapshot.restore().expect("Should restore");
        assert_eq!(restored.block_count(), doc.block_count());
    }

    #[test]
    fn test_transaction_state_new() {
        let doc = Document::create();
        let tx = TransactionState::new(Some("test-tx".to_string()), &doc)
            .expect("Should create transaction");

        assert_eq!(tx.name, Some("test-tx".to_string()));
        assert!(tx.savepoints.is_empty());
    }

    #[test]
    fn test_transaction_state_savepoint() {
        let doc = Document::create();
        let mut tx = TransactionState::new(None, &doc).expect("Should create transaction");

        tx.create_savepoint("sp1".to_string(), &doc)
            .expect("Should create savepoint");

        assert_eq!(tx.savepoints.len(), 1);
        assert_eq!(tx.savepoints[0].name, "sp1");
    }

    #[test]
    fn test_stateful_document_from_document() {
        let doc = Document::create();
        let stateful = StatefulDocument::from_document(doc);

        assert!(stateful.cli_state.sessions.is_empty());
        assert!(stateful.cli_state.snapshots.is_empty());
        assert!(stateful.cli_state.transaction.is_none());
    }
}
