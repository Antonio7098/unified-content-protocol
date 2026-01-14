//! Block ID generation with 96-bit collision resistance.
//!
//! Block IDs are deterministic, content-addressed identifiers derived from:
//! - Content type
//! - Semantic role (optional)
//! - Normalized content
//! - Namespace (optional, for multi-tenant scenarios)
//!
//! Using 96 bits of entropy ensures collision probability < 10⁻¹⁵ at 10M blocks.

use crate::content::Content;
use crate::error::{Error, ErrorCode, Result};
use crate::normalize::normalize_content;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

/// Block identifier with 96 bits of entropy (12 bytes).
///
/// Format: `blk_<24 hex characters>`
///
/// # Example
/// ```
/// use ucm_core::BlockId;
///
/// let id = BlockId::from_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
///                               0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
/// assert!(id.to_string().starts_with("blk_"));
/// assert_eq!(id.to_string().len(), 28); // "blk_" + 24 hex chars
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId(#[serde(with = "hex_array")] pub [u8; 12]);

impl BlockId {
    /// Create a BlockId from raw bytes
    pub fn from_bytes(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }

    /// Generate a root block ID (all zeros with marker)
    pub fn root() -> Self {
        let mut bytes = [0u8; 12];
        bytes[0] = 0xFF; // Marker for root
        Self(bytes)
    }

    /// Check if this is a root block ID
    pub fn is_root(&self) -> bool {
        self.0[0] == 0xFF && self.0[1..].iter().all(|&b| b == 0)
    }

    /// Create a BlockId from hex string (12 hex chars = 6 bytes, padded to 12)
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s).map_err(|_| {
            Error::new(
                ErrorCode::E002InvalidBlockId,
                format!("Invalid hex string: {}", s),
            )
        })?;
        if bytes.len() > 12 {
            return Err(Error::new(
                ErrorCode::E002InvalidBlockId,
                "Hex string too long",
            ));
        }
        let mut arr = [0u8; 12];
        let start = 12 - bytes.len();
        arr[start..].copy_from_slice(&bytes);
        Ok(Self(arr))
    }
}

impl fmt::Debug for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockId({})", self)
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blk_{}", hex::encode(self.0))
    }
}

impl FromStr for BlockId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let hex_part = s
            .strip_prefix("blk_")
            .ok_or_else(|| Error::InvalidBlockId(format!("missing 'blk_' prefix: {}", s)))?;

        if hex_part.len() != 24 {
            return Err(Error::InvalidBlockId(format!(
                "expected 24 hex characters, got {}",
                hex_part.len()
            )));
        }

        let bytes = hex::decode(hex_part)
            .map_err(|e| Error::InvalidBlockId(format!("invalid hex: {}", e)))?;

        if bytes.len() != 12 {
            return Err(Error::InvalidBlockId(format!(
                "expected 12 bytes, got {}",
                bytes.len()
            )));
        }

        let mut arr = [0u8; 12];
        arr.copy_from_slice(&bytes);
        Ok(BlockId(arr))
    }
}

/// Content hash (full SHA256)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(#[serde(with = "hex_array_32")] pub [u8; 32]);

impl ContentHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContentHash({})", hex::encode(&self.0[..8]))
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

/// Configuration for ID generation
#[derive(Debug, Clone, Default)]
pub struct IdGeneratorConfig {
    /// Namespace for multi-tenant scenarios
    pub namespace: Option<String>,
    /// Whether to include semantic role in hash
    pub include_semantic_role: bool,
}

impl IdGeneratorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_semantic_role(mut self, include: bool) -> Self {
        self.include_semantic_role = include;
        self
    }
}

/// ID generator with configurable options
#[derive(Debug, Clone)]
pub struct IdGenerator {
    config: IdGeneratorConfig,
}

impl IdGenerator {
    pub fn new(config: IdGeneratorConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(IdGeneratorConfig::default())
    }

    /// Generate a block ID from content
    pub fn generate(&self, content: &Content, semantic_role: Option<&str>) -> BlockId {
        generate_block_id(content, semantic_role, self.config.namespace.as_deref())
    }

