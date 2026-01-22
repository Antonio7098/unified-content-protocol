//! Core type wrappers for WASM.

use wasm_bindgen::prelude::*;
use ucm_core::content::{BinaryEncoding, CompositeLayout, Media, MediaSource, MediaType, Math, MathFormat};

/// Content type enumeration.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ContentType {
    Text = 0,
    Code = 1,
    Table = 2,
    Math = 3,
    Media = 4,
    Json = 5,
    Binary = 6,
    Composite = 7,
}

impl From<&ucm_core::Content> for ContentType {
    fn from(c: &ucm_core::Content) -> Self {
        match c {
            ucm_core::Content::Text(_) => ContentType::Text,
            ucm_core::Content::Code(_) => ContentType::Code,
            ucm_core::Content::Table(_) => ContentType::Table,
            ucm_core::Content::Math(_) => ContentType::Math,
            ucm_core::Content::Media(_) => ContentType::Media,
            ucm_core::Content::Json { .. } => ContentType::Json,
            ucm_core::Content::Binary { .. } => ContentType::Binary,
            ucm_core::Content::Composite { .. } => ContentType::Composite,
        }
    }
}

/// Edge type enumeration.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum EdgeType {
    DerivedFrom = 0,
    Supersedes = 1,
    TransformedFrom = 2,
    References = 3,
    CitedBy = 4,
    LinksTo = 5,
    Supports = 6,
    Contradicts = 7,
    Elaborates = 8,
    Summarizes = 9,
    ParentOf = 10,
    ChildOf = 11,
    SiblingOf = 12,
    PreviousSibling = 13,
    NextSibling = 14,
    VersionOf = 15,
    AlternativeOf = 16,
    TranslationOf = 17,
}

impl From<&ucm_core::EdgeType> for EdgeType {
    fn from(et: &ucm_core::EdgeType) -> Self {
        match et {
            ucm_core::EdgeType::DerivedFrom => EdgeType::DerivedFrom,
            ucm_core::EdgeType::Supersedes => EdgeType::Supersedes,
            ucm_core::EdgeType::TransformedFrom => EdgeType::TransformedFrom,
            ucm_core::EdgeType::References => EdgeType::References,
            ucm_core::EdgeType::CitedBy => EdgeType::CitedBy,
            ucm_core::EdgeType::LinksTo => EdgeType::LinksTo,
            ucm_core::EdgeType::Supports => EdgeType::Supports,
            ucm_core::EdgeType::Contradicts => EdgeType::Contradicts,
            ucm_core::EdgeType::Elaborates => EdgeType::Elaborates,
            ucm_core::EdgeType::Summarizes => EdgeType::Summarizes,
            ucm_core::EdgeType::ParentOf => EdgeType::ParentOf,
            ucm_core::EdgeType::ChildOf => EdgeType::ChildOf,
            ucm_core::EdgeType::SiblingOf => EdgeType::SiblingOf,
            ucm_core::EdgeType::PreviousSibling => EdgeType::PreviousSibling,
            ucm_core::EdgeType::NextSibling => EdgeType::NextSibling,
            ucm_core::EdgeType::VersionOf => EdgeType::VersionOf,
            ucm_core::EdgeType::AlternativeOf => EdgeType::AlternativeOf,
            ucm_core::EdgeType::TranslationOf => EdgeType::TranslationOf,
            ucm_core::EdgeType::Custom(_) => EdgeType::References,
        }
    }
}

impl From<EdgeType> for ucm_core::EdgeType {
    fn from(et: EdgeType) -> Self {
        match et {
            EdgeType::DerivedFrom => ucm_core::EdgeType::DerivedFrom,
            EdgeType::Supersedes => ucm_core::EdgeType::Supersedes,
            EdgeType::TransformedFrom => ucm_core::EdgeType::TransformedFrom,
            EdgeType::References => ucm_core::EdgeType::References,
            EdgeType::CitedBy => ucm_core::EdgeType::CitedBy,
            EdgeType::LinksTo => ucm_core::EdgeType::LinksTo,
            EdgeType::Supports => ucm_core::EdgeType::Supports,
            EdgeType::Contradicts => ucm_core::EdgeType::Contradicts,
            EdgeType::Elaborates => ucm_core::EdgeType::Elaborates,
            EdgeType::Summarizes => ucm_core::EdgeType::Summarizes,
            EdgeType::ParentOf => ucm_core::EdgeType::ParentOf,
            EdgeType::ChildOf => ucm_core::EdgeType::ChildOf,
            EdgeType::SiblingOf => ucm_core::EdgeType::SiblingOf,
            EdgeType::PreviousSibling => ucm_core::EdgeType::PreviousSibling,
            EdgeType::NextSibling => ucm_core::EdgeType::NextSibling,
            EdgeType::VersionOf => ucm_core::EdgeType::VersionOf,
            EdgeType::AlternativeOf => ucm_core::EdgeType::AlternativeOf,
            EdgeType::TranslationOf => ucm_core::EdgeType::TranslationOf,
        }
    }
}

