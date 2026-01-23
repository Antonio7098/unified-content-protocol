//! LLM utilities wrapper for Python.

use pyo3::prelude::*;
use ucp_llm::{IdMapper, PromptBuilder, UclCapability};

use crate::document::PyDocument;
use crate::types::PyBlockId;

/// UCL command capability enumeration.
#[pyclass(name = "UclCapability", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyUclCapability {
    Edit = 0,
    Append = 1,
    Move = 2,
    Delete = 3,
    Link = 4,
    Snapshot = 5,
    Transaction = 6,
}

impl From<PyUclCapability> for UclCapability {
    fn from(cap: PyUclCapability) -> Self {
        match cap {
            PyUclCapability::Edit => UclCapability::Edit,
            PyUclCapability::Append => UclCapability::Append,
            PyUclCapability::Move => UclCapability::Move,
            PyUclCapability::Delete => UclCapability::Delete,
            PyUclCapability::Link => UclCapability::Link,
            PyUclCapability::Snapshot => UclCapability::Snapshot,
            PyUclCapability::Transaction => UclCapability::Transaction,
        }
    }
}

impl From<&UclCapability> for PyUclCapability {
    fn from(cap: &UclCapability) -> Self {
        match cap {
            UclCapability::Edit => PyUclCapability::Edit,
            UclCapability::Append => PyUclCapability::Append,
            UclCapability::Move => PyUclCapability::Move,
            UclCapability::Delete => PyUclCapability::Delete,
            UclCapability::Link => PyUclCapability::Link,
            UclCapability::Snapshot => PyUclCapability::Snapshot,
            UclCapability::Transaction => PyUclCapability::Transaction,
        }
    }
}

#[pymethods]
impl PyUclCapability {
    /// Get all available capabilities.
    #[staticmethod]
    fn all() -> Vec<PyUclCapability> {
        vec![
            PyUclCapability::Edit,
            PyUclCapability::Append,
            PyUclCapability::Move,
            PyUclCapability::Delete,
            PyUclCapability::Link,
            PyUclCapability::Snapshot,
            PyUclCapability::Transaction,
        ]
    }

    /// Get the command names for this capability.
    fn command_names(&self) -> Vec<&'static str> {
        let cap: UclCapability = (*self).into();
        cap.command_names()
    }

    /// Get documentation for this capability.
    fn documentation(&self) -> &'static str {
        let cap: UclCapability = (*self).into();
        cap.documentation()
    }

    fn __str__(&self) -> &'static str {
        match self {
            PyUclCapability::Edit => "Edit",
            PyUclCapability::Append => "Append",
            PyUclCapability::Move => "Move",
            PyUclCapability::Delete => "Delete",
            PyUclCapability::Link => "Link",
            PyUclCapability::Snapshot => "Snapshot",
            PyUclCapability::Transaction => "Transaction",
        }
    }

    fn __repr__(&self) -> String {
        format!("UclCapability.{}", self.__str__())
    }
}

/// Bidirectional mapping between BlockIds and short numeric IDs.
///
/// Useful for token-efficient LLM prompts by replacing long block IDs
/// with short numeric identifiers.
#[pyclass(name = "IdMapper")]
pub struct PyIdMapper {
    inner: IdMapper,
}

#[pymethods]
impl PyIdMapper {
    /// Create a new empty IdMapper.
    #[new]
    fn new() -> Self {
        Self {
            inner: IdMapper::new(),
        }
    }

    /// Create a mapper from a document, assigning sequential IDs to all blocks.
    #[staticmethod]
    fn from_document(doc: &PyDocument) -> Self {
        Self {
            inner: IdMapper::from_document(doc.inner()),
        }
    }

    /// Register a BlockId and get its short ID.
    fn register(&mut self, block_id: &PyBlockId) -> u32 {
        self.inner.register(block_id.inner())
    }

    /// Get short ID for a BlockId.
    fn to_short_id(&self, block_id: &PyBlockId) -> Option<u32> {
        self.inner.to_short_id(block_id.inner())
    }

    /// Get BlockId for a short ID.
    fn to_block_id(&self, short_id: u32) -> Option<PyBlockId> {
        self.inner
            .to_block_id(short_id)
            .map(|id| PyBlockId::from(*id))
    }

    /// Convert a string containing block IDs to use short IDs.
    fn shorten_text(&self, text: &str) -> String {
        self.inner.shorten_text(text)
    }

    /// Convert a string containing short IDs back to block IDs.
    fn expand_text(&self, text: &str) -> String {
        self.inner.expand_text(text)
    }

    /// Convert UCL commands from long BlockIds to short numeric IDs.
    fn shorten_ucl(&self, ucl: &str) -> String {
        self.inner.shorten_ucl(ucl)
    }

    /// Convert UCL commands from short numeric IDs back to full BlockIds.
    fn expand_ucl(&self, ucl: &str) -> String {
        self.inner.expand_ucl(ucl)
    }

