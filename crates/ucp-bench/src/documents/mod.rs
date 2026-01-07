//! Document registry for multi-document benchmarking.

mod llm_benchmark_compendium;
mod ml_tutorial;
mod quickstart_blog;

use crate::suite::result::DocumentSnapshot;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use ucm_core::Document;

#[derive(Clone)]
pub struct DocumentDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub tags: &'static [&'static str],
    builder: fn() -> Document,
    llm_description: fn() -> &'static str,
    ucm_serializer: fn(&Document) -> Value,
}

impl DocumentDefinition {
    pub fn build(&self) -> Document {
        (self.builder)()
    }

    pub fn prompt_text(&self) -> &'static str {
        (self.llm_description)()
    }

    pub fn to_ucm_json(&self, doc: &Document) -> Value {
        (self.ucm_serializer)(doc)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentSummary {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentDetailPayload {
    pub summary: DocumentSummary,
    pub llm_description: String,
    pub snapshot: DocumentSnapshot,
    pub ucm: Value,
}

#[derive(Clone)]
pub struct DocumentRegistry {
    documents: HashMap<&'static str, DocumentDefinition>,
    order: Vec<&'static str>,
}

pub static DOCUMENTS: Lazy<DocumentRegistry> = Lazy::new(DocumentRegistry::new);

impl DocumentRegistry {
    pub fn new() -> Self {
        let mut documents = HashMap::new();
        let mut order = Vec::new();

        for def in Self::builtin_definitions() {
            order.push(def.id);
            documents.insert(def.id, def);
        }

        Self { documents, order }
    }

    pub fn list(&self) -> Vec<DocumentSummary> {
        self.order
            .iter()
            .filter_map(|id| self.documents.get(id))
            .map(|def| DocumentSummary {
                id: def.id.to_string(),
                name: def.name.to_string(),
                summary: def.summary.to_string(),
                tags: def.tags.iter().map(|t| t.to_string()).collect(),
            })
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<DocumentDefinition> {
        self.documents.get(id).cloned()
    }

    pub fn default(&self) -> Option<DocumentDefinition> {
        self.order
            .first()
            .and_then(|id| self.documents.get(*id).cloned())
    }

    pub fn detail_payload(&self, id: &str) -> Option<DocumentDetailPayload> {
        let def = self.get(id)?;
        let doc = def.build();
        let snapshot = DocumentSnapshot::from_document(&doc);
        let ucm = def.to_ucm_json(&doc);
        Some(DocumentDetailPayload {
            summary: DocumentSummary {
                id: def.id.to_string(),
                name: def.name.to_string(),
                summary: def.summary.to_string(),
                tags: def.tags.iter().map(|t| t.to_string()).collect(),
            },
            llm_description: def.prompt_text().to_string(),
            snapshot,
            ucm,
        })
    }

    pub fn builtin_ids(&self) -> Vec<String> {
        self.order.iter().map(|id| id.to_string()).collect()
    }

    fn builtin_definitions() -> Vec<DocumentDefinition> {
        vec![
            llm_benchmark_compendium::definition(),
            ml_tutorial::definition(),
            quickstart_blog::definition(),
        ]
    }
}