/// Content wrapper for WASM.
#[wasm_bindgen]
pub struct Content {
    inner: ucm_core::Content,
}

#[wasm_bindgen]
impl Content {
    /// Create plain text content.
    #[wasm_bindgen(js_name = text)]
    pub fn text(text: &str) -> Content {
        Content {
            inner: ucm_core::Content::text(text),
        }
    }

    /// Create markdown text content.
    #[wasm_bindgen(js_name = markdown)]
    pub fn markdown(text: &str) -> Content {
        Content {
            inner: ucm_core::Content::markdown(text),
        }
    }

    /// Create code content.
    #[wasm_bindgen(js_name = code)]
    pub fn code(language: &str, source: &str) -> Content {
        Content {
            inner: ucm_core::Content::code(language, source),
        }
    }

    /// Create JSON content.
    #[wasm_bindgen(js_name = json)]
    pub fn json(value: JsValue) -> Result<Content, JsValue> {
        let json_value: serde_json::Value = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;
        Ok(Content {
            inner: ucm_core::Content::json(json_value),
        })
    }

    /// Create table content from rows.
    #[wasm_bindgen(js_name = table)]
    pub fn table(rows: JsValue) -> Result<Content, JsValue> {
        let rows_vec: Vec<Vec<String>> = serde_wasm_bindgen::from_value(rows)
            .map_err(|e| JsValue::from_str(&format!("Invalid table data: {}", e)))?;
        Ok(Content {
            inner: ucm_core::Content::table(rows_vec),
        })
    }

    /// Create math content (LaTeX by default).
    #[wasm_bindgen(js_name = math)]
    pub fn math(expression: &str, display_mode: Option<bool>, format: Option<String>) -> Result<Content, JsValue> {
        let math_format = match format.as_deref().unwrap_or("latex").to_lowercase().as_str() {
            "latex" => MathFormat::LaTeX,
            "mathml" => MathFormat::MathML,
            "asciimath" => MathFormat::AsciiMath,
            f => return Err(JsValue::from_str(&format!("Unknown math format: {}. Use 'latex', 'mathml', or 'asciimath'", f))),
        };
        Ok(Content {
            inner: ucm_core::Content::Math(Math {
                format: math_format,
                expression: expression.to_string(),
                display_mode: display_mode.unwrap_or(false),
            }),
        })
    }

