//! Validation commands

use anyhow::Result;
use ucm_engine::ValidationPipeline;

use crate::cli::OutputFormat;
use crate::output::{print_validation_result, read_document};

/// Validate a document
pub fn validate(
    input: Option<String>,
    max_blocks: Option<usize>,
    max_depth: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let mut pipeline = ValidationPipeline::new();

    if let Some(max) = max_blocks {
        pipeline = pipeline.with_max_blocks(max);
    }

    if let Some(max) = max_depth {
        pipeline = pipeline.with_max_depth(max);
    }

    let result = pipeline.validate(&doc);

    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_validation_result(&result);
        }
    }

    if !result.is_valid {
        std::process::exit(1);
    }

    Ok(())
}
