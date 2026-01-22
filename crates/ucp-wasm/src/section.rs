//! Section management bindings for WASM.

use wasm_bindgen::prelude::*;
use ucm_engine::section::{ClearResult, DeletedContent};

use crate::Document;

/// Result of a section clear operation with undo support.
#[wasm_bindgen]
pub struct WasmClearResult {
    removed_ids: Vec<String>,
    deleted_content: WasmDeletedContent,
}

#[wasm_bindgen]
impl WasmClearResult {
    /// Get the IDs of removed blocks.
    #[wasm_bindgen(getter, js_name = removedIds)]
    pub fn removed_ids(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in &self.removed_ids {
            arr.push(&JsValue::from_str(id));
        }
        arr
    }

    /// Get the deleted content for potential restoration.
    #[wasm_bindgen(getter, js_name = deletedContent)]
    pub fn deleted_content(&self) -> WasmDeletedContent {
        self.deleted_content.clone()
    }

    /// Get the number of removed blocks.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.removed_ids.len()
    }
}

impl From<ClearResult> for WasmClearResult {
    fn from(result: ClearResult) -> Self {
        Self {
            removed_ids: result.removed_ids.iter().map(|id| id.to_string()).collect(),
            deleted_content: WasmDeletedContent::from(result.deleted_content),
        }
    }
}

/// Deleted content that can be restored.
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmDeletedContent {
    inner: DeletedContent,
}

impl WasmDeletedContent {
    pub fn inner(&self) -> &DeletedContent {
        &self.inner
    }
}

impl From<DeletedContent> for WasmDeletedContent {
    fn from(deleted: DeletedContent) -> Self {
        Self { inner: deleted }
    }
}

#[wasm_bindgen]
impl WasmDeletedContent {
    /// Check if there is any deleted content.
    #[wasm_bindgen(getter, js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the number of deleted blocks.
    #[wasm_bindgen(getter, js_name = blockCount)]
    pub fn block_count(&self) -> usize {
        self.inner.block_count()
    }

    /// Get all block IDs in the deleted content.
    #[wasm_bindgen(js_name = blockIds)]
    pub fn block_ids(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for id in self.inner.block_ids() {
            arr.push(&JsValue::from_str(&id.to_string()));
        }
        arr
    }

    /// Get the parent block ID where this content was attached.
    #[wasm_bindgen(getter, js_name = parentId)]
    pub fn parent_id(&self) -> String {
        self.inner.parent_id.to_string()
    }

    /// Get the deletion timestamp as ISO 8601 string.
    #[wasm_bindgen(getter, js_name = deletedAt)]
    pub fn deleted_at(&self) -> String {
        self.inner.deleted_at.to_rfc3339()
    }

    /// Serialize to JSON string for persistence.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.inner)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Deserialize from JSON string.
    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json_str: &str) -> Result<WasmDeletedContent, JsValue> {
        let deleted: DeletedContent = serde_json::from_str(json_str)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self { inner: deleted })
    }
}

/// Clear a section's content with undo support.
#[wasm_bindgen(js_name = clearSectionWithUndo)]
pub fn clear_section_with_undo(doc: &mut Document, section_id: &str) -> Result<WasmClearResult, JsValue> {
    let block_id: ucm_core::BlockId = section_id
        .parse()
        .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", section_id)))?;
    
    let result = ucm_engine::section::clear_section_content_with_undo(doc.inner_mut(), &block_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(WasmClearResult::from(result))
}

/// Restore previously deleted section content.
#[wasm_bindgen(js_name = restoreDeletedSection)]
pub fn restore_deleted_section(doc: &mut Document, deleted: &WasmDeletedContent) -> Result<js_sys::Array, JsValue> {
    let restored = ucm_engine::section::restore_deleted_content(doc.inner_mut(), deleted.inner())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let arr = js_sys::Array::new();
    for id in restored {
        arr.push(&JsValue::from_str(&id.to_string()));
    }
    Ok(arr)
}

/// Find a section by path (e.g., "Introduction > Getting Started").
#[wasm_bindgen(js_name = findSectionByPath)]
pub fn find_section_by_path(doc: &Document, path: &str) -> Option<String> {
    ucm_engine::section::find_section_by_path(doc.inner(), path).map(|id| id.to_string())
}

/// Get all sections (heading blocks) in the document.
#[wasm_bindgen(js_name = getAllSections)]
pub fn get_all_sections(doc: &Document) -> JsValue {
    let sections: Vec<_> = ucm_engine::section::get_all_sections(doc.inner())
        .into_iter()
        .map(|(id, level)| {
            serde_json::json!({
                "id": id.to_string(),
                "level": level
            })
        })
        .collect();
    
    serde_wasm_bindgen::to_value(&sections).unwrap_or(JsValue::NULL)
}

/// Get the depth of a section in the document hierarchy.
#[wasm_bindgen(js_name = getSectionDepth)]
pub fn get_section_depth(doc: &Document, section_id: &str) -> Result<Option<usize>, JsValue> {
    let block_id: ucm_core::BlockId = section_id
        .parse()
        .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", section_id)))?;
    
    Ok(ucm_engine::section::get_section_depth(doc.inner(), &block_id))
}
