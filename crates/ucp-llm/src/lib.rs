//! # UCP LLM
//!
//! LLM-focused utilities for the Unified Content Protocol.
//!
//! This crate provides helpers for turning UCM documents into LLM-friendly
//! prompts and UCL command scaffolds, with a focus on token efficiency,
//! deterministic mappings, and safe prompt composition.
//!
//! ## Key Types
//!
//! - [`ContextManager`] - Context window management with expansion and pruning
//! - [`IdMapper`] - Token-efficient ID mapping (shortens block IDs for LLMs)
//! - [`PromptBuilder`] - Dynamic prompt generation with capability scoping
//!
//! ## Example
//!
//! ```rust
//! use ucp_llm::IdMapper;
//! use ucm_core::BlockId;
//!
//! let mut mapper = IdMapper::new();
//! let block_id = BlockId::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
//! let short_id = mapper.register(block_id);
//! println!("Short ID: {} (saves ~18 tokens)", short_id);
//! ```

pub mod context;
pub mod id_mapper;
pub mod prompt_builder;

pub use context::{
    CompressionMethod, ContextConstraints, ContextManager, ContextStatistics, ContextUpdateResult,
    ContextWindow, ExpandDirection, ExpansionPolicy, InclusionReason, PruningPolicy,
};
pub use id_mapper::IdMapper;
pub use prompt_builder::{presets, PromptBuilder, UclCapability};
