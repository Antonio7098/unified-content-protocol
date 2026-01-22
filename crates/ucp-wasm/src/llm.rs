//! LLM utilities wrapper for WASM.

use ucp_llm::{IdMapper, PromptBuilder, UclCapability};
use wasm_bindgen::prelude::*;

use crate::Document;

/// UCL command capability enumeration.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WasmUclCapability {
    Edit = 0,
    Append = 1,
    Move = 2,
    Delete = 3,
    Link = 4,
    Snapshot = 5,
    Transaction = 6,
}

impl From<WasmUclCapability> for UclCapability {
    fn from(cap: WasmUclCapability) -> Self {
        match cap {
            WasmUclCapability::Edit => UclCapability::Edit,
            WasmUclCapability::Append => UclCapability::Append,
            WasmUclCapability::Move => UclCapability::Move,
            WasmUclCapability::Delete => UclCapability::Delete,
            WasmUclCapability::Link => UclCapability::Link,
            WasmUclCapability::Snapshot => UclCapability::Snapshot,
            WasmUclCapability::Transaction => UclCapability::Transaction,
        }
    }
}

impl From<&UclCapability> for WasmUclCapability {
    fn from(cap: &UclCapability) -> Self {
        match cap {
            UclCapability::Edit => WasmUclCapability::Edit,
            UclCapability::Append => WasmUclCapability::Append,
            UclCapability::Move => WasmUclCapability::Move,
            UclCapability::Delete => WasmUclCapability::Delete,
            UclCapability::Link => WasmUclCapability::Link,
            UclCapability::Snapshot => WasmUclCapability::Snapshot,
            UclCapability::Transaction => WasmUclCapability::Transaction,
        }
    }
}

/// Bidirectional mapping between BlockIds and short numeric IDs.
///
/// Useful for token-efficient LLM prompts by replacing long block IDs
/// with short numeric identifiers.
#[wasm_bindgen(js_name = IdMapper)]
pub struct WasmIdMapper {
    inner: IdMapper,
}

#[wasm_bindgen(js_class = IdMapper)]
impl WasmIdMapper {
    /// Create a new empty IdMapper.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: IdMapper::new(),
        }
    }

    /// Create a mapper from a document, assigning sequential IDs to all blocks.
    #[wasm_bindgen(js_name = fromDocument)]
    pub fn from_document(doc: &Document) -> Self {
        Self {
            inner: IdMapper::from_document(doc.inner()),
        }
    }

    /// Register a BlockId and get its short ID.
    pub fn register(&mut self, block_id: &str) -> Result<u32, JsValue> {
        let id: ucm_core::BlockId = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;
        Ok(self.inner.register(&id))
    }

    /// Get short ID for a BlockId.
    #[wasm_bindgen(js_name = toShortId)]
    pub fn to_short_id(&self, block_id: &str) -> Result<Option<u32>, JsValue> {
        let id: ucm_core::BlockId = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;
        Ok(self.inner.to_short_id(&id))
    }

    /// Get BlockId for a short ID.
    #[wasm_bindgen(js_name = toBlockId)]
    pub fn to_block_id(&self, short_id: u32) -> Option<String> {
        self.inner.to_block_id(short_id).map(|id| id.to_string())
    }

    /// Convert a string containing block IDs to use short IDs.
    #[wasm_bindgen(js_name = shortenText)]
    pub fn shorten_text(&self, text: &str) -> String {
        self.inner.shorten_text(text)
    }

    /// Convert a string containing short IDs back to block IDs.
    #[wasm_bindgen(js_name = expandText)]
    pub fn expand_text(&self, text: &str) -> String {
        self.inner.expand_text(text)
    }

    /// Convert UCL commands from long BlockIds to short numeric IDs.
    #[wasm_bindgen(js_name = shortenUcl)]
    pub fn shorten_ucl(&self, ucl: &str) -> String {
        self.inner.shorten_ucl(ucl)
    }

    /// Convert UCL commands from short numeric IDs back to full BlockIds.
    #[wasm_bindgen(js_name = expandUcl)]
    pub fn expand_ucl(&self, ucl: &str) -> String {
        self.inner.expand_ucl(ucl)
    }

    /// Estimate token savings from using short IDs.
    /// Returns { originalTokens, shortenedTokens, savings }.
    #[wasm_bindgen(js_name = estimateTokenSavings)]
    pub fn estimate_token_savings(&self, text: &str) -> JsValue {
        let (original, shortened, savings) = self.inner.estimate_token_savings(text);
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("originalTokens"),
            &JsValue::from_f64(original as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("shortenedTokens"),
            &JsValue::from_f64(shortened as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("savings"),
            &JsValue::from_f64(savings as f64),
        );
        obj.into()
    }

    /// Generate a normalized document representation for LLM prompts.
    #[wasm_bindgen(js_name = documentToPrompt)]
    pub fn document_to_prompt(&self, doc: &Document) -> String {
        self.inner.document_to_prompt(doc.inner())
    }

    /// Get the mapping table as a string (useful for debugging).
    #[wasm_bindgen(js_name = mappingTable)]
    pub fn mapping_table(&self) -> String {
        self.inner.mapping_table()
    }

    /// Total number of mappings.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
}

