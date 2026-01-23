//! Document type wrapper for Python.

use pyo3::prelude::*;
use ucm_core::{Block, Content, Document, Edge, EdgeType};

use crate::block::PyBlock;
use crate::content::PyContent;
use crate::edge::PyEdgeType;
use crate::errors::IntoPyResult;
use crate::section::{write_section as write_section_fn, PyWriteSectionResult};
use crate::types::PyBlockId;

/// A UCM document is a collection of blocks with hierarchical structure.
#[pyclass(name = "Document")]
pub struct PyDocument {
    inner: Document,
}

impl PyDocument {
    pub fn new(doc: Document) -> Self {
        Self { inner: doc }
    }

    pub fn inner(&self) -> &Document {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Document {
        &mut self.inner
    }
}

#[pymethods]
impl PyDocument {
    /// Create a new empty document.
    #[staticmethod]
    #[pyo3(signature = (title=None))]
    fn create(title: Option<&str>) -> Self {
        let mut doc = Document::create();
        if let Some(t) = title {
            doc.metadata.title = Some(t.to_string());
        }
        PyDocument::new(doc)
    }

    /// Get the document ID.
    #[getter]
    fn id(&self) -> String {
        self.inner.id.0.clone()
    }

    /// Get the root block ID.
    #[getter]
    fn root_id(&self) -> PyBlockId {
        PyBlockId::from(self.inner.root)
    }

    /// Get the document title.
    #[getter]
    fn title(&self) -> Option<String> {
        self.inner.metadata.title.clone()
    }

    /// Set the document title.
    #[setter]
    fn set_title(&mut self, title: Option<String>) {
        self.inner.metadata.title = title;
    }

    /// Get the document description.
    #[getter]
    fn description(&self) -> Option<String> {
        self.inner.metadata.description.clone()
    }

    /// Set the document description.
    #[setter]
    fn set_description(&mut self, description: Option<String>) {
        self.inner.metadata.description = description;
    }

    /// Get the total block count.
    #[getter]
    fn block_count(&self) -> usize {
        self.inner.block_count()
    }

    /// Get a block by ID.
    fn get_block(&self, id: &PyBlockId) -> Option<PyBlock> {
        self.inner.get_block(id.inner()).map(PyBlock::from)
    }

    /// Get the children of a block.
    fn children(&self, parent_id: &PyBlockId) -> Vec<PyBlockId> {
        self.inner
            .children(parent_id.inner())
            .iter()
            .map(|id| PyBlockId::from(*id))
            .collect()
    }

    /// Get the parent of a block.
    fn parent(&self, child_id: &PyBlockId) -> Option<PyBlockId> {
        self.inner
            .parent(child_id.inner())
            .map(|id| PyBlockId::from(*id))
    }

    /// Get all ancestors of a block (from parent to root).
    fn ancestors(&self, id: &PyBlockId) -> Vec<PyBlockId> {
        let mut result = Vec::new();
        let mut current = self.inner.parent(id.inner()).cloned();
        while let Some(parent_id) = current {
            result.push(PyBlockId::from(parent_id));
            current = self.inner.parent(&parent_id).cloned();
        }
        result
    }

