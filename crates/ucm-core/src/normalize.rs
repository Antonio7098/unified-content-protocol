//! Content normalization for deterministic hashing.
//!
//! Normalization ensures that semantically equivalent content produces
//! identical hashes, regardless of superficial differences like whitespace
//! or Unicode representation.

use crate::content::{Cell, Code, Column, Content, Math, Media, MediaSource, Row, Table, Text};
use unicode_normalization::UnicodeNormalization;

/// Normalization configuration
#[derive(Debug, Clone, Copy, Default)]
pub struct NormalizationConfig {
    /// Unicode normalization form
    pub unicode_form: UnicodeForm,
    /// Whitespace handling
    pub whitespace: WhitespaceNorm,
    /// Line ending normalization
    pub line_endings: LineEndingNorm,
}

/// Unicode normalization form (per TR15)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UnicodeForm {
    /// Canonical Decomposition, followed by Canonical Composition (default)
    #[default]
    NFC,
    /// Canonical Decomposition
    NFD,
    /// Compatibility Decomposition, followed by Canonical Composition
    NFKC,
    /// Compatibility Decomposition
    NFKD,
}

/// Whitespace normalization strategy
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WhitespaceNorm {
    /// Collapse runs of whitespace to single space (default for text)
    #[default]
    Collapse,
    /// Preserve whitespace exactly (for code)
    Preserve,
    /// Trim leading/trailing only
    Trim,
}

/// Line ending normalization
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LineEndingNorm {
    /// Unix-style \n (default)
    #[default]
    LF,
    /// Windows-style \r\n
    CRLF,
    /// Preserve original
    Preserve,
}

/// Normalize content for hashing.
///
/// This function produces a canonical string representation of any content
/// type that can be used for deterministic ID generation.
///
/// # Example
/// ```
/// use ucm_core::content::{Content, TextFormat};
/// use ucm_core::normalize::normalize_content;
///
/// let content = Content::Text(ucm_core::content::Text {
///     text: "  Hello   World  ".to_string(),
///     format: TextFormat::Plain,
/// });
///
/// let normalized = normalize_content(&content);
/// assert_eq!(normalized, "Hello World");
/// ```
pub fn normalize_content(content: &Content) -> String {
    match content {
        Content::Text(text) => normalize_text_content(text),
        Content::Code(code) => normalize_code_content(code),
        Content::Table(table) => normalize_table_content(table),
        Content::Math(math) => normalize_math_content(math),
        Content::Media(media) => normalize_media_content(media),
        Content::Json { value, .. } => canonical_json(value),
        Content::Binary {
            data, mime_type, ..
        } => {
            format!("{}:{}", mime_type, hex::encode(sha256_hash(data)))
        }
        Content::Composite { layout, children } => {
            let children_str: Vec<String> = children.iter().map(|id| id.to_string()).collect();
            format!("{:?}:[{}]", layout, children_str.join(","))
        }
    }
}

fn normalize_text_content(text: &Text) -> String {
    normalize_text(
        &text.text,
        NormalizationConfig {
            whitespace: WhitespaceNorm::Collapse,
            ..Default::default()
        },
    )
}

fn normalize_code_content(code: &Code) -> String {
    // Code preserves whitespace but normalizes line endings
    let config = NormalizationConfig {
        whitespace: WhitespaceNorm::Preserve,
        line_endings: LineEndingNorm::LF,
        ..Default::default()
    };
    format!(
        "{}:{}",
        code.language.to_lowercase(),
        normalize_text(&code.source, config)
    )
}

fn normalize_table_content(table: &Table) -> String {
    let columns: Vec<String> = table.columns.iter().map(normalize_column).collect();

    let rows: Vec<String> = table.rows.iter().map(normalize_row).collect();

    format!("columns:[{}],rows:[{}]", columns.join(","), rows.join(","))
}

fn normalize_column(column: &Column) -> String {
    let name = normalize_text(&column.name, NormalizationConfig::default());
    match &column.data_type {
        Some(dt) => format!("{}:{:?}", name, dt),
        None => name,
    }
}

fn normalize_row(row: &Row) -> String {
    let cells: Vec<String> = row.cells.iter().map(normalize_cell).collect();
    format!("[{}]", cells.join(","))
}

fn normalize_cell(cell: &Cell) -> String {
    match cell {
        Cell::Null => "null".to_string(),
        Cell::Text(s) => format!("\"{}\"", normalize_text(s, NormalizationConfig::default())),
        Cell::Number(n) => {
            // Normalize floating point representation
            if n.fract() == 0.0 {
                format!("{:.0}", n)
            } else {
                format!("{}", n)
            }
        }
        Cell::Boolean(b) => b.to_string(),
        Cell::Date(s) => format!("d:{}", s),
        Cell::DateTime(s) => format!("dt:{}", s),
        Cell::Json(v) => canonical_json(v),
    }
}

fn normalize_math_content(math: &Math) -> String {
    let normalized_expr = normalize_text(
        &math.expression,
        NormalizationConfig {
            whitespace: WhitespaceNorm::Collapse,
            ..Default::default()
        },
    );
    format!("{:?}:{}", math.format, normalized_expr)
}

