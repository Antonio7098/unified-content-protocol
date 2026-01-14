//! Block metadata for search, display, and LLM optimization.

use crate::content::Content;
use crate::id::ContentHash;
use crate::normalize::is_cjk_character;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

/// Block metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Semantic role in document structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_role: Option<SemanticRole>,

    /// Human-readable label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Searchable tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Pre-computed summary for folding/context management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Estimated token count (computed lazily)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_estimate: Option<TokenEstimate>,

    /// Content hash for change detection
    pub content_hash: ContentHash,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,

    /// Custom key-value metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, serde_json::Value>,
}

impl BlockMetadata {
    /// Create new metadata with current timestamp
    pub fn new(content_hash: ContentHash) -> Self {
        let now = Utc::now();
        Self {
            semantic_role: None,
            label: None,
            tags: Vec::new(),
            summary: None,
            token_estimate: None,
            content_hash,
            created_at: now,
            modified_at: now,
            custom: HashMap::new(),
        }
    }

    /// Set semantic role
    pub fn with_role(mut self, role: SemanticRole) -> Self {
        self.semantic_role = Some(role);
        self
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    /// Set summary
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Set custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }

    /// Update modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }

    /// Check if block has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

impl Default for BlockMetadata {
    fn default() -> Self {
        Self::new(ContentHash::from_bytes([0u8; 32]))
    }
}

/// Semantic role in document structure
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticRole {
    /// Primary category
    pub category: RoleCategory,
    /// Subcategory (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    /// Custom qualifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

impl SemanticRole {
    pub fn new(category: RoleCategory) -> Self {
        Self {
            category,
            subcategory: None,
            qualifier: None,
        }
    }

    pub fn with_subcategory(mut self, sub: impl Into<String>) -> Self {
        self.subcategory = Some(sub.into());
        self
    }

    pub fn with_qualifier(mut self, qual: impl Into<String>) -> Self {
        self.qualifier = Some(qual.into());
        self
    }

    /// Parse from string format (e.g., "intro.hook")
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let category = RoleCategory::from_str(parts[0]).ok()?;
        let subcategory = parts.get(1).map(|s| s.to_string());
        let qualifier = parts.get(2).map(|s| s.to_string());

        Some(Self {
            category,
            subcategory,
            qualifier,
        })
    }
}

impl std::fmt::Display for SemanticRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.category.as_str())?;
        if let Some(ref sub) = self.subcategory {
            write!(f, ".{}", sub)?;
        }
        if let Some(ref qual) = self.qualifier {
            write!(f, ".{}", qual)?;
        }
        Ok(())
    }
}

/// Semantic role categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleCategory {
    // Document structure
    Title,
    Subtitle,
    Abstract,
    TableOfContents,

    // Headings (H1-H6)
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,

    // Paragraphs and lists
    Paragraph,
    List,

    // Introduction
    Intro,
    IntroHook,
    IntroContext,
    IntroThesis,

    // Body
    Body,
    BodyArgument,
    BodyEvidence,
    BodyExample,
    BodyCounterargument,
    BodyTransition,

    // Conclusion
    Conclusion,
    ConclusionSummary,
    ConclusionImplication,
    ConclusionCallToAction,

    // Special sections
    Sidebar,
    Callout,
    Warning,
    Note,
    Quote,

    // Technical
    Definition,
    Theorem,
    Proof,
    Algorithm,
    Code,

    // Meta
    Metadata,
    Citation,
    Footnote,
    Appendix,
    Reference,

    // Custom
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleCategoryParseError(pub String);

impl fmt::Display for RoleCategoryParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown role category '{}'", self.0)
    }
}

impl StdError for RoleCategoryParseError {}

impl RoleCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Subtitle => "subtitle",
            Self::Abstract => "abstract",
            Self::TableOfContents => "toc",
            Self::Heading1 => "heading1",
            Self::Heading2 => "heading2",
            Self::Heading3 => "heading3",
            Self::Heading4 => "heading4",
            Self::Heading5 => "heading5",
            Self::Heading6 => "heading6",
            Self::Paragraph => "paragraph",
            Self::List => "list",
            Self::Intro => "intro",
            Self::IntroHook => "intro_hook",
            Self::IntroContext => "intro_context",
            Self::IntroThesis => "intro_thesis",
            Self::Body => "body",
            Self::BodyArgument => "body_argument",
            Self::BodyEvidence => "body_evidence",
            Self::BodyExample => "body_example",
            Self::BodyCounterargument => "body_counterargument",
            Self::BodyTransition => "body_transition",
            Self::Conclusion => "conclusion",
            Self::ConclusionSummary => "conclusion_summary",
            Self::ConclusionImplication => "conclusion_implication",
            Self::ConclusionCallToAction => "conclusion_cta",
            Self::Sidebar => "sidebar",
            Self::Callout => "callout",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Quote => "quote",
            Self::Definition => "definition",
            Self::Theorem => "theorem",
            Self::Proof => "proof",
            Self::Algorithm => "algorithm",
            Self::Code => "code",
            Self::Metadata => "metadata",
            Self::Citation => "citation",
            Self::Footnote => "footnote",
            Self::Appendix => "appendix",
            Self::Reference => "reference",
            Self::Custom => "custom",
        }
    }
}