    /// Create media content (image, audio, video, document).
    #[wasm_bindgen(js_name = media)]
    pub fn media(
        media_type: &str,
        url: &str,
        alt_text: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Content, JsValue> {
        let mt = match media_type.to_lowercase().as_str() {
            "image" => MediaType::Image,
            "audio" => MediaType::Audio,
            "video" => MediaType::Video,
            "document" => MediaType::Document,
            t => return Err(JsValue::from_str(&format!("Unknown media type: {}. Use 'image', 'audio', 'video', or 'document'", t))),
        };
        let mut media = Media::image(MediaSource::url(url));
        media.media_type = mt;
        if let Some(alt) = alt_text {
            media = media.with_alt(alt);
        }
        if let (Some(w), Some(h)) = (width, height) {
            media = media.with_dimensions(w, h);
        }
        Ok(Content {
            inner: ucm_core::Content::Media(media),
        })
    }

    /// Create binary content.
    #[wasm_bindgen(js_name = binary)]
    pub fn binary(mime_type: &str, data: &[u8], encoding: Option<String>) -> Result<Content, JsValue> {
        let enc = match encoding.as_deref().unwrap_or("raw").to_lowercase().as_str() {
            "raw" => BinaryEncoding::Raw,
            "base64" => BinaryEncoding::Base64,
            "hex" => BinaryEncoding::Hex,
            e => return Err(JsValue::from_str(&format!("Unknown encoding: {}. Use 'raw', 'base64', or 'hex'", e))),
        };
        Ok(Content {
            inner: ucm_core::Content::Binary {
                mime_type: mime_type.to_string(),
                data: data.to_vec(),
                encoding: enc,
            },
        })
    }

    /// Create composite content (container for other blocks).
    #[wasm_bindgen(js_name = composite)]
    pub fn composite(layout: Option<String>, children: Option<Vec<String>>) -> Result<Content, JsValue> {
        let layout_str = layout.as_deref().unwrap_or("vertical");
        let composite_layout = match layout_str.to_lowercase().as_str() {
            "vertical" => CompositeLayout::Vertical,
            "horizontal" => CompositeLayout::Horizontal,
            "tabs" => CompositeLayout::Tabs,
            s if s.starts_with("grid") => {
                let cols = s.trim_start_matches("grid")
                    .trim_start_matches(':')
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .parse::<usize>()
                    .unwrap_or(2);
                CompositeLayout::Grid(cols)
            }
            l => return Err(JsValue::from_str(&format!("Unknown layout: {}. Use 'vertical', 'horizontal', 'tabs', or 'grid:N'", l))),
        };
        let child_ids: Vec<ucm_core::BlockId> = children
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(Content {
            inner: ucm_core::Content::Composite {
                layout: composite_layout,
                children: child_ids,
            },
        })
    }

    /// Get the content type.
    #[wasm_bindgen(getter, js_name = contentType)]
    pub fn content_type(&self) -> ContentType {
        ContentType::from(&self.inner)
    }

    /// Get the type tag string.
    #[wasm_bindgen(getter, js_name = typeTag)]
    pub fn type_tag(&self) -> String {
        self.inner.type_tag().to_string()
    }

    /// Check if empty.
    #[wasm_bindgen(getter, js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get size in bytes.
    #[wasm_bindgen(getter, js_name = sizeBytes)]
    pub fn size_bytes(&self) -> usize {
        self.inner.size_bytes()
    }

    /// Get text content if this is a text block.
    #[wasm_bindgen(js_name = asText)]
    pub fn as_text(&self) -> Option<String> {
        match &self.inner {
            ucm_core::Content::Text(t) => Some(t.text.clone()),
            _ => None,
        }
    }

    /// Get code content if this is a code block (returns object {language, source}).
    #[wasm_bindgen(js_name = asCode)]
    pub fn as_code(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Code(c) => {
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("language"), &JsValue::from_str(&c.language));
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("source"), &JsValue::from_str(&c.source));
                obj.into()
            }
            _ => JsValue::NULL,
        }
    }

    /// Get JSON content if this is a JSON block.
    #[wasm_bindgen(js_name = asJson)]
    pub fn as_json(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Json { value, .. } => {
                serde_wasm_bindgen::to_value(value).unwrap_or(JsValue::NULL)
            }
            _ => JsValue::NULL,
        }
    }

    /// Get math content if this is a math block (returns object {expression, displayMode, format}).
    #[wasm_bindgen(js_name = asMath)]
    pub fn as_math(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Math(m) => {
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("expression"), &JsValue::from_str(&m.expression));
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("displayMode"), &JsValue::from_bool(m.display_mode));
                let format = match m.format {
                    MathFormat::LaTeX => "latex",
                    MathFormat::MathML => "mathml",
                    MathFormat::AsciiMath => "asciimath",
                };
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("format"), &JsValue::from_str(format));
                obj.into()
            }
            _ => JsValue::NULL,
        }
    }

    /// Get media content if this is a media block (returns object {mediaType, url, altText}).
    #[wasm_bindgen(js_name = asMedia)]
    pub fn as_media(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Media(m) => {
                let obj = js_sys::Object::new();
                let media_type = match m.media_type {
                    MediaType::Image => "image",
                    MediaType::Audio => "audio",
                    MediaType::Video => "video",
                    MediaType::Document => "document",
                };
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("mediaType"), &JsValue::from_str(media_type));
                let url = match &m.source {
                    MediaSource::Url(u) => u.clone(),
                    MediaSource::Base64(b) => format!("data:base64,{}", b),
                    MediaSource::Reference(id) => format!("ref:{}", id),
                    MediaSource::External(e) => format!("{}://{}/{}", e.provider, e.bucket, e.key),
                };
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("url"), &JsValue::from_str(&url));
                if let Some(alt) = &m.alt_text {
                    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("altText"), &JsValue::from_str(alt));
                }
                obj.into()
            }
            _ => JsValue::NULL,
        }
    }

    /// Get binary content if this is a binary block (returns object {mimeType, data}).
    #[wasm_bindgen(js_name = asBinary)]
    pub fn as_binary(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Binary { mime_type, data, .. } => {
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("mimeType"), &JsValue::from_str(mime_type));
                let arr = js_sys::Uint8Array::from(data.as_slice());
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &arr);
                obj.into()
            }
            _ => JsValue::NULL,
        }
    }

    /// Get table content if this is a table block (returns object {columns, rows}).
    #[wasm_bindgen(js_name = asTable)]
    pub fn as_table(&self) -> JsValue {
        match &self.inner {
            ucm_core::Content::Table(t) => {
                let obj = js_sys::Object::new();
                let columns: Vec<String> = t.columns.iter().map(|c| c.name.clone()).collect();
                let rows: Vec<Vec<String>> = t.rows.iter().map(|r| {
                    r.cells.iter().map(|c| match c {
                        ucm_core::content::Cell::Null => "null".to_string(),
                        ucm_core::content::Cell::Text(s) => s.clone(),
                        ucm_core::content::Cell::Number(n) => n.to_string(),
                        ucm_core::content::Cell::Boolean(b) => b.to_string(),
                        ucm_core::content::Cell::Date(d) => d.clone(),
                        ucm_core::content::Cell::DateTime(dt) => dt.clone(),
                        ucm_core::content::Cell::Json(v) => v.to_string(),
                    }).collect()
                }).collect();
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("columns"), &serde_wasm_bindgen::to_value(&columns).unwrap_or(JsValue::NULL));
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("rows"), &serde_wasm_bindgen::to_value(&rows).unwrap_or(JsValue::NULL));
                obj.into()
            }
            _ => JsValue::NULL,
        }
    }
}

impl Content {
    pub fn inner(&self) -> &ucm_core::Content {
        &self.inner
    }

    pub fn into_inner(self) -> ucm_core::Content {
        self.inner
    }
}
