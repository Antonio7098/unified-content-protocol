//! Content type wrapper for Python.

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use ucm_core::content::{
    BinaryEncoding, CompositeLayout, Math, MathFormat, Media, MediaSource, MediaType,
};
use ucm_core::{BlockId, Content};

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
        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e)))?;
        Ok(PyContent(Content::json(json_value)))
    }

    /// Create table content from rows.
    #[staticmethod]
    fn table(rows: Vec<Vec<String>>) -> Self {
        PyContent(Content::table(rows))
    }

    /// Create math content (LaTeX by default).
    #[staticmethod]
    #[pyo3(signature = (expression, display_mode=false, format="latex"))]
    fn math(expression: &str, display_mode: bool, format: &str) -> PyResult<Self> {
        let math_format = match format.to_lowercase().as_str() {
            "latex" => MathFormat::LaTeX,
            "mathml" => MathFormat::MathML,
            "asciimath" => MathFormat::AsciiMath,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown math format: {}. Use 'latex', 'mathml', or 'asciimath'",
                    format
                )))
            }
        };
        Ok(PyContent(Content::Math(Math {
            format: math_format,
            expression: expression.to_string(),
            display_mode,
        })))
    }

    /// Create media content (image, audio, video, document).
    #[staticmethod]
    #[pyo3(signature = (media_type, url, alt_text=None, width=None, height=None))]
    fn media(
        media_type: &str,
        url: &str,
        alt_text: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> PyResult<Self> {
        let mt = match media_type.to_lowercase().as_str() {
            "image" => MediaType::Image,
            "audio" => MediaType::Audio,
            "video" => MediaType::Video,
            "document" => MediaType::Document,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown media type: {}. Use 'image', 'audio', 'video', or 'document'",
                    media_type
                )))
            }
        };
        let mut media = Media::image(MediaSource::url(url));
        media.media_type = mt;
        if let Some(alt) = alt_text {
            media = media.with_alt(alt);
        }
        if let (Some(w), Some(h)) = (width, height) {
            media = media.with_dimensions(w, h);
        }
        Ok(PyContent(Content::Media(media)))
    }

    /// Create binary content.
    #[staticmethod]
    #[pyo3(signature = (mime_type, data, encoding="raw"))]
    fn binary(mime_type: &str, data: &Bound<'_, PyBytes>, encoding: &str) -> PyResult<Self> {
        let enc = match encoding.to_lowercase().as_str() {
            "raw" => BinaryEncoding::Raw,
            "base64" => BinaryEncoding::Base64,
            "hex" => BinaryEncoding::Hex,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown encoding: {}. Use 'raw', 'base64', or 'hex'",
                    encoding
                )))
            }
        };
        Ok(PyContent(Content::Binary {
            mime_type: mime_type.to_string(),
            data: data.as_bytes().to_vec(),
            encoding: enc,
        }))
    }

    /// Create composite content (container for other blocks).
    #[staticmethod]
    #[pyo3(signature = (layout="vertical", children=None))]
    fn composite(layout: &str, children: Option<Vec<String>>) -> PyResult<Self> {
        let composite_layout = match layout.to_lowercase().as_str() {
            "vertical" => CompositeLayout::Vertical,
            "horizontal" => CompositeLayout::Horizontal,
            "tabs" => CompositeLayout::Tabs,
            s if s.starts_with("grid") => {
                // Parse "grid:N" or "grid(N)" format
                let cols = s
                    .trim_start_matches("grid")
                    .trim_start_matches(':')
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .parse::<usize>()
                    .unwrap_or(2);
                CompositeLayout::Grid(cols)
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown layout: {}. Use 'vertical', 'horizontal', 'tabs', or 'grid:N'",
                    layout
                )))
            }
        };
        let child_ids: Vec<BlockId> = children
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(PyContent(Content::Composite {
            layout: composite_layout,
            children: child_ids,
        }))
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
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "JSON serialization error: {}",
                        e
                    ))
                })?;
                let json_module = py.import_bound("json")?;
                let obj = json_module.call_method1("loads", (json_str,))?;
                Ok(Some(obj.into()))
            }
            _ => Ok(None),
        }
    }

    /// Get the math expression if this is a math block.
    fn as_math(&self) -> Option<(String, bool, String)> {
        match &self.0 {
            Content::Math(m) => {
                let format = match m.format {
                    MathFormat::LaTeX => "latex",
                    MathFormat::MathML => "mathml",
                    MathFormat::AsciiMath => "asciimath",
                };
                Some((m.expression.clone(), m.display_mode, format.to_string()))
            }
            _ => None,
        }
    }

    /// Get the media info if this is a media block.
    fn as_media(&self) -> Option<(String, String, Option<String>)> {
        match &self.0 {
            Content::Media(m) => {
                let media_type = match m.media_type {
                    MediaType::Image => "image",
                    MediaType::Audio => "audio",
                    MediaType::Video => "video",
                    MediaType::Document => "document",
                };
                let url = match &m.source {
                    MediaSource::Url(u) => u.clone(),
                    MediaSource::Base64(b) => format!("data:base64,{}", b),
                    MediaSource::Reference(id) => format!("ref:{}", id),
                    MediaSource::External(e) => format!("{}://{}/{}", e.provider, e.bucket, e.key),
                };
                Some((media_type.to_string(), url, m.alt_text.clone()))
            }
            _ => None,
        }
    }

    /// Get the binary data if this is a binary block.
    fn as_binary<'py>(&self, py: Python<'py>) -> Option<(String, Bound<'py, PyBytes>)> {
        match &self.0 {
            Content::Binary {
                mime_type, data, ..
            } => Some((mime_type.clone(), PyBytes::new_bound(py, data))),
            _ => None,
        }
    }

    /// Get the table data if this is a table block.
    fn as_table(&self) -> Option<(Vec<String>, Vec<Vec<String>>)> {
        match &self.0 {
            Content::Table(t) => {
                let columns: Vec<String> = t.columns.iter().map(|c| c.name.clone()).collect();
                let rows: Vec<Vec<String>> = t
                    .rows
                    .iter()
                    .map(|r| {
                        r.cells
                            .iter()
                            .map(|c| match c {
                                ucm_core::content::Cell::Null => "null".to_string(),
                                ucm_core::content::Cell::Text(s) => s.clone(),
                                ucm_core::content::Cell::Number(n) => n.to_string(),
                                ucm_core::content::Cell::Boolean(b) => b.to_string(),
                                ucm_core::content::Cell::Date(d) => d.clone(),
                                ucm_core::content::Cell::DateTime(dt) => dt.clone(),
                                ucm_core::content::Cell::Json(v) => v.to_string(),
                            })
                            .collect()
                    })
                    .collect();
                Some((columns, rows))
            }
            _ => None,
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
            Content::Binary {
                mime_type, data, ..
            } => {
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