    /// Get all descendants of a block.
    fn descendants(&self, id: &PyBlockId) -> Vec<PyBlockId> {
        self.inner
            .descendants(id.inner())
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    /// Check if a block is reachable from root.
    fn is_reachable(&self, id: &PyBlockId) -> bool {
        self.inner.is_reachable(id.inner())
    }

    /// Check if one block is an ancestor of another.
    fn is_ancestor(&self, potential_ancestor: &PyBlockId, block: &PyBlockId) -> bool {
        self.inner
            .is_ancestor(potential_ancestor.inner(), block.inner())
    }

    /// Add a new block to the document.
    #[pyo3(signature = (parent_id, content, role=None, label=None, tags=None))]
    fn add_block(
        &mut self,
        parent_id: &PyBlockId,
        content: &str,
        role: Option<&str>,
        label: Option<&str>,
        tags: Option<Vec<String>>,
    ) -> PyResult<PyBlockId> {
        let mut block = Block::new(Content::text(content), role);
        if let Some(l) = label {
            block.metadata.label = Some(l.to_string());
        }
        if let Some(t) = tags {
            block.metadata.tags = t;
        }
        let id = self
            .inner
            .add_block(block, parent_id.inner())
            .into_py_result()?;
        Ok(PyBlockId::from(id))
    }

    /// Add a new block with specific content type.
    #[pyo3(signature = (parent_id, content, role=None, label=None, index=None))]
    fn add_block_with_content(
        &mut self,
        parent_id: &PyBlockId,
        content: &PyContent,
        role: Option<&str>,
        label: Option<&str>,
        index: Option<usize>,
    ) -> PyResult<PyBlockId> {
        let mut block = Block::new(content.inner().clone(), role);
        if let Some(l) = label {
            block.metadata.label = Some(l.to_string());
        }

        let id = if let Some(idx) = index {
            self.inner
                .add_block_at(block, parent_id.inner(), idx)
                .into_py_result()?
        } else {
            self.inner
                .add_block(block, parent_id.inner())
                .into_py_result()?
        };
        Ok(PyBlockId::from(id))
    }

    /// Add a code block.
    #[pyo3(signature = (parent_id, language, source, label=None))]
    fn add_code(
        &mut self,
        parent_id: &PyBlockId,
        language: &str,
        source: &str,
        label: Option<&str>,
    ) -> PyResult<PyBlockId> {
        let mut block = Block::new(Content::code(language, source), None);
        if let Some(l) = label {
            block.metadata.label = Some(l.to_string());
        }
        let id = self
            .inner
            .add_block(block, parent_id.inner())
            .into_py_result()?;
        Ok(PyBlockId::from(id))
    }

    /// Edit a block's content.
    #[pyo3(signature = (id, content, role=None))]
    fn edit_block(&mut self, id: &PyBlockId, content: &str, role: Option<&str>) -> PyResult<()> {
        let block = self
            .inner
            .get_block_mut(id.inner())
            .ok_or_else(|| crate::errors::PyBlockNotFoundError::new_err(id.to_string_repr()))?;
        block.update_content(Content::text(content), role);
        Ok(())
    }

    /// Edit a block with specific content.
    #[pyo3(signature = (id, content, role=None))]
    fn edit_block_content(
        &mut self,
        id: &PyBlockId,
        content: &PyContent,
        role: Option<&str>,
    ) -> PyResult<()> {
        let block = self
            .inner
            .get_block_mut(id.inner())
            .ok_or_else(|| crate::errors::PyBlockNotFoundError::new_err(id.to_string_repr()))?;
        block.update_content(content.inner().clone(), role);
        Ok(())
    }

    /// Move a block to a new parent.
    #[pyo3(signature = (id, new_parent_id, index=None))]
    fn move_block(
        &mut self,
        id: &PyBlockId,
        new_parent_id: &PyBlockId,
        index: Option<usize>,
    ) -> PyResult<()> {
        if let Some(idx) = index {
            self.inner
                .move_block_at(id.inner(), new_parent_id.inner(), idx)
                .into_py_result()
        } else {
            self.inner
                .move_block(id.inner(), new_parent_id.inner())
                .into_py_result()
        }
    }

    /// Delete a block.
    #[pyo3(signature = (id, cascade=false))]
    fn delete_block(&mut self, id: &PyBlockId, cascade: bool) -> PyResult<Vec<PyBlockId>> {
        let deleted = if cascade {
            self.inner.delete_cascade(id.inner()).into_py_result()?
        } else {
            vec![self.inner.delete_block(id.inner()).into_py_result()?]
        };
        Ok(deleted.into_iter().map(|b| PyBlockId::from(b.id)).collect())
    }

    /// Add a tag to a block.
    fn add_tag(&mut self, id: &PyBlockId, tag: &str) -> PyResult<()> {
        let block = self
            .inner
            .get_block_mut(id.inner())
            .ok_or_else(|| crate::errors::PyBlockNotFoundError::new_err(id.to_string_repr()))?;
        if !block.metadata.tags.contains(&tag.to_string()) {
            block.metadata.tags.push(tag.to_string());
        }
        Ok(())
    }

    /// Remove a tag from a block.
    fn remove_tag(&mut self, id: &PyBlockId, tag: &str) -> PyResult<bool> {
        let block = self
            .inner
            .get_block_mut(id.inner())
            .ok_or_else(|| crate::errors::PyBlockNotFoundError::new_err(id.to_string_repr()))?;
        let len_before = block.metadata.tags.len();
        block.metadata.tags.retain(|t| t != tag);
        Ok(block.metadata.tags.len() < len_before)
    }

    /// Set a block's label.
    #[pyo3(signature = (id, label=None))]
    fn set_label(&mut self, id: &PyBlockId, label: Option<&str>) -> PyResult<()> {
        let block = self
            .inner
            .get_block_mut(id.inner())
            .ok_or_else(|| crate::errors::PyBlockNotFoundError::new_err(id.to_string_repr()))?;
        block.metadata.label = label.map(|s| s.to_string());
        Ok(())
    }

    /// Add an edge to a block.
    fn add_edge(
        &mut self,
        source_id: &PyBlockId,
        edge_type: PyEdgeType,
        target_id: &PyBlockId,
    ) -> PyResult<()> {
        let et: EdgeType = edge_type.into();
        let edge = Edge::new(et, *target_id.inner());

        let block = self.inner.get_block_mut(source_id.inner()).ok_or_else(|| {
            crate::errors::PyBlockNotFoundError::new_err(source_id.to_string_repr())
        })?;
        block.add_edge(edge.clone());

        // Also update edge index
        self.inner.edge_index.add_edge(source_id.inner(), &edge);
        Ok(())
    }

    /// Remove an edge from a block.
    fn remove_edge(
        &mut self,
        source_id: &PyBlockId,
        edge_type: PyEdgeType,
        target_id: &PyBlockId,
    ) -> PyResult<bool> {
        let et: EdgeType = edge_type.into();

        let block = self.inner.get_block_mut(source_id.inner()).ok_or_else(|| {
            crate::errors::PyBlockNotFoundError::new_err(source_id.to_string_repr())
        })?;
        let removed = block.remove_edge(target_id.inner(), &et);

        if removed {
            self.inner
                .edge_index
                .remove_edge(source_id.inner(), target_id.inner(), &et);
        }
        Ok(removed)
    }

    /// Find blocks by tag.
    fn find_by_tag(&self, tag: &str) -> Vec<PyBlockId> {
        self.inner
            .indices
            .find_by_tag(tag)
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    /// Find blocks by content type.
    fn find_by_type(&self, content_type: &str) -> Vec<PyBlockId> {
        self.inner
            .indices
            .find_by_type(content_type)
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    /// Find a block by label.
    fn find_by_label(&self, label: &str) -> Option<PyBlockId> {
        self.inner.indices.find_by_label(label).map(PyBlockId::from)
    }

    /// Get outgoing edges from a block.
    fn outgoing_edges(&self, id: &PyBlockId) -> Vec<(PyEdgeType, PyBlockId)> {
        self.inner
            .edge_index
            .outgoing_from(id.inner())
            .iter()
            .map(|(et, target)| (PyEdgeType::from(et), PyBlockId::from(*target)))
            .collect()
    }

    /// Get incoming edges to a block.
    fn incoming_edges(&self, id: &PyBlockId) -> Vec<(PyEdgeType, PyBlockId)> {
        self.inner
            .edge_index
            .incoming_to(id.inner())
            .iter()
            .map(|(et, source)| (PyEdgeType::from(et), PyBlockId::from(*source)))
            .collect()
    }

    /// Find orphaned blocks (unreachable from root).
    fn find_orphans(&self) -> Vec<PyBlockId> {
        self.inner
            .find_orphans()
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    /// Prune unreachable blocks.
    fn prune_unreachable(&mut self) -> Vec<PyBlockId> {
        self.inner
            .prune_unreachable()
            .into_iter()
            .map(|b| PyBlockId::from(b.id))
            .collect()
    }

    /// Validate the document structure.
    fn validate(&self) -> Vec<(String, String, String)> {
        self.inner
            .validate()
            .into_iter()
            .map(|issue| {
                (
                    format!("{:?}", issue.severity),
                    issue.code.code().to_string(),
                    issue.message,
                )
            })
            .collect()
    }

    /// Get all block IDs in the document.
    fn block_ids(&self) -> Vec<PyBlockId> {
        self.inner
            .blocks
            .keys()
            .map(|id| PyBlockId::from(*id))
            .collect()
    }

    /// Iterate over all blocks.
    #[getter]
    fn blocks(&self) -> Vec<PyBlock> {
        self.inner.blocks.values().map(PyBlock::from).collect()
    }

    /// Serialize to JSON string.
    fn to_json(&self) -> PyResult<String> {
        // Create a serializable representation
        let blocks: Vec<_> = self.inner.blocks.values().collect();
        serde_json::to_string_pretty(&serde_json::json!({
            "id": self.inner.id.0,
            "root": self.inner.root.to_string(),
            "structure": self.inner.structure.iter()
                .map(|(k, v)| (k.to_string(), v.iter().map(|id| id.to_string()).collect::<Vec<_>>()))
                .collect::<std::collections::HashMap<_, _>>(),
            "blocks": blocks,
            "metadata": {
                "title": self.inner.metadata.title,
                "description": self.inner.metadata.description,
                "authors": self.inner.metadata.authors,
                "created_at": self.inner.metadata.created_at.to_rfc3339(),
                "modified_at": self.inner.metadata.modified_at.to_rfc3339(),
            }
        }))
        .map_err(|e| crate::errors::PyUcpError::new_err(format!("Serialization error: {}", e)))
    }

    /// Get document version.
    #[getter]
    fn version(&self) -> u64 {
        self.inner.version.counter
    }

    /// Get created timestamp as ISO 8601 string.
    #[getter]
    fn created_at(&self) -> String {
        self.inner.metadata.created_at.to_rfc3339()
    }

    /// Get modified timestamp as ISO 8601 string.
    #[getter]
    fn modified_at(&self) -> String {
        self.inner.metadata.modified_at.to_rfc3339()
    }

    /// Get the siblings of a block (children of same parent, excluding self).
    fn siblings(&self, id: &PyBlockId) -> Vec<PyBlockId> {
        if let Some(parent_id) = self.inner.parent(id.inner()) {
            self.inner
                .children(parent_id)
                .iter()
                .filter(|&child_id| child_id != id.inner())
                .map(|child_id| PyBlockId::from(*child_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get the depth of a block from the root (root has depth 0).
    fn depth(&self, id: &PyBlockId) -> usize {
        let mut depth = 0;
        let mut current = self.inner.parent(id.inner()).cloned();
        while let Some(parent_id) = current {
            depth += 1;
            current = self.inner.parent(&parent_id).cloned();
        }
        depth
    }

    /// Find blocks by semantic role.
    fn find_by_role(&self, role: &str) -> Vec<PyBlockId> {
        self.inner
            .blocks
            .values()
            .filter(|block| {
                block
                    .metadata
                    .semantic_role
                    .as_ref()
                    .map(|r| r.to_string() == role)
                    .unwrap_or(false)
            })
            .map(|block| PyBlockId::from(block.id))
            .collect()
    }

    /// Get the path from root to a block (list of block IDs).
    fn path_from_root(&self, id: &PyBlockId) -> Vec<PyBlockId> {
        let mut path = Vec::new();
        let mut current = Some(*id.inner());
        while let Some(block_id) = current {
            path.push(PyBlockId::from(block_id));
            current = self.inner.parent(&block_id).cloned();
        }
        path.reverse();
        path
    }

    /// Get the index of a block among its siblings.
    fn sibling_index(&self, id: &PyBlockId) -> Option<usize> {
        if let Some(parent_id) = self.inner.parent(id.inner()) {
            self.inner
                .children(parent_id)
                .iter()
                .position(|child_id| child_id == id.inner())
        } else {
            None
        }
    }

    /// Write markdown content into a section by block ID.
    #[pyo3(signature = (section_id, markdown, base_heading_level=None))]
    fn write_section(
        &mut self,
        section_id: &PyBlockId,
        markdown: &str,
        base_heading_level: Option<usize>,
    ) -> PyResult<PyWriteSectionResult> {
        write_section_fn(self, section_id, markdown, base_heading_level)
    }

    fn __repr__(&self) -> String {
        format!(
            "Document(id={}, blocks={}, title={:?})",
            self.inner.id,
            self.inner.block_count(),
            self.inner.metadata.title
        )
    }

    fn __len__(&self) -> usize {
        self.inner.block_count()
    }
}