impl Default for WasmIdMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing LLM prompts with specific capabilities.
#[wasm_bindgen(js_name = PromptBuilder)]
pub struct WasmPromptBuilder {
    inner: PromptBuilder,
}

#[wasm_bindgen(js_class = PromptBuilder)]
impl WasmPromptBuilder {
    /// Create a new prompt builder with no capabilities.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: PromptBuilder::new(),
        }
    }

    /// Create a builder with all capabilities enabled.
    #[wasm_bindgen(js_name = withAllCapabilities)]
    pub fn with_all_capabilities() -> Self {
        Self {
            inner: PromptBuilder::with_all_capabilities(),
        }
    }

    /// Add a single capability.
    #[wasm_bindgen(js_name = withCapability)]
    pub fn with_capability(&self, cap: WasmUclCapability) -> Self {
        Self {
            inner: self.inner.clone().with_capability(cap.into()),
        }
    }

    /// Remove a capability.
    #[wasm_bindgen(js_name = withoutCapability)]
    pub fn without_capability(&self, cap: WasmUclCapability) -> Self {
        Self {
            inner: self.inner.clone().without_capability(cap.into()),
        }
    }

    /// Set custom system context (prepended to prompt).
    #[wasm_bindgen(js_name = withSystemContext)]
    pub fn with_system_context(&self, context: &str) -> Self {
        Self {
            inner: self.inner.clone().with_system_context(context),
        }
    }

    /// Set task-specific context.
    #[wasm_bindgen(js_name = withTaskContext)]
    pub fn with_task_context(&self, context: &str) -> Self {
        Self {
            inner: self.inner.clone().with_task_context(context),
        }
    }

    /// Add a custom rule.
    #[wasm_bindgen(js_name = withRule)]
    pub fn with_rule(&self, rule: &str) -> Self {
        Self {
            inner: self.inner.clone().with_rule(rule),
        }
    }

    /// Enable short ID mode (for token efficiency).
    #[wasm_bindgen(js_name = withShortIds)]
    pub fn with_short_ids(&self, enabled: bool) -> Self {
        Self {
            inner: self.inner.clone().with_short_ids(enabled),
        }
    }

    /// Build the system prompt.
    #[wasm_bindgen(js_name = buildSystemPrompt)]
    pub fn build_system_prompt(&self) -> String {
        self.inner.build_system_prompt()
    }

    /// Build a complete prompt with document context.
    #[wasm_bindgen(js_name = buildPrompt)]
    pub fn build_prompt(&self, document_description: &str, task: &str) -> String {
        self.inner.build_prompt(document_description, task)
    }

    /// Check if a capability is enabled.
    #[wasm_bindgen(js_name = hasCapability)]
    pub fn has_capability(&self, cap: WasmUclCapability) -> bool {
        self.inner.has_capability(cap.into())
    }
}

impl Default for WasmPromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset prompt configurations for common use cases.
#[wasm_bindgen(js_name = PromptPresets)]
pub struct WasmPromptPresets;

#[wasm_bindgen(js_class = PromptPresets)]
impl WasmPromptPresets {
    /// Basic editing only (EDIT, APPEND, DELETE).
    #[wasm_bindgen(js_name = basicEditing)]
    pub fn basic_editing() -> WasmPromptBuilder {
        WasmPromptBuilder {
            inner: ucp_llm::presets::basic_editing(),
        }
    }

    /// Structure manipulation (MOVE, LINK).
    #[wasm_bindgen(js_name = structureManipulation)]
    pub fn structure_manipulation() -> WasmPromptBuilder {
        WasmPromptBuilder {
            inner: ucp_llm::presets::structure_manipulation(),
        }
    }

    /// Full document editing (all except transactions).
    #[wasm_bindgen(js_name = fullEditing)]
    pub fn full_editing() -> WasmPromptBuilder {
        WasmPromptBuilder {
            inner: ucp_llm::presets::full_editing(),
        }
    }

    /// Version control focused.
    #[wasm_bindgen(js_name = versionControl)]
    pub fn version_control() -> WasmPromptBuilder {
        WasmPromptBuilder {
            inner: ucp_llm::presets::version_control(),
        }
    }

    /// Token-efficient mode with short IDs.
    #[wasm_bindgen(js_name = tokenEfficient)]
    pub fn token_efficient() -> WasmPromptBuilder {
        WasmPromptBuilder {
            inner: ucp_llm::presets::token_efficient(),
        }
    }
}
