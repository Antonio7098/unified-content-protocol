use std::{cell::RefCell, path::Path};

use chrono::Utc;
use rusqlite::{params, Connection};
use ucm_core::{BlockId, PortableDocument};

use super::{
    GraphNodeRecord, GraphStore, GraphStoreError, GraphStoreObservability, GraphStoreStats,
};
use crate::types::GraphEdgeSummary;

#[derive(Debug)]
pub struct SqliteGraphStore {
    graph_key: String,
    connection: RefCell<Connection>,
    stats: GraphStoreStats,
}

impl SqliteGraphStore {
    pub fn import_document(
        path: impl AsRef<Path>,
        graph_key: impl Into<String>,
        portable: &PortableDocument,
    ) -> Result<Self, GraphStoreError> {
        let graph_key = graph_key.into();
        let connection = Connection::open(path)?;
        init_schema(&connection)?;
        let payload = serde_json::to_string(portable)?;
        let document = portable.to_document()?;
        let parent_by_child = document
            .structure
            .iter()
            .flat_map(|(parent, children)| children.iter().map(move |child| (*child, *parent)))
            .collect::<std::collections::HashMap<_, _>>();
        let explicit_edge_count = document
            .blocks
            .values()
            .map(|block| block.edges.len())
            .sum::<usize>();
        let structural_edge_count = parent_by_child.len();

        connection.execute("BEGIN IMMEDIATE TRANSACTION", [])?;
        connection.execute(
            "DELETE FROM edges WHERE graph_key = ?1",
            params![graph_key.as_str()],
        )?;
        connection.execute(
            "DELETE FROM structure WHERE graph_key = ?1",
            params![graph_key.as_str()],
        )?;
        connection.execute(
            "DELETE FROM nodes WHERE graph_key = ?1",
            params![graph_key.as_str()],
        )?;
        connection.execute(
            "DELETE FROM graphs WHERE graph_key = ?1",
            params![graph_key.as_str()],
        )?;

        connection.execute(
            "INSERT INTO graphs (graph_key, document_json, document_id, root_block_id, node_count, explicit_edge_count, structural_edge_count, captured_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                graph_key.as_str(),
                payload,
                document.id.0.as_str(),
                document.root.to_string(),
                document.blocks.len() as i64,
                explicit_edge_count as i64,
                structural_edge_count as i64,
                Utc::now().to_rfc3339(),
            ],
        )?;

        for (parent, children) in &document.structure {
            for (ordinal, child) in children.iter().enumerate() {
                connection.execute(
                    "INSERT INTO structure (graph_key, parent_block_id, child_block_id, ordinal) VALUES (?1, ?2, ?3, ?4)",
                    params![graph_key.as_str(), parent.to_string(), child.to_string(), ordinal as i64],
                )?;
            }
        }

        for block in document.blocks.values() {
            let parent = parent_by_child.get(&block.id).map(ToString::to_string);
            connection.execute(
                "INSERT INTO nodes (graph_key, block_id, label, content_type, semantic_role, tags_json, parent_block_id, child_count, outgoing_count, incoming_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    graph_key.as_str(),
                    block.id.to_string(),
                    block.metadata.label.clone(),
                    block.content_type(),
                    block.metadata.semantic_role.as_ref().map(ToString::to_string),
                    serde_json::to_string(&block.metadata.tags)?,
                    parent,
                    document.children(&block.id).len() as i64,
                    document.edge_index.outgoing_from(&block.id).len() as i64,
                    document.edge_index.incoming_to(&block.id).len() as i64,
                ],
            )?;

            for edge in &block.edges {
                connection.execute(
                    "INSERT INTO edges (graph_key, source_block_id, target_block_id, relation) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        graph_key.as_str(),
                        block.id.to_string(),
                        edge.target.to_string(),
                        edge_relation(&edge.edge_type),
                    ],
                )?;
            }
        }

        connection.execute("COMMIT", [])?;
        Self::open_with_connection(graph_key, connection)
    }

    pub fn open(
        path: impl AsRef<Path>,
        graph_key: impl Into<String>,
    ) -> Result<Self, GraphStoreError> {
        let connection = Connection::open(path)?;
        init_schema(&connection)?;
        Self::open_with_connection(graph_key.into(), connection)
    }

    fn open_with_connection(
        graph_key: String,
        connection: Connection,
    ) -> Result<Self, GraphStoreError> {
        let stats = load_stats(&connection, &graph_key)?;
        Ok(Self {
            graph_key,
            connection: RefCell::new(connection),
            stats,
        })
    }
}

