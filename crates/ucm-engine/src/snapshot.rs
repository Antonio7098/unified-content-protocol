//! Snapshot management for document versioning.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ucm_core::{Document, DocumentVersion, Error, Result};

/// Snapshot identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub String);

impl SnapshotId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl std::fmt::Display for SnapshotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A snapshot of document state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Snapshot identifier (name)
    pub id: SnapshotId,
    /// Optional description
    pub description: Option<String>,
    /// When the snapshot was created
    pub created_at: DateTime<Utc>,
    /// Document version at snapshot time
    pub document_version: DocumentVersion,
    /// Serialized document data
    pub data: SnapshotData,
}

/// Snapshot data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotData {
    /// Full document copy
    Full(SerializedDocument),
    /// Delta from a base snapshot (future optimization)
    Delta {
        base: SnapshotId,
        changes: Vec<SnapshotChange>,
    },
}

/// Serialized document for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedDocument {
    /// JSON representation of the document
    pub json: String,
}

impl SerializedDocument {
    pub fn from_document(doc: &Document) -> Result<Self> {
        // We need to serialize the document structure
        // For now, we'll create a serializable version
        let serializable = SerializableDocument::from(doc);
        let json = serde_json::to_string(&serializable)
            .map_err(|e| Error::Internal(format!("Failed to serialize document: {}", e)))?;
        Ok(Self { json })
    }

    pub fn to_document(&self) -> Result<Document> {
        let serializable: SerializableDocument = serde_json::from_str(&self.json)
            .map_err(|e| Error::Internal(format!("Failed to deserialize document: {}", e)))?;
        Ok(serializable.into())
    }
}

/// Serializable version of Document
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableDocument {
    id: String,
    root: String,
    structure: HashMap<String, Vec<String>>,
    blocks: HashMap<String, serde_json::Value>,
    metadata: serde_json::Value,
    version: DocumentVersion,
}

impl From<&Document> for SerializableDocument {
    fn from(doc: &Document) -> Self {
        let structure: HashMap<String, Vec<String>> = doc
            .structure
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|id| id.to_string()).collect()))
            .collect();

        let blocks: HashMap<String, serde_json::Value> = doc
            .blocks
            .iter()
            .map(|(k, v)| (k.to_string(), serde_json::to_value(v).unwrap_or_default()))
            .collect();

        Self {
            id: doc.id.0.clone(),
            root: doc.root.to_string(),
            structure,
            blocks,
            metadata: serde_json::to_value(&doc.metadata).unwrap_or_default(),
            version: doc.version.clone(),
        }
    }
}

impl From<SerializableDocument> for Document {
    fn from(s: SerializableDocument) -> Self {
        use ucm_core::{Block, BlockId, DocumentId, DocumentMetadata};

        let root: BlockId = s.root.parse().unwrap_or_else(|_| BlockId::root());

        let structure: HashMap<BlockId, Vec<BlockId>> = s
            .structure
            .into_iter()
            .filter_map(|(k, v)| {
                let key: BlockId = k.parse().ok()?;
                let values: Vec<BlockId> = v.into_iter().filter_map(|id| id.parse().ok()).collect();
                Some((key, values))
            })
            .collect();

        let blocks: HashMap<BlockId, Block> = s
            .blocks
            .into_iter()
            .filter_map(|(k, v)| {
                let key: BlockId = k.parse().ok()?;
                let block: Block = serde_json::from_value(v).ok()?;
                Some((key, block))
            })
            .collect();

        let metadata: DocumentMetadata = serde_json::from_value(s.metadata).unwrap_or_default();

        let mut doc = Document::new(DocumentId::new(s.id));
        doc.root = root;
        doc.structure = structure;
        doc.blocks = blocks;
        doc.metadata = metadata;
        doc.version = s.version;
        doc.rebuild_indices();
        doc
    }
}

/// Change record for delta snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotChange {
    AddBlock {
        id: String,
        block: serde_json::Value,
    },
    RemoveBlock {
        id: String,
    },
    ModifyBlock {
        id: String,
        block: serde_json::Value,
    },
    UpdateStructure {
        parent: String,
        children: Vec<String>,
    },
}