impl FromStr for RoleCategory {
    type Err = RoleCategoryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "title" => Ok(Self::Title),
            "subtitle" => Ok(Self::Subtitle),
            "abstract" => Ok(Self::Abstract),
            "toc" | "table_of_contents" => Ok(Self::TableOfContents),
            "heading1" | "h1" => Ok(Self::Heading1),
            "heading2" | "h2" => Ok(Self::Heading2),
            "heading3" | "h3" => Ok(Self::Heading3),
            "heading4" | "h4" => Ok(Self::Heading4),
            "heading5" | "h5" => Ok(Self::Heading5),
            "heading6" | "h6" => Ok(Self::Heading6),
            "paragraph" | "para" | "p" => Ok(Self::Paragraph),
            "list" | "ul" | "ol" => Ok(Self::List),
            "intro" | "introduction" => Ok(Self::Intro),
            "intro_hook" | "hook" => Ok(Self::IntroHook),
            "intro_context" | "context" => Ok(Self::IntroContext),
            "intro_thesis" | "thesis" => Ok(Self::IntroThesis),
            "body" => Ok(Self::Body),
            "body_argument" | "argument" => Ok(Self::BodyArgument),
            "body_evidence" | "evidence" => Ok(Self::BodyEvidence),
            "body_example" | "example" => Ok(Self::BodyExample),
            "body_counterargument" | "counterargument" => Ok(Self::BodyCounterargument),
            "body_transition" | "transition" => Ok(Self::BodyTransition),
            "conclusion" => Ok(Self::Conclusion),
            "conclusion_summary" | "summary" => Ok(Self::ConclusionSummary),
            "conclusion_implication" | "implication" => Ok(Self::ConclusionImplication),
            "conclusion_cta" | "cta" | "call_to_action" => Ok(Self::ConclusionCallToAction),
            "sidebar" => Ok(Self::Sidebar),
            "callout" => Ok(Self::Callout),
            "warning" => Ok(Self::Warning),
            "note" => Ok(Self::Note),
            "quote" | "blockquote" => Ok(Self::Quote),
            "definition" => Ok(Self::Definition),
            "theorem" => Ok(Self::Theorem),
            "proof" => Ok(Self::Proof),
            "algorithm" => Ok(Self::Algorithm),
            "code" => Ok(Self::Code),
            "metadata" | "meta" => Ok(Self::Metadata),
            "citation" | "cite" => Ok(Self::Citation),
            "footnote" => Ok(Self::Footnote),
            "appendix" => Ok(Self::Appendix),
            "reference" | "ref" => Ok(Self::Reference),
            "custom" => Ok(Self::Custom),
            _ => Err(RoleCategoryParseError(s.to_string())),
        }
    }
}

/// Token estimation with model awareness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenEstimate {
    /// Estimated tokens for GPT-4 tokenizer
    pub gpt4: u32,
    /// Estimated tokens for Claude tokenizer
    pub claude: u32,
    /// Estimated tokens for Llama tokenizer
    pub llama: u32,
    /// Generic estimate (average)
    pub generic: u32,
}

impl TokenEstimate {
    /// Create a new token estimate with all models
    pub fn new(gpt4: u32, claude: u32, llama: u32) -> Self {
        let generic = (gpt4 + claude + llama) / 3;
        Self {
            gpt4,
            claude,
            llama,
            generic,
        }
    }

    /// Compute token estimate from content
    pub fn compute(content: &Content) -> Self {
        match content {
            Content::Text(text) => Self::estimate_text(&text.text),
            Content::Code(code) => Self::estimate_code(&code.source, &code.language),
            Content::Table(table) => Self::estimate_table(&table.columns, &table.rows),
            Content::Json { value, .. } => Self::estimate_json(value),
            Content::Math(math) => Self::estimate_text(&math.expression),
            _ => Self::default_estimate(),
        }
    }

