//! Core type wrappers for WASM.

use wasm_bindgen::prelude::*;

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
}

impl Content {
    pub fn inner(&self) -> &ucm_core::Content {
        &self.inner
    }

    pub fn into_inner(self) -> ucm_core::Content {
        self.inner
    }
}