fn normalize_media_content(media: &Media) -> String {
    let source = match &media.source {
        MediaSource::Url(url) => format!("url:{}", url),
        MediaSource::Base64(data) => format!("b64:{}", &data[..data.len().min(32)]),
        MediaSource::Reference(id) => format!("ref:{}", id),
        MediaSource::External(ext) => {
            format!("ext:{}:{}:{}", ext.provider, ext.bucket, ext.key)
        }
    };

    match &media.content_hash {
        Some(hash) => format!(
            "{:?}:{}:hash:{}",
            media.media_type,
            source,
            hex::encode(hash)
        ),
        None => format!("{:?}:{}", media.media_type, source),
    }
}

/// Normalize a text string according to configuration.
pub fn normalize_text(text: &str, config: NormalizationConfig) -> String {
    // Step 1: Unicode normalization
    let unicode_normalized = match config.unicode_form {
        UnicodeForm::NFC => text.nfc().collect::<String>(),
        UnicodeForm::NFD => text.nfd().collect::<String>(),
        UnicodeForm::NFKC => text.nfkc().collect::<String>(),
        UnicodeForm::NFKD => text.nfkd().collect::<String>(),
    };

    // Step 2: Line ending normalization
    let line_normalized = match config.line_endings {
        LineEndingNorm::LF => unicode_normalized.replace("\r\n", "\n").replace('\r', "\n"),
        LineEndingNorm::CRLF => unicode_normalized
            .replace("\r\n", "\n")
            .replace('\r', "\n")
            .replace('\n', "\r\n"),
        LineEndingNorm::Preserve => unicode_normalized,
    };

    // Step 3: Whitespace normalization
    match config.whitespace {
        WhitespaceNorm::Collapse => line_normalized
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
        WhitespaceNorm::Trim => line_normalized.trim().to_string(),
        WhitespaceNorm::Preserve => line_normalized,
    }
}

/// Canonical JSON serialization (RFC 8785).
///
/// - Object keys sorted lexicographically
/// - No whitespace
/// - Numbers in canonical form
pub fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut pairs: Vec<_> = map.iter().collect();
            pairs.sort_by(|a, b| a.0.cmp(b.0));
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", escape_json_string(k), canonical_json(v)))
                .collect();
            format!("{{{}}}", inner.join(","))
        }
        serde_json::Value::Array(arr) => {
            let inner: Vec<String> = arr.iter().map(canonical_json).collect();
            format!("[{}]", inner.join(","))
        }
        serde_json::Value::String(s) => format!("\"{}\"", escape_json_string(s)),
        serde_json::Value::Number(n) => {
            // Canonical number representation
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 && f.abs() < 1e15 {
                    format!("{:.0}", f)
                } else {
                    format!("{}", f)
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
    }
}

/// Escape a string for JSON output
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

/// Compute SHA256 hash of data
fn sha256_hash(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Check if a character is CJK (Chinese, Japanese, Korean)
pub fn is_cjk_character(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |   // CJK Unified Ideographs
        '\u{3400}'..='\u{4DBF}' |   // CJK Extension A
        '\u{F900}'..='\u{FAFF}' |   // CJK Compatibility Ideographs
        '\u{3040}'..='\u{309F}' |   // Hiragana
        '\u{30A0}'..='\u{30FF}' |   // Katakana
        '\u{AC00}'..='\u{D7AF}'     // Hangul Syllables
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::TextFormat;

    #[test]
    fn test_normalize_text_whitespace() {
        let result = normalize_text("  hello   world  ", NormalizationConfig::default());
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_text_preserve() {
        let config = NormalizationConfig {
            whitespace: WhitespaceNorm::Preserve,
            ..Default::default()
        };
        let result = normalize_text("  hello   world  ", config);
        assert_eq!(result, "  hello   world  ");
    }

    #[test]
    fn test_normalize_line_endings() {
        let config = NormalizationConfig {
            line_endings: LineEndingNorm::LF,
            whitespace: WhitespaceNorm::Preserve,
            ..Default::default()
        };
        let result = normalize_text("line1\r\nline2\rline3", config);
        assert_eq!(result, "line1\nline2\nline3");
    }

    #[test]
    fn test_canonical_json_sorted_keys() {
        let json = serde_json::json!({"b": 1, "a": 2});
        let canonical = canonical_json(&json);
        assert_eq!(canonical, "{\"a\":2,\"b\":1}");
    }

    #[test]
    fn test_canonical_json_nested() {
        let json = serde_json::json!({"outer": {"b": 1, "a": 2}});
        let canonical = canonical_json(&json);
        assert_eq!(canonical, "{\"outer\":{\"a\":2,\"b\":1}}");
    }

    #[test]
    fn test_normalize_content_text() {
        let content = Content::Text(Text {
            text: "  Hello   World  ".to_string(),
            format: TextFormat::Plain,
        });
        let normalized = normalize_content(&content);
        assert_eq!(normalized, "Hello World");
    }

    #[test]
    fn test_normalize_content_code() {
        let content = Content::Code(Code {
            language: "Rust".to_string(),
            source: "fn main() {\n    println!(\"hello\");\n}".to_string(),
            highlights: vec![],
        });
        let normalized = normalize_content(&content);
        assert!(normalized.starts_with("rust:"));
    }

    #[test]
    fn test_is_cjk() {
        assert!(is_cjk_character('中'));
        assert!(is_cjk_character('あ'));
        assert!(is_cjk_character('한'));
        assert!(!is_cjk_character('a'));
        assert!(!is_cjk_character('1'));
    }
}