    /// Get estimate for a specific model
    pub fn for_model(&self, model: TokenModel) -> u32 {
        match model {
            TokenModel::GPT4 => self.gpt4,
            TokenModel::Claude => self.claude,
            TokenModel::Llama => self.llama,
            TokenModel::Generic => self.generic,
        }
    }

    fn estimate_text(text: &str) -> Self {
        let char_count = text.chars().count();
        let word_count = text.split_whitespace().count();

        // Detect script type for better estimation
        let cjk_count = text.chars().filter(|c| is_cjk_character(*c)).count();
        let cjk_ratio = cjk_count as f32 / char_count.max(1) as f32;

        // CJK characters are ~1-2 tokens each, Latin ~4 chars per token
        let base_estimate = if cjk_ratio > 0.5 {
            (char_count as f32 * 1.5) as u32
        } else {
            (word_count as f32 * 1.3 + char_count as f32 / 4.0) as u32 / 2
        };

        Self {
            gpt4: base_estimate,
            claude: (base_estimate as f32 * 1.1) as u32,
            llama: (base_estimate as f32 * 0.95) as u32,
            generic: base_estimate,
        }
    }

    fn estimate_code(source: &str, language: &str) -> Self {
        let line_count = source.lines().count();
        let char_count = source.len();

        // Code typically has more tokens due to punctuation
        let base = (char_count / 3 + line_count * 2) as u32;

        // Language-specific adjustments
        let factor = match language.to_lowercase().as_str() {
            "rust" | "cpp" | "c" | "c++" => 1.2,
            "python" => 0.9,
            "javascript" | "typescript" | "js" | "ts" => 1.1,
            "go" | "golang" => 1.0,
            "java" => 1.15,
            _ => 1.0,
        };

        let adjusted = (base as f32 * factor) as u32;

        Self {
            gpt4: adjusted,
            claude: (adjusted as f32 * 1.05) as u32,
            llama: (adjusted as f32 * 0.95) as u32,
            generic: adjusted,
        }
    }

    fn estimate_table(columns: &[crate::content::Column], rows: &[crate::content::Row]) -> Self {
        let cell_count = columns.len() * rows.len();
        let header_tokens = columns.len() * 5; // ~5 tokens per header
        let cell_tokens = cell_count * 3; // ~3 tokens per cell average
        let structure_tokens = rows.len() * 2; // Row separators

        let total = (header_tokens + cell_tokens + structure_tokens) as u32;

        Self {
            gpt4: total,
            claude: (total as f32 * 1.1) as u32,
            llama: total,
            generic: total,
        }
    }

    fn estimate_json(value: &serde_json::Value) -> Self {
        let serialized = serde_json::to_string(value).unwrap_or_default();
        Self::estimate_text(&serialized)
    }

    fn default_estimate() -> Self {
        Self {
            gpt4: 100,
            claude: 110,
            llama: 95,
            generic: 100,
        }
    }
}

impl Default for TokenEstimate {
    fn default() -> Self {
        Self::default_estimate()
    }
}

/// Token model selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenModel {
    GPT4,
    Claude,
    Llama,
    Generic,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_role_parse() {
        let role = SemanticRole::parse("intro.hook").unwrap();
        assert_eq!(role.category, RoleCategory::Intro);
        assert_eq!(role.subcategory, Some("hook".to_string()));
    }

    #[test]
    fn test_semantic_role_display() {
        let role = SemanticRole::new(RoleCategory::Intro)
            .with_subcategory("hook")
            .with_qualifier("v2");
        assert_eq!(role.to_string(), "intro.hook.v2");
    }

    #[test]
    fn test_role_category_roundtrip() {
        let category = RoleCategory::BodyEvidence;
        let s = category.as_str();
        let parsed = RoleCategory::from_str(s).unwrap();
        assert_eq!(parsed, category);
    }

    #[test]
    fn test_token_estimate_text() {
        let estimate = TokenEstimate::estimate_text("Hello, world! This is a test.");
        assert!(estimate.gpt4 > 0);
        assert!(estimate.claude > 0);
    }

    #[test]
    fn test_token_estimate_cjk() {
        let estimate = TokenEstimate::estimate_text("你好世界");
        // CJK should have higher token count per character
        assert!(estimate.gpt4 > 0);
    }

    #[test]
    fn test_metadata_builder() {
        let hash = ContentHash::from_bytes([1u8; 32]);
        let metadata = BlockMetadata::new(hash)
            .with_label("Test Block")
            .with_tags(["important", "draft"])
            .with_role(SemanticRole::new(RoleCategory::Intro));

        assert_eq!(metadata.label, Some("Test Block".to_string()));
        assert!(metadata.has_tag("important"));
        assert!(metadata.has_tag("draft"));
    }
}