/// Manages document snapshots
#[derive(Debug, Default)]
pub struct SnapshotManager {
    snapshots: HashMap<SnapshotId, Snapshot>,
    max_snapshots: usize,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
            max_snapshots: 100,
        }
    }

    pub fn with_max_snapshots(max: usize) -> Self {
        Self {
            snapshots: HashMap::new(),
            max_snapshots: max,
        }
    }

    /// Create a snapshot of the document
    pub fn create(
        &mut self,
        name: impl Into<String>,
        doc: &Document,
        description: Option<String>,
    ) -> Result<SnapshotId> {
        let id = SnapshotId::new(name);

        // Check if we need to evict old snapshots
        if self.snapshots.len() >= self.max_snapshots {
            self.evict_oldest();
        }

        let data = SnapshotData::Full(SerializedDocument::from_document(doc)?);

        let snapshot = Snapshot {
            id: id.clone(),
            description,
            created_at: Utc::now(),
            document_version: doc.version.clone(),
            data,
        };

        self.snapshots.insert(id.clone(), snapshot);
        Ok(id)
    }

    /// Restore a document from a snapshot
    pub fn restore(&self, name: &str) -> Result<Document> {
        let id = SnapshotId::new(name);
        let snapshot = self
            .snapshots
            .get(&id)
            .ok_or_else(|| Error::Internal(format!("Snapshot '{}' not found", name)))?;

        match &snapshot.data {
            SnapshotData::Full(serialized) => serialized.to_document(),
            SnapshotData::Delta { .. } => {
                // TODO: implement delta restoration
                Err(Error::Internal("Delta snapshots not yet supported".into()))
            }
        }
    }

    /// Get a snapshot by name
    pub fn get(&self, name: &str) -> Option<&Snapshot> {
        self.snapshots.get(&SnapshotId::new(name))
    }

    /// List all snapshots
    pub fn list(&self) -> Vec<&Snapshot> {
        let mut snapshots: Vec<_> = self.snapshots.values().collect();
        snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        snapshots
    }

    /// Delete a snapshot
    pub fn delete(&mut self, name: &str) -> bool {
        self.snapshots.remove(&SnapshotId::new(name)).is_some()
    }

    /// Check if a snapshot exists
    pub fn exists(&self, name: &str) -> bool {
        self.snapshots.contains_key(&SnapshotId::new(name))
    }

    /// Get snapshot count
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    /// Evict the oldest snapshot
    fn evict_oldest(&mut self) {
        if let Some(oldest) = self
            .snapshots
            .values()
            .min_by_key(|s| s.created_at)
            .map(|s| s.id.clone())
        {
            self.snapshots.remove(&oldest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::{Block, Content, DocumentId};

    #[test]
    fn test_snapshot_create_restore() {
        let mut mgr = SnapshotManager::new();
        let mut doc = Document::new(DocumentId::new("test"));

        let root = doc.root;
        doc.add_block(Block::new(Content::text("Hello"), Some("intro")), &root)
            .unwrap();

        mgr.create("v1", &doc, Some("First version".into()))
            .unwrap();

        let restored = mgr.restore("v1").unwrap();
        assert_eq!(restored.block_count(), doc.block_count());
    }

    #[test]
    fn test_snapshot_list() {
        let mut mgr = SnapshotManager::new();
        let doc = Document::create();

        mgr.create("v1", &doc, None).unwrap();
        mgr.create("v2", &doc, None).unwrap();
        mgr.create("v3", &doc, None).unwrap();

        assert_eq!(mgr.count(), 3);
        assert_eq!(mgr.list().len(), 3);
    }

    #[test]
    fn test_snapshot_delete() {
        let mut mgr = SnapshotManager::new();
        let doc = Document::create();

        mgr.create("v1", &doc, None).unwrap();
        assert!(mgr.exists("v1"));

        mgr.delete("v1");
        assert!(!mgr.exists("v1"));
    }

    #[test]
    fn test_snapshot_eviction() {
        let mut mgr = SnapshotManager::with_max_snapshots(2);
        let doc = Document::create();

        mgr.create("v1", &doc, None).unwrap();
        mgr.create("v2", &doc, None).unwrap();
        mgr.create("v3", &doc, None).unwrap();

        assert_eq!(mgr.count(), 2);
        assert!(!mgr.exists("v1")); // v1 should be evicted
    }
}