    /// Generate a content hash
    pub fn content_hash(&self, content: &Content) -> ContentHash {
        compute_content_hash(content)
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Generate a deterministic block ID from content.
///
/// The ID is derived from:
/// 1. Optional namespace (for multi-tenant isolation)
/// 2. Content type discriminant
/// 3. Optional semantic role
/// 4. Normalized content
///
/// # Arguments
/// * `content` - The block content
/// * `semantic_role` - Optional semantic role (e.g., "intro.hook")
/// * `namespace` - Optional namespace for multi-tenant scenarios
///
/// # Example
/// ```
/// use ucm_core::Content;
/// use ucm_core::id::generate_block_id;
///
/// let content = Content::text("Hello, world!");
///
/// let id1 = generate_block_id(&content, Some("intro"), None);
/// let id2 = generate_block_id(&content, Some("intro"), None);
/// assert_eq!(id1, id2); // Deterministic
///
/// let id3 = generate_block_id(&content, Some("conclusion"), None);
/// assert_ne!(id1, id3); // Different role = different ID
/// ```
pub fn generate_block_id(
    content: &Content,
    semantic_role: Option<&str>,
    namespace: Option<&str>,
) -> BlockId {
    let mut hasher = Sha256::new();

    // Add namespace if present
    if let Some(ns) = namespace {
        hasher.update(ns.as_bytes());
        hasher.update(b":");
    }

    // Add content type discriminant
    hasher.update(content.type_tag().as_bytes());
    hasher.update(b":");

    // Add semantic role
    if let Some(role) = semantic_role {
        hasher.update(role.as_bytes());
    }
    hasher.update(b":");

    // Add normalized content
    let normalized = normalize_content(content);
    hasher.update(normalized.as_bytes());

    // Extract 96 bits (12 bytes) from the 256-bit hash
    let hash = hasher.finalize();
    let mut id_bytes = [0u8; 12];
    id_bytes.copy_from_slice(&hash[..12]);

    BlockId(id_bytes)
}

/// Compute the full content hash (SHA256)
pub fn compute_content_hash(content: &Content) -> ContentHash {
    let mut hasher = Sha256::new();
    let normalized = normalize_content(content);
    hasher.update(normalized.as_bytes());
    let hash = hasher.finalize();
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&hash);
    ContentHash(hash_bytes)
}

// Serde helpers for hex encoding
mod hex_array {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 12], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 12], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 12 {
            return Err(serde::de::Error::custom("expected 12 bytes"));
        }
        let mut arr = [0u8; 12];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }
}

mod hex_array_32 {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("expected 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_id_display() {
        let id = BlockId::from_bytes([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
        ]);
        assert_eq!(id.to_string(), "blk_0102030405060708090a0b0c");
    }

    #[test]
    fn test_block_id_parse() {
        let id_str = "blk_0102030405060708090a0b0c";
        let id: BlockId = id_str.parse().unwrap();
        assert_eq!(id.to_string(), id_str);
    }

    #[test]
    fn test_block_id_parse_invalid() {
        assert!("invalid".parse::<BlockId>().is_err());
        assert!("blk_invalid".parse::<BlockId>().is_err());
        assert!("blk_0102".parse::<BlockId>().is_err()); // Too short
    }

    #[test]
    fn test_deterministic_id_generation() {
        let content = Content::text("Hello, world!");

        let id1 = generate_block_id(&content, Some("intro"), None);
        let id2 = generate_block_id(&content, Some("intro"), None);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_role_different_id() {
        let content = Content::text("Hello, world!");

        let id1 = generate_block_id(&content, Some("intro"), None);
        let id2 = generate_block_id(&content, Some("conclusion"), None);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_namespace_isolation() {
        let content = Content::text("Hello");

        let id1 = generate_block_id(&content, None, Some("tenant-a"));
        let id2 = generate_block_id(&content, None, Some("tenant-b"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_root_block_id() {
        let root = BlockId::root();
        assert!(root.is_root());

        let non_root = BlockId::from_bytes([0x01; 12]);
        assert!(!non_root.is_root());
    }

    #[test]
    fn test_content_hash() {
        let content = Content::text("Hello");

        let hash1 = compute_content_hash(&content);
        let hash2 = compute_content_hash(&content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_id_generator() {
        let gen = IdGenerator::new(IdGeneratorConfig::new().with_namespace("test"));
        let content = Content::text("Hello");

        let id = gen.generate(&content, Some("intro"));
        assert!(!id.is_root());
    }
}
