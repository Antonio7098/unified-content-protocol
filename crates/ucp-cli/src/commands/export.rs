//! Export commands

use anyhow::Result;

use crate::cli::{ExportCommands, OutputFormat};
use crate::output::{read_document, write_output, DocumentJson};

pub fn handle(cmd: ExportCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        ExportCommands::Markdown { input, output } => markdown(input, output, format),
        ExportCommands::Json {
            input,
            output,
            pretty,
        } => json(input, output, pretty, format),
    }
}

fn markdown(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;
    let md = ucp_translator_markdown::render_markdown(&doc)?;

    match format {
        OutputFormat::Json => {
            // In JSON mode, return the markdown as a JSON string
            println!("{}", serde_json::to_string(&md)?);
        }
        OutputFormat::Text => {
            if output.is_some() {
                write_output(&md, output)?;
            } else {
                println!("{}", md);
            }
        }
    }

    Ok(())
}

fn json(
    input: Option<String>,
    output: Option<String>,
    pretty: bool,
    _format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;
    let doc_json = DocumentJson::from_document(&doc);

    let json_str = if pretty {
        serde_json::to_string_pretty(&doc_json)?
    } else {
        serde_json::to_string(&doc_json)?
    };

    write_output(&json_str, output)?;
    Ok(())
}
