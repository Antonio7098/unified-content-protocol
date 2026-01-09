//! # UCM Core
//!
//! Core types and traits for the Unified Content Model (UCM).
//!
//! This crate provides the fundamental building blocks for representing
//! structured content in a graph-based intermediate representation.
//!
//! ## Key Types
//!
//! - [`Block`] - The fundamental unit of content
//! - [`BlockId`] - Content-addressed identifier with 96-bit collision resistance
//! - [`Content`] - Typed content (text, table, code, etc.)
//! - [`Document`] - A collection of blocks with hierarchical structure
//! - [`Edge`] - Explicit relationships between blocks
//!
//! ## Example
//!
//! ```rust
//! use ucm_core::Content;
//! use ucm_core::id::generate_block_id;
//!
//! let content = Content::text("Hello, UCP!");
//! let id = generate_block_id(&content, Some("intro.hook"), None);
//! println!("Block ID: {}", id);
//! ```

pub mod block;
pub mod content;
pub mod document;
pub mod edge;
pub mod error;
pub mod id;
pub mod metadata;
pub mod normalize;
pub mod version;

pub use block::{Block, BlockState};
pub use content::{
    BinaryEncoding, Cell, Code, Column, CompositeLayout, Content, DataType, Dimensions, JsonSchema,
    LineRange, Math, MathFormat, Media, MediaSource, MediaType, Row, Table, TableSchema, Text,
    TextFormat,
};
pub use document::{Document, DocumentId, DocumentMetadata};
pub use edge::{Edge, EdgeIndex, EdgeMetadata, EdgeType};
pub use error::{Error, ErrorCode, Result, ValidationIssue, ValidationSeverity};
pub use id::{BlockId, ContentHash, IdGenerator, IdGeneratorConfig};
pub use metadata::{BlockMetadata, RoleCategory, SemanticRole, TokenEstimate, TokenModel};
pub use version::{DocumentVersion, Version};