    /// Estimate token savings from using short IDs.
    ///
    /// Returns (original_tokens, shortened_tokens, savings).
    fn estimate_token_savings(&self, text: &str) -> (usize, usize, usize) {
        self.inner.estimate_token_savings(text)
    }

    /// Generate a normalized document representation for LLM prompts.
    fn document_to_prompt(&self, doc: &PyDocument) -> String {
        self.inner.document_to_prompt(doc.inner())
    }

    /// Get the mapping table as a string (useful for debugging).
    fn mapping_table(&self) -> String {
        self.inner.mapping_table()
    }

    /// Total number of mappings.
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!("IdMapper(mappings={})", self.inner.len())
    }
}

/// Builder for constructing LLM prompts with specific capabilities.
#[pyclass(name = "PromptBuilder")]
#[derive(Clone)]
pub struct PyPromptBuilder {
    inner: PromptBuilder,
}

#[pymethods]
impl PyPromptBuilder {
    /// Create a new prompt builder with no capabilities.
    #[new]
    fn new() -> Self {
        Self {
            inner: PromptBuilder::new(),
        }
    }

    /// Create a builder with all capabilities enabled.
    #[staticmethod]
    fn with_all_capabilities() -> Self {
        Self {
            inner: PromptBuilder::with_all_capabilities(),
        }
    }

    /// Add a single capability.
    fn with_capability(&self, cap: PyUclCapability) -> Self {
        Self {
            inner: self.inner.clone().with_capability(cap.into()),
        }
    }

    /// Add multiple capabilities.
    fn with_capabilities(&self, caps: Vec<PyUclCapability>) -> Self {
        Self {
            inner: self
                .inner
                .clone()
                .with_capabilities(caps.into_iter().map(|c| c.into())),
        }
    }

    /// Remove a capability.
    fn without_capability(&self, cap: PyUclCapability) -> Self {
        Self {
            inner: self.inner.clone().without_capability(cap.into()),
        }
    }

    /// Set custom system context (prepended to prompt).
    fn with_system_context(&self, context: &str) -> Self {
        Self {
            inner: self.inner.clone().with_system_context(context),
        }
    }

    /// Set task-specific context.
    fn with_task_context(&self, context: &str) -> Self {
        Self {
            inner: self.inner.clone().with_task_context(context),
        }
    }

    /// Add a custom rule.
    fn with_rule(&self, rule: &str) -> Self {
        Self {
            inner: self.inner.clone().with_rule(rule),
        }
    }

    /// Enable short ID mode (for token efficiency).
    fn with_short_ids(&self, enabled: bool) -> Self {
        Self {
            inner: self.inner.clone().with_short_ids(enabled),
        }
    }

    /// Build the system prompt.
    fn build_system_prompt(&self) -> String {
        self.inner.build_system_prompt()
    }

    /// Build a complete prompt with document context.
    fn build_prompt(&self, document_description: &str, task: &str) -> String {
        self.inner.build_prompt(document_description, task)
    }

    /// Check if a capability is enabled.
    fn has_capability(&self, cap: PyUclCapability) -> bool {
        self.inner.has_capability(cap.into())
    }

    /// Get a list of enabled capabilities.
    fn capabilities(&self) -> Vec<PyUclCapability> {
        self.inner
            .capabilities()
            .map(PyUclCapability::from)
            .collect()
    }

    fn __repr__(&self) -> String {
        let caps: Vec<_> = self.capabilities().iter().map(|c| c.__str__()).collect();
        format!("PromptBuilder(capabilities=[{}])", caps.join(", "))
    }
}

/// Preset prompt configurations for common use cases.
#[pyclass(name = "PromptPresets")]
pub struct PyPromptPresets;

#[pymethods]
impl PyPromptPresets {
    /// Basic editing only (EDIT, APPEND, DELETE).
    #[staticmethod]
    fn basic_editing() -> PyPromptBuilder {
        PyPromptBuilder {
            inner: ucp_llm::presets::basic_editing(),
        }
    }

    /// Structure manipulation (MOVE, LINK).
    #[staticmethod]
    fn structure_manipulation() -> PyPromptBuilder {
        PyPromptBuilder {
            inner: ucp_llm::presets::structure_manipulation(),
        }
    }

    /// Full document editing (all except transactions).
    #[staticmethod]
    fn full_editing() -> PyPromptBuilder {
        PyPromptBuilder {
            inner: ucp_llm::presets::full_editing(),
        }
    }

    /// Version control focused.
    #[staticmethod]
    fn version_control() -> PyPromptBuilder {
        PyPromptBuilder {
            inner: ucp_llm::presets::version_control(),
        }
    }

    /// Token-efficient mode with short IDs.
    #[staticmethod]
    fn token_efficient() -> PyPromptBuilder {
        PyPromptBuilder {
            inner: ucp_llm::presets::token_efficient(),
        }
    }
}
