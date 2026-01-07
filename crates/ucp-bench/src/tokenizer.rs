//! Token counting utilities.
//!
//! We default to the `cl100k_base` encoding used by GPT-4/4o style models.
//! For other model families (Llama, Qwen, etc.) we fall back to a simple
//! heuristic (4 characters ≈ 1 token) which keeps cost estimates consistent
//! even when the upstream API does not report usage statistics.

use once_cell::sync::Lazy;
use tiktoken_rs::{cl100k_base, CoreBPE};

static GPT_ENCODER: Lazy<CoreBPE> = Lazy::new(|| cl100k_base().expect("load cl100k_base"));

/// Count tokens for a given text and model hint.
pub fn count_tokens(model_hint: &str, text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    if use_gpt_like_encoding(model_hint) {
        // encode_with_special_tokens returns Vec<usize> directly
        return GPT_ENCODER.encode_with_special_tokens(text).len();
    }

    heuristic_tokens(text)
}

fn use_gpt_like_encoding(model_hint: &str) -> bool {
    let hint = model_hint.to_lowercase();
    hint.contains("gpt")
        || hint.contains("claude")
        || hint.contains("sonnet")
        || hint.contains("opus")
}

fn heuristic_tokens(text: &str) -> usize {
    // A widely used approximation is 4 characters per token for English text.
    // Ensure at least 1 token for non-empty text.
    ((text.chars().count().max(4)) / 4).max(1)
}
