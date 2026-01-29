//! Import commands

use anyhow::Result;

use crate::cli::{ImportCommands, OutputFormat};
use crate::output::{print_success, read_file, write_document};

pub fn handle(cmd: ImportCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        ImportCommands::Markdown { file, output } => markdown(file, output, format),
        ImportCommands::Html {
            file,
            output,
            extract_images,
            extract_links,
        } => html(file, output, extract_images, extract_links, format),
    }
}

fn markdown(file: String, output: Option<String>, format: OutputFormat) -> Result<()> {
    let content = read_file(&file)?;
    let doc = ucp_translator_markdown::parse_markdown(&content)?;

    match format {
        OutputFormat::Json => {
            write_document(&doc, output)?;
        }
        OutputFormat::Text => {
            print_success(&format!("Imported {} ({} blocks)", file, doc.block_count()));
            write_document(&doc, output)?;
        }
    }

    Ok(())
}

fn html(
    file: String,
    output: Option<String>,
    extract_images: bool,
    extract_links: bool,
    format: OutputFormat,
) -> Result<()> {
    let content = read_file(&file)?;

    let config = ucp_translator_html::HtmlParserConfig {
        extract_images,
        extract_links,
        ..Default::default()
    };

    let parser = ucp_translator_html::HtmlParser::with_config(config);
    let doc = parser.parse(&content)?;

    match format {
        OutputFormat::Json => {
            write_document(&doc, output)?;
        }
        OutputFormat::Text => {
            print_success(&format!("Imported {} ({} blocks)", file, doc.block_count()));
            write_document(&doc, output)?;
        }
    }

    Ok(())
}
