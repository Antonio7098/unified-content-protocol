//! Document type wrapper for WASM.

use wasm_bindgen::prelude::*;

use crate::errors::IntoWasmResult;
use crate::types::{Content, EdgeType};

/// A UCM document is a collection of blocks with hierarchical structure.
#[wasm_bindgen]
pub struct Document {
    inner: ucm_core::Document,
}

impl Document {
    pub fn new(doc: ucm_core::Document) -> Self {
        Self { inner: doc }
    }

    pub fn inner(&self) -> &ucm_core::Document {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut ucm_core::Document {
        &mut self.inner
    }
}

#[wasm_bindgen]
impl Document {
    /// Create a new empty document.
    #[wasm_bindgen(constructor)]
    pub fn create(title: Option<String>) -> Document {
        let mut doc = ucm_core::Document::create();
        if let Some(t) = title {
            doc.metadata.title = Some(t);
        }
        Document::new(doc)
    }

    /// Get the document ID.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.0.clone()
    }

    /// Get the root block ID.
    #[wasm_bindgen(getter, js_name = rootId)]
    pub fn root_id(&self) -> String {
        self.inner.root.to_string()
    }

    /// Get the document title.
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<String> {
        self.inner.metadata.title.clone()
    }

    /// Set the document title.
    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, title: Option<String>) {
        self.inner.metadata.title = title;
    }

    /// Get the total block count.
    #[wasm_bindgen(js_name = blockCount)]
    pub fn block_count(&self) -> usize {
        self.inner.block_count()
    }

    /// Get a block by ID (returns JSON representation).
    #[wasm_bindgen(js_name = getBlock)]
    pub fn get_block(&self, id: &str) -> Result<JsValue, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        match self.inner.get_block(&block_id) {
            Some(block) => {
                let obj = js_sys::Object::new();
                
                // Set id
                js_sys::Reflect::set(&obj, &JsValue::from_str("id"), &JsValue::from_str(&block.id.to_string()))?;
                
                // Set contentType
                js_sys::Reflect::set(&obj, &JsValue::from_str("contentType"), &JsValue::from_str(block.content_type()))?;
                
                // Set text/language/source based on content type
                match &block.content {
                    ucm_core::Content::Text(t) => {
                        js_sys::Reflect::set(&obj, &JsValue::from_str("text"), &JsValue::from_str(&t.text))?;
                    }
                    ucm_core::Content::Code(c) => {
                        js_sys::Reflect::set(&obj, &JsValue::from_str("language"), &JsValue::from_str(&c.language))?;
                        js_sys::Reflect::set(&obj, &JsValue::from_str("source"), &JsValue::from_str(&c.source))?;
                    }
                    _ => {}
                }
                
                // Set role
                if let Some(role) = &block.metadata.semantic_role {
                    js_sys::Reflect::set(&obj, &JsValue::from_str("role"), &JsValue::from_str(role.category.as_str()))?;
                }
                
                // Set label
                if let Some(label) = &block.metadata.label {
                    js_sys::Reflect::set(&obj, &JsValue::from_str("label"), &JsValue::from_str(label))?;
                }
                
                // Set tags
                let tags_arr = js_sys::Array::new();
                for tag in &block.metadata.tags {
                    tags_arr.push(&JsValue::from_str(tag));
                }
                js_sys::Reflect::set(&obj, &JsValue::from_str("tags"), &tags_arr)?;
                
                // Set version
                js_sys::Reflect::set(&obj, &JsValue::from_str("version"), &JsValue::from_f64(block.version.counter as f64))?;
                
                Ok(obj.into())
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Get children of a block.
    #[wasm_bindgen(js_name = children)]
    pub fn children(&self, parent_id: &str) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", parent_id)))?;

        let arr = js_sys::Array::new();
        for child_id in self.inner.children(&block_id) {
            arr.push(&JsValue::from_str(&child_id.to_string()));
        }
        Ok(arr)
    }

    /// Get parent of a block.
    #[wasm_bindgen(js_name = parent)]
    pub fn parent(&self, child_id: &str) -> Result<Option<String>, JsValue> {
        let block_id: ucm_core::BlockId = child_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", child_id)))?;

        Ok(self.inner.parent(&block_id).map(|id| id.to_string()))
    }

    /// Get descendants of a block.
    #[wasm_bindgen(js_name = descendants)]
    pub fn descendants(&self, id: &str) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let arr = js_sys::Array::new();
        for desc_id in self.inner.descendants(&block_id) {
            arr.push(&JsValue::from_str(&desc_id.to_string()));
        }
        Ok(arr)
    }

    /// Add a new text block.
    #[wasm_bindgen(js_name = addBlock)]
    pub fn add_block(
        &mut self,
        parent_id: &str,
        content: &str,
        role: Option<String>,
        label: Option<String>,
    ) -> Result<String, JsValue> {
        let parent: ucm_core::BlockId = parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", parent_id)))?;

        let mut block = ucm_core::Block::new(
            ucm_core::Content::text(content),
            role.as_deref(),
        );
        if let Some(l) = label {
            block.metadata.label = Some(l);
        }

        let id = self.inner.add_block(block, &parent).into_wasm_result()?;
        Ok(id.to_string())
    }

    /// Add a block with specific content.
    #[wasm_bindgen(js_name = addBlockWithContent)]
    pub fn add_block_with_content(
        &mut self,
        parent_id: &str,
        content: &Content,
        role: Option<String>,
        label: Option<String>,
    ) -> Result<String, JsValue> {
        let parent: ucm_core::BlockId = parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", parent_id)))?;

        let mut block = ucm_core::Block::new(content.inner().clone(), role.as_deref());
        if let Some(l) = label {
            block.metadata.label = Some(l);
        }

        let id = self.inner.add_block(block, &parent).into_wasm_result()?;
        Ok(id.to_string())
    }

    /// Add a code block.
    #[wasm_bindgen(js_name = addCode)]
    pub fn add_code(
        &mut self,
        parent_id: &str,
        language: &str,
        source: &str,
        label: Option<String>,
    ) -> Result<String, JsValue> {
        let parent: ucm_core::BlockId = parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", parent_id)))?;

        let mut block = ucm_core::Block::new(ucm_core::Content::code(language, source), None);
        if let Some(l) = label {
            block.metadata.label = Some(l);
        }

        let id = self.inner.add_block(block, &parent).into_wasm_result()?;
        Ok(id.to_string())
    }

    /// Edit a block's content.
    #[wasm_bindgen(js_name = editBlock)]
    pub fn edit_block(&mut self, id: &str, content: &str, role: Option<String>) -> Result<(), JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let block = self
            .inner
            .get_block_mut(&block_id)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", id)))?;

        block.update_content(ucm_core::Content::text(content), role.as_deref());
        Ok(())
    }

    /// Move a block to a new parent.
    #[wasm_bindgen(js_name = moveBlock)]
    pub fn move_block(&mut self, id: &str, new_parent_id: &str, index: Option<usize>) -> Result<(), JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;
        let new_parent: ucm_core::BlockId = new_parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", new_parent_id)))?;

        if let Some(idx) = index {
            self.inner.move_block_at(&block_id, &new_parent, idx).into_wasm_result()
        } else {
            self.inner.move_block(&block_id, &new_parent).into_wasm_result()
        }
    }

    /// Delete a block.
    #[wasm_bindgen(js_name = deleteBlock)]
    pub fn delete_block(&mut self, id: &str, cascade: Option<bool>) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let deleted = if cascade.unwrap_or(false) {
            self.inner.delete_cascade(&block_id).into_wasm_result()?
        } else {
            vec![self.inner.delete_block(&block_id).into_wasm_result()?]
        };

        let arr = js_sys::Array::new();
        for block in deleted {
            arr.push(&JsValue::from_str(&block.id.to_string()));
        }
        Ok(arr)
    }

    /// Add a tag to a block.
    #[wasm_bindgen(js_name = addTag)]
    pub fn add_tag(&mut self, id: &str, tag: &str) -> Result<(), JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let block = self
            .inner
            .get_block_mut(&block_id)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", id)))?;

        if !block.metadata.tags.contains(&tag.to_string()) {
            block.metadata.tags.push(tag.to_string());
            // Update the tag index
            self.inner.indices.by_tag.entry(tag.to_string()).or_default().insert(block_id);
        }
        Ok(())
    }

    /// Find blocks by tag.
    #[wasm_bindgen(js_name = findByTag)]
    pub fn find_by_tag(&self, tag: &str) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in self.inner.indices.find_by_tag(tag) {
            arr.push(&JsValue::from_str(&id.to_string()));
        }
        arr
    }

    /// Find blocks by content type.
    #[wasm_bindgen(js_name = findByType)]
    pub fn find_by_type(&self, content_type: &str) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in self.inner.indices.find_by_type(content_type) {
            arr.push(&JsValue::from_str(&id.to_string()));
        }
        arr
    }

    /// Find a block by label.
    #[wasm_bindgen(js_name = findByLabel)]
    pub fn find_by_label(&self, label: &str) -> Option<String> {
        self.inner.indices.find_by_label(label).map(|id| id.to_string())
    }

    /// Find orphaned blocks.
    #[wasm_bindgen(js_name = findOrphans)]
    pub fn find_orphans(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in self.inner.find_orphans() {
            arr.push(&JsValue::from_str(&id.to_string()));
        }
        arr
    }

    /// Prune unreachable blocks.
    #[wasm_bindgen(js_name = pruneUnreachable)]
    pub fn prune_unreachable(&mut self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for block in self.inner.prune_unreachable() {
            arr.push(&JsValue::from_str(&block.id.to_string()));
        }
        arr
    }

    /// Validate the document.
    #[wasm_bindgen(js_name = validate)]
    pub fn validate(&self) -> JsValue {
        let issues: Vec<_> = self
            .inner
            .validate()
            .into_iter()
            .map(|issue| {
                serde_json::json!({
                    "severity": format!("{:?}", issue.severity),
                    "code": issue.code.code(),
                    "message": issue.message,
                })
            })
            .collect();

        serde_wasm_bindgen::to_value(&issues).unwrap_or(JsValue::NULL)
    }

    /// Get all block IDs.
    #[wasm_bindgen(js_name = blockIds)]
    pub fn block_ids(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in self.inner.blocks.keys() {
            arr.push(&JsValue::from_str(&id.to_string()));
        }
        arr
    }

    /// Serialize to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        let obj = js_sys::Object::new();
        
        // Set id
        js_sys::Reflect::set(&obj, &JsValue::from_str("id"), &JsValue::from_str(&self.inner.id.0))?;
        
        // Set root
        js_sys::Reflect::set(&obj, &JsValue::from_str("root"), &JsValue::from_str(&self.inner.root.to_string()))?;
        
        // Set blocks as object
        let blocks_obj = js_sys::Object::new();
        for (id, block) in &self.inner.blocks {
            let block_obj = js_sys::Object::new();
            js_sys::Reflect::set(&block_obj, &JsValue::from_str("id"), &JsValue::from_str(&id.to_string()))?;
            js_sys::Reflect::set(&block_obj, &JsValue::from_str("contentType"), &JsValue::from_str(block.content_type()))?;
            
            match &block.content {
                ucm_core::Content::Text(t) => {
                    js_sys::Reflect::set(&block_obj, &JsValue::from_str("text"), &JsValue::from_str(&t.text))?;
                }
                ucm_core::Content::Code(c) => {
                    js_sys::Reflect::set(&block_obj, &JsValue::from_str("language"), &JsValue::from_str(&c.language))?;
                    js_sys::Reflect::set(&block_obj, &JsValue::from_str("source"), &JsValue::from_str(&c.source))?;
                }
                _ => {}
            }
            
            js_sys::Reflect::set(&blocks_obj, &JsValue::from_str(&id.to_string()), &block_obj)?;
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("blocks"), &blocks_obj)?;
        
        // Set structure as object
        let structure_obj = js_sys::Object::new();
        for (parent_id, children) in &self.inner.structure {
            let children_arr = js_sys::Array::new();
            for child_id in children {
                children_arr.push(&JsValue::from_str(&child_id.to_string()));
            }
            js_sys::Reflect::set(&structure_obj, &JsValue::from_str(&parent_id.to_string()), &children_arr)?;
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("structure"), &structure_obj)?;
        
        // Set metadata
        let metadata_obj = js_sys::Object::new();
        if let Some(title) = &self.inner.metadata.title {
            js_sys::Reflect::set(&metadata_obj, &JsValue::from_str("title"), &JsValue::from_str(title))?;
        }
        if let Some(desc) = &self.inner.metadata.description {
            js_sys::Reflect::set(&metadata_obj, &JsValue::from_str("description"), &JsValue::from_str(desc))?;
        }
        js_sys::Reflect::set(&metadata_obj, &JsValue::from_str("createdAt"), &JsValue::from_str(&self.inner.metadata.created_at.to_rfc3339()))?;
        js_sys::Reflect::set(&metadata_obj, &JsValue::from_str("modifiedAt"), &JsValue::from_str(&self.inner.metadata.modified_at.to_rfc3339()))?;
        js_sys::Reflect::set(&obj, &JsValue::from_str("metadata"), &metadata_obj)?;
        
        Ok(obj.into())
    }

    /// Get document version.
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u64 {
        self.inner.version.counter
    }

    /// Get the document description.
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.inner.metadata.description.clone()
    }

    /// Set the document description.
    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, description: Option<String>) {
        self.inner.metadata.description = description;
    }

    /// Get created timestamp as ISO 8601 string.
    #[wasm_bindgen(getter, js_name = createdAt)]
    pub fn created_at(&self) -> String {
        self.inner.metadata.created_at.to_rfc3339()
    }

    /// Get modified timestamp as ISO 8601 string.
    #[wasm_bindgen(getter, js_name = modifiedAt)]
    pub fn modified_at(&self) -> String {
        self.inner.metadata.modified_at.to_rfc3339()
    }

    /// Get all ancestors of a block (from parent to root).
    #[wasm_bindgen(js_name = ancestors)]
    pub fn ancestors(&self, id: &str) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let arr = js_sys::Array::new();
        let mut current = self.inner.parent(&block_id).cloned();
        while let Some(parent_id) = current {
            arr.push(&JsValue::from_str(&parent_id.to_string()));
            current = self.inner.parent(&parent_id).cloned();
        }
        Ok(arr)
    }

    /// Check if a block is reachable from root.
    #[wasm_bindgen(js_name = isReachable)]
    pub fn is_reachable(&self, id: &str) -> Result<bool, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;
        Ok(self.inner.is_reachable(&block_id))
    }

    /// Check if one block is an ancestor of another.
    #[wasm_bindgen(js_name = isAncestor)]
    pub fn is_ancestor(&self, potential_ancestor: &str, block: &str) -> Result<bool, JsValue> {
        let ancestor_id: ucm_core::BlockId = potential_ancestor
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", potential_ancestor)))?;
        let block_id: ucm_core::BlockId = block
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block)))?;
        Ok(self.inner.is_ancestor(&ancestor_id, &block_id))
    }

    /// Add a block at a specific index.
    #[wasm_bindgen(js_name = addBlockAt)]
    pub fn add_block_at(
        &mut self,
        parent_id: &str,
        content: &str,
        index: usize,
        role: Option<String>,
        label: Option<String>,
    ) -> Result<String, JsValue> {
        let parent: ucm_core::BlockId = parent_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", parent_id)))?;

        let mut block = ucm_core::Block::new(
            ucm_core::Content::text(content),
            role.as_deref(),
        );
        if let Some(l) = label {
            block.metadata.label = Some(l);
        }

        let id = self.inner.add_block_at(block, &parent, index).into_wasm_result()?;
        Ok(id.to_string())
    }

    /// Edit a block with specific content type.
    #[wasm_bindgen(js_name = editBlockContent)]
    pub fn edit_block_content(
        &mut self,
        id: &str,
        content: &Content,
        role: Option<String>,
    ) -> Result<(), JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let block = self
            .inner
            .get_block_mut(&block_id)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", id)))?;

        block.update_content(content.inner().clone(), role.as_deref());
        Ok(())
    }

    /// Remove a tag from a block.
    #[wasm_bindgen(js_name = removeTag)]
    pub fn remove_tag(&mut self, id: &str, tag: &str) -> Result<bool, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let block = self
            .inner
            .get_block_mut(&block_id)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", id)))?;

        let len_before = block.metadata.tags.len();
        block.metadata.tags.retain(|t| t != tag);
        let removed = block.metadata.tags.len() < len_before;
        
        if removed {
            // Update the tag index
            if let Some(set) = self.inner.indices.by_tag.get_mut(tag) {
                set.remove(&block_id);
            }
        }
        Ok(removed)
    }

    /// Set a block's label.
    #[wasm_bindgen(js_name = setLabel)]
    pub fn set_label(&mut self, id: &str, label: Option<String>) -> Result<(), JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let block = self
            .inner
            .get_block_mut(&block_id)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", id)))?;

        block.metadata.label = label;
        Ok(())
    }

    /// Add an edge from one block to another.
    #[wasm_bindgen(js_name = addEdge)]
    pub fn add_edge(
        &mut self,
        source_id: &str,
        edge_type: EdgeType,
        target_id: &str,
    ) -> Result<(), JsValue> {
        let source: ucm_core::BlockId = source_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", source_id)))?;
        let target: ucm_core::BlockId = target_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", target_id)))?;

        let et: ucm_core::EdgeType = edge_type.into();
        let edge = ucm_core::Edge::new(et, target);

        let block = self
            .inner
            .get_block_mut(&source)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", source_id)))?;
        block.add_edge(edge.clone());

        // Also update edge index
        self.inner.edge_index.add_edge(&source, &edge);
        Ok(())
    }

    /// Remove an edge from one block to another.
    #[wasm_bindgen(js_name = removeEdge)]
    pub fn remove_edge(
        &mut self,
        source_id: &str,
        edge_type: EdgeType,
        target_id: &str,
    ) -> Result<bool, JsValue> {
        let source: ucm_core::BlockId = source_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", source_id)))?;
        let target: ucm_core::BlockId = target_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", target_id)))?;

        let et: ucm_core::EdgeType = edge_type.into();

        let block = self
            .inner
            .get_block_mut(&source)
            .ok_or_else(|| JsValue::from_str(&format!("Block not found: {}", source_id)))?;
        let removed = block.remove_edge(&target, &et);

        if removed {
            self.inner.edge_index.remove_edge(&source, &target, &et);
        }
        Ok(removed)
    }

    /// Get outgoing edges from a block.
    #[wasm_bindgen(js_name = outgoingEdges)]
    pub fn outgoing_edges(&self, id: &str) -> Result<JsValue, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let edges: Vec<_> = self
            .inner
            .edge_index
            .outgoing_from(&block_id)
            .iter()
            .map(|(et, target)| {
                serde_json::json!({
                    "edgeType": et.as_str(),
                    "target": target.to_string()
                })
            })
            .collect();

        serde_wasm_bindgen::to_value(&edges).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get incoming edges to a block.
    #[wasm_bindgen(js_name = incomingEdges)]
    pub fn incoming_edges(&self, id: &str) -> Result<JsValue, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let edges: Vec<_> = self
            .inner
            .edge_index
            .incoming_to(&block_id)
            .iter()
            .map(|(et, source)| {
                serde_json::json!({
                    "edgeType": et.as_str(),
                    "source": source.to_string()
                })
            })
            .collect();

        serde_wasm_bindgen::to_value(&edges).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get all blocks in the document.
    #[wasm_bindgen(js_name = blocks)]
    pub fn blocks(&self) -> Result<JsValue, JsValue> {
        let blocks: Vec<_> = self
            .inner
            .blocks
            .values()
            .map(|block| {
                serde_json::json!({
                    "id": block.id.to_string(),
                    "contentType": block.content_type(),
                    "content": match &block.content {
                        ucm_core::Content::Text(t) => serde_json::json!({"text": t.text}),
                        ucm_core::Content::Code(c) => serde_json::json!({"language": c.language, "source": c.source}),
                        _ => serde_json::json!({"type": block.content_type()}),
                    },
                    "role": block.metadata.semantic_role.as_ref().map(|r| r.to_string()),
                    "label": block.metadata.label,
                    "tags": block.metadata.tags,
                    "version": block.version.counter,
                })
            })
            .collect();

        serde_wasm_bindgen::to_value(&blocks).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get the siblings of a block (children of same parent, excluding self).
    #[wasm_bindgen(js_name = siblings)]
    pub fn siblings(&self, id: &str) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let arr = js_sys::Array::new();
        if let Some(parent_id) = self.inner.parent(&block_id) {
            for child_id in self.inner.children(parent_id) {
                if child_id != &block_id {
                    arr.push(&JsValue::from_str(&child_id.to_string()));
                }
            }
        }
        Ok(arr)
    }

    /// Get the depth of a block from the root (root has depth 0).
    #[wasm_bindgen(js_name = depth)]
    pub fn depth(&self, id: &str) -> Result<usize, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let mut depth = 0;
        let mut current = self.inner.parent(&block_id).cloned();
        while let Some(parent_id) = current {
            depth += 1;
            current = self.inner.parent(&parent_id).cloned();
        }
        Ok(depth)
    }

    /// Find blocks by semantic role.
    #[wasm_bindgen(js_name = findByRole)]
    pub fn find_by_role(&self, role: &str) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for block in self.inner.blocks.values() {
            if let Some(ref block_role) = block.metadata.semantic_role {
                if block_role.to_string() == role {
                    arr.push(&JsValue::from_str(&block.id.to_string()));
                }
            }
        }
        arr
    }

    /// Get the path from root to a block (list of block IDs).
    #[wasm_bindgen(js_name = pathFromRoot)]
    pub fn path_from_root(&self, id: &str) -> Result<js_sys::Array, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        let mut path = Vec::new();
        let mut current = Some(block_id);
        while let Some(bid) = current {
            path.push(bid.to_string());
            current = self.inner.parent(&bid).cloned();
        }
        path.reverse();

        let arr = js_sys::Array::new();
        for p in path {
            arr.push(&JsValue::from_str(&p));
        }
        Ok(arr)
    }

    /// Get the index of a block among its siblings.
    #[wasm_bindgen(js_name = siblingIndex)]
    pub fn sibling_index(&self, id: &str) -> Result<Option<usize>, JsValue> {
        let block_id: ucm_core::BlockId = id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))?;

        if let Some(parent_id) = self.inner.parent(&block_id) {
            Ok(self
                .inner
                .children(parent_id)
                .iter()
                .position(|child_id| child_id == &block_id))
        } else {
            Ok(None)
        }
    }
}