impl GraphStore for SqliteGraphStore {
    fn stats(&self) -> GraphStoreStats {
        self.stats.clone()
    }

    fn observability(&self) -> GraphStoreObservability {
        GraphStoreObservability {
            stats: self.stats(),
            indexed_fields: vec![
                "block_id".to_string(),
                "label".to_string(),
                "content_type".to_string(),
                "semantic_role".to_string(),
                "parent_block_id".to_string(),
                "source_block_id".to_string(),
                "target_block_id".to_string(),
            ],
        }
    }

    fn root_id(&self) -> BlockId {
        self.stats.root_block_id
    }

    fn node_ids(&self) -> Vec<BlockId> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare("SELECT block_id FROM nodes WHERE graph_key = ?1 ORDER BY block_id")
            .expect("prepare node id query");
        stmt.query_map(params![self.graph_key.as_str()], |row| {
            row.get::<_, String>(0)
        })
        .expect("query node ids")
        .filter_map(|value| value.ok())
        .filter_map(|value| value.parse().ok())
        .collect()
    }

    fn node(&self, block_id: BlockId) -> Option<GraphNodeRecord> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare(
                "SELECT n.label,
                        n.content_type,
                        n.semantic_role,
                        n.tags_json,
                        n.parent_block_id,
                        (SELECT COUNT(*) FROM structure s WHERE s.graph_key = n.graph_key AND s.parent_block_id = n.block_id) AS child_count,
                        (SELECT COUNT(*) FROM edges e WHERE e.graph_key = n.graph_key AND e.source_block_id = n.block_id) AS outgoing_count,
                        (SELECT COUNT(*) FROM edges e WHERE e.graph_key = n.graph_key AND e.target_block_id = n.block_id) AS incoming_count
                 FROM nodes n WHERE n.graph_key = ?1 AND n.block_id = ?2",
            )
            .ok()?;
        stmt.query_row(
            params![self.graph_key.as_str(), block_id.to_string()],
            |row| {
                let tags_json: String = row.get(3)?;
                Ok(GraphNodeRecord {
                    block_id,
                    label: row.get(0)?,
                    content_type: row.get(1)?,
                    semantic_role: row.get(2)?,
                    tags: serde_json::from_str(&tags_json).unwrap_or_default(),
                    parent: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|value| value.parse().ok()),
                    children: row.get::<_, i64>(5)? as usize,
                    outgoing_edges: row.get::<_, i64>(6)? as usize,
                    incoming_edges: row.get::<_, i64>(7)? as usize,
                })
            },
        )
        .ok()
    }

    fn children(&self, block_id: BlockId) -> Vec<BlockId> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare(
                "SELECT child_block_id FROM structure WHERE graph_key = ?1 AND parent_block_id = ?2 ORDER BY ordinal",
            )
            .expect("prepare child query");
        stmt.query_map(
            params![self.graph_key.as_str(), block_id.to_string()],
            |row| row.get::<_, String>(0),
        )
        .expect("query children")
        .filter_map(|value| value.ok())
        .filter_map(|value| value.parse().ok())
        .collect()
    }

    fn parent(&self, block_id: BlockId) -> Option<BlockId> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare("SELECT parent_block_id FROM nodes WHERE graph_key = ?1 AND block_id = ?2")
            .ok()?;
        stmt.query_row(
            params![self.graph_key.as_str(), block_id.to_string()],
            |row| row.get::<_, Option<String>>(0),
        )
        .ok()
        .flatten()
        .and_then(|value| value.parse().ok())
    }

    fn outgoing_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare(
                "SELECT target_block_id, relation FROM edges WHERE graph_key = ?1 AND source_block_id = ?2 ORDER BY relation, target_block_id",
            )
            .expect("prepare outgoing edge query");
        stmt.query_map(
            params![self.graph_key.as_str(), block_id.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .expect("query outgoing edges")
        .filter_map(|value| value.ok())
        .filter_map(|(target, relation)| {
            Some(GraphEdgeSummary {
                source: block_id,
                target: target.parse().ok()?,
                relation,
                direction: "outgoing".to_string(),
            })
        })
        .collect()
    }

    fn incoming_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary> {
        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare(
                "SELECT source_block_id, relation FROM edges WHERE graph_key = ?1 AND target_block_id = ?2 ORDER BY relation, source_block_id",
            )
            .expect("prepare incoming edge query");
        stmt.query_map(
            params![self.graph_key.as_str(), block_id.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .expect("query incoming edges")
        .filter_map(|value| value.ok())
        .filter_map(|(source, relation)| {
            Some(GraphEdgeSummary {
                source: source.parse().ok()?,
                target: block_id,
                relation,
                direction: "incoming".to_string(),
            })
        })
        .collect()
    }

    fn resolve_selector(&self, selector: &str) -> Option<BlockId> {
        if selector == "root" {
            return Some(self.root_id());
        }
        if let Ok(block_id) = selector.parse::<BlockId>() {
            if self.node(block_id).is_some() {
                return Some(block_id);
            }
        }

        let conn = self.connection.borrow();
        let mut stmt = conn
            .prepare("SELECT block_id FROM nodes WHERE graph_key = ?1 AND label = ?2 LIMIT 1")
            .ok()?;
        stmt.query_row(params![self.graph_key.as_str(), selector], |row| {
            row.get::<_, String>(0)
        })
        .ok()
        .and_then(|value| value.parse().ok())
    }

    fn to_portable_document(&self) -> Result<PortableDocument, GraphStoreError> {
        let conn = self.connection.borrow();
        let payload = conn.query_row(
            "SELECT document_json FROM graphs WHERE graph_key = ?1",
            params![self.graph_key.as_str()],
            |row| row.get::<_, String>(0),
        )?;
        Ok(serde_json::from_str(&payload)?)
    }
}

fn init_schema(connection: &Connection) -> Result<(), GraphStoreError> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS graphs (
            graph_key TEXT PRIMARY KEY,
            document_json TEXT NOT NULL,
            document_id TEXT NOT NULL,
            root_block_id TEXT NOT NULL,
            node_count INTEGER NOT NULL,
            explicit_edge_count INTEGER NOT NULL,
            structural_edge_count INTEGER NOT NULL,
            captured_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS nodes (
            graph_key TEXT NOT NULL,
            block_id TEXT NOT NULL,
            label TEXT,
            content_type TEXT NOT NULL,
            semantic_role TEXT,
            tags_json TEXT NOT NULL,
            parent_block_id TEXT,
            child_count INTEGER NOT NULL,
            outgoing_count INTEGER NOT NULL,
            incoming_count INTEGER NOT NULL,
            PRIMARY KEY (graph_key, block_id)
        );
        CREATE TABLE IF NOT EXISTS structure (
            graph_key TEXT NOT NULL,
            parent_block_id TEXT NOT NULL,
            child_block_id TEXT NOT NULL,
            ordinal INTEGER NOT NULL,
            PRIMARY KEY (graph_key, parent_block_id, child_block_id)
        );
        CREATE TABLE IF NOT EXISTS edges (
            graph_key TEXT NOT NULL,
            source_block_id TEXT NOT NULL,
            target_block_id TEXT NOT NULL,
            relation TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_nodes_graph_label ON nodes(graph_key, label);
        CREATE INDEX IF NOT EXISTS idx_nodes_graph_content_type ON nodes(graph_key, content_type);
        CREATE INDEX IF NOT EXISTS idx_nodes_graph_parent ON nodes(graph_key, parent_block_id);
        CREATE INDEX IF NOT EXISTS idx_structure_graph_parent ON structure(graph_key, parent_block_id, ordinal);
        CREATE INDEX IF NOT EXISTS idx_edges_graph_source ON edges(graph_key, source_block_id, relation);
        CREATE INDEX IF NOT EXISTS idx_edges_graph_target ON edges(graph_key, target_block_id, relation);",
    )?;
    Ok(())
}

fn load_stats(
    connection: &Connection,
    graph_key: &str,
) -> Result<GraphStoreStats, GraphStoreError> {
    connection
        .query_row(
            "SELECT document_id, root_block_id, node_count, explicit_edge_count, structural_edge_count, captured_at FROM graphs WHERE graph_key = ?1",
            params![graph_key],
            |row| {
                Ok(GraphStoreStats {
                    backend: "sqlite".to_string(),
                    document_id: row.get(0)?,
                    root_block_id: row
                        .get::<_, String>(1)?
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                    node_count: row.get::<_, i64>(2)? as usize,
                    explicit_edge_count: row.get::<_, i64>(3)? as usize,
                    structural_edge_count: row.get::<_, i64>(4)? as usize,
                    captured_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .map(|value| value.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    graph_key: Some(graph_key.to_string()),
                })
            },
        )
        .map_err(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => GraphStoreError::GraphNotFound(graph_key.to_string()),
            other => GraphStoreError::Sqlite(other),
        })
}

fn edge_relation(edge_type: &ucm_core::EdgeType) -> String {
    match edge_type {
        ucm_core::EdgeType::Custom(value) => value.clone(),
        _ => serde_json::to_value(edge_type)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .unwrap_or_else(|| format!("{:?}", edge_type).to_lowercase()),
    }
}
