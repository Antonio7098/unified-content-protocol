//! Content type wrapper for Python.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use ucm_core::Content;

// Helper for creating a new dict (PyO3 0.22 API)
fn new_dict(py: Python<'_>) -> Bound<'_, PyDict> {
    PyDict::new_bound(py)
}

/// Block content with typed payload.
#[pyclass(name = "Content")]
#[derive(Clone)]
pub struct PyContent(pub(crate) Content);

impl PyContent {
    pub fn inner(&self) -> &Content {
        &self.0
    }
}

impl From<Content> for PyContent {
    fn from(content: Content) -> Self {
        Self(content)
    }
}

#[pymethods]
impl PyContent {
    /// Create plain text content.
    #[staticmethod]
    fn text(text: &str) -> Self {
        PyContent(Content::text(text))
    }

    /// Create markdown text content.
    #[staticmethod]
    fn markdown(text: &str) -> Self {
        PyContent(Content::markdown(text))
    }

    /// Create code content.
    #[staticmethod]
    fn code(language: &str, source: &str) -> Self {
        PyContent(Content::code(language, source))
    }

    /// Create JSON content.
    #[staticmethod]
    fn json(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert Python object to serde_json::Value via JSON string
        let json_str: String = py
            .import_bound("json")?
            .call_method1("dumps", (value,))?
            .extract()?;
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e))
            })?;
        Ok(PyContent(Content::json(json_value)))
    }

    /// Create table content from rows.
    #[staticmethod]
    fn table(rows: Vec<Vec<String>>) -> Self {
        PyContent(Content::table(rows))
    }

    /// Get the content type tag (e.g., "text", "code", "table").
    #[getter]
    fn type_tag(&self) -> &'static str {
        self.0.type_tag()
    }

    /// Check if the content is empty.
    #[getter]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the approximate size in bytes.
    #[getter]
    fn size_bytes(&self) -> usize {
        self.0.size_bytes()
    }

    /// Get the text content if this is a text block.
    fn as_text(&self) -> Option<String> {
        match &self.0 {
            Content::Text(t) => Some(t.text.clone()),
            _ => None,
        }
    }

    /// Get the code source if this is a code block.
    fn as_code(&self) -> Option<(String, String)> {
        match &self.0 {
            Content::Code(c) => Some((c.language.clone(), c.source.clone())),
            _ => None,
        }
    }

    /// Get the JSON value if this is a JSON block.
    fn as_json(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        match &self.0 {
            Content::Json { value, .. } => {
                let json_str = serde_json::to_string(value).map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!("JSON serialization error: {}", e))
                })?;
                let json_module = py.import_bound("json")?;
                let obj = json_module.call_method1("loads", (json_str,))?;
                Ok(Some(obj.into()))
            }
            _ => Ok(None),
        }
    }

    /// Convert content to a Python dict representation.
    fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = new_dict(py);
        dict.set_item("type", self.type_tag())?;

        match &self.0 {
            Content::Text(t) => {
                dict.set_item("text", &t.text)?;
                dict.set_item("format", format!("{:?}", t.format).to_lowercase())?;
            }
            Content::Code(c) => {
                dict.set_item("language", &c.language)?;
                dict.set_item("source", &c.source)?;
            }
            Content::Table(t) => {
                let columns: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
                dict.set_item("columns", columns)?;
                dict.set_item("row_count", t.rows.len())?;
            }
            Content::Math(m) => {
                dict.set_item("expression", &m.expression)?;
                dict.set_item("display_mode", m.display_mode)?;
            }
            Content::Media(m) => {
                dict.set_item("media_type", format!("{:?}", m.media_type).to_lowercase())?;
                if let Some(alt) = &m.alt_text {
                    dict.set_item("alt_text", alt)?;
                }
            }
            Content::Json { value, .. } => {
                let json_str = serde_json::to_string(value).unwrap_or_default();
                let json_module = py.import_bound("json")?;
                let obj = json_module.call_method1("loads", (json_str,))?;
                dict.set_item("value", obj)?;
            }
            Content::Binary { mime_type, data, .. } => {
                dict.set_item("mime_type", mime_type)?;
                dict.set_item("size", data.len())?;
            }
            Content::Composite { children, .. } => {
                let ids: Vec<String> = children.iter().map(|id| id.to_string()).collect();
                dict.set_item("children", ids)?;
            }
        }

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            Content::Text(t) => {
                let preview = if t.text.len() > 50 {
                    format!("{}...", &t.text[..50])
                } else {
                    t.text.clone()
                };
                format!("Content.text({:?})", preview)
            }
            Content::Code(c) => format!("Content.code({:?}, ...)", c.language),
            Content::Table(t) => format!(
                "Content.table(columns={}, rows={})",
                t.columns.len(),
                t.rows.len()
            ),
            _ => format!("Content(type={:?})", self.type_tag()),
        }
    }
}
