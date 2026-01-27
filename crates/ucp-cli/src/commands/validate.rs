//! Validation commands

use anyhow::Result;
use serde::Serialize;
use ucm_engine::validate::ResourceLimits;
use ucm_engine::ValidationPipeline;

use crate::cli::OutputFormat;
use crate::output::{print_validation_result, read_document};

/// Serializable version of ValidationResult
#[derive(Serialize)]
struct ValidationResultJson {
    valid: bool,
    issues: Vec<ValidationIssueJson>,
}

#[derive(Serialize)]
struct ValidationIssueJson {
    severity: String,
    code: String,
    message: String,
}

impl From<&ucm_engine::ValidationResult> for ValidationResultJson {
    fn from(result: &ucm_engine::ValidationResult) -> Self {
        Self {
            valid: result.valid,
            issues: result.issues.iter().map(|i| ValidationIssueJson {
                severity: format!("{:?}", i.severity),
                code: format!("{:?}", i.code),
                message: i.message.clone(),
            }).collect(),
        }
    }
}

/// Validate a document
pub fn validate(
    input: Option<String>,
    max_blocks: Option<usize>,
    max_depth: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let mut limits = ResourceLimits::default();

    if let Some(max) = max_blocks {
        limits.max_block_count = max;
    }

    if let Some(max) = max_depth {
        limits.max_nesting_depth = max;
    }

    let pipeline = ValidationPipeline::with_limits(limits);
    let result = pipeline.validate_document(&doc);

    match format {
        OutputFormat::Json => {
            let json_result = ValidationResultJson::from(&result);
            println!("{}", serde_json::to_string_pretty(&json_result)?);
        }
        OutputFormat::Text => {
            print_validation_result(&result);
        }
    }

    if !result.valid {
        std::process::exit(1);
    }

    Ok(())
}
