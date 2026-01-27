//! Document management commands

use anyhow::Result;
use serde::Serialize;
use ucm_core::{Document, TokenModel};

use crate::cli::OutputFormat;
use crate::output::{print_document_info, print_success, write_document, read_document, DocumentJson};

/// Create a new document
pub fn create(output: Option<String>, title: Option<String>, format: OutputFormat) -> Result<()> {
    let mut doc = Document::create();

    if let Some(t) = title {
        doc.metadata.title = Some(t);
    }

    match format {
        OutputFormat::Json => {
            write_document(&doc, output)?;
        }
        OutputFormat::Text => {
            if output.is_some() {
                write_document(&doc, output)?;
            } else {
                // In text mode without output, show the document info
                print_success("Created new document");
                print_document_info(&doc);
                let doc_json = DocumentJson::from_document(&doc);
                println!("\n{}", serde_json::to_string_pretty(&doc_json)?);
            }
        }
    }

    Ok(())
}

/// Display document information
pub fn info(input: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    #[derive(Serialize)]
    struct DocumentInfo {
        id: String,
        root: String,
        block_count: usize,
        total_tokens: Option<u32>,
        version: u64,
        title: Option<String>,
        description: Option<String>,
        edge_count: usize,
    }

    let info = DocumentInfo {
        id: doc.id.to_string(),
        root: doc.root.to_string(),
        block_count: doc.block_count(),
        total_tokens: Some(doc.total_tokens(TokenModel::Generic)),
        version: doc.version.counter,
        title: doc.metadata.title.clone(),
        description: doc.metadata.description.clone(),
        edge_count: doc.edge_index.edge_count(),
    };

    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        OutputFormat::Text => {
            print_document_info(&doc);
        }
    }

    Ok(())
}
