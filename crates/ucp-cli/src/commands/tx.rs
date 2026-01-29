//! Transaction commands

use anyhow::Result;
use serde::Serialize;

use crate::cli::{OutputFormat, TxCommands};
use crate::output::{print_error, print_success, print_warning};
use crate::state::{read_stateful_document, write_stateful_document, TransactionState};

pub fn handle(cmd: TxCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        TxCommands::Begin {
            input,
            output,
            name,
        } => begin(input, output, name, format),
        TxCommands::Commit { input, output } => commit(input, output, format),
        TxCommands::Rollback { input, output } => rollback(input, output, format),
        TxCommands::Savepoint {
            input,
            output,
            name,
        } => savepoint(input, output, name, format),
    }
}

fn begin(
    input: Option<String>,
    output: Option<String>,
    name: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    // Check if already in a transaction
    if stateful.state().transaction.is_some() {
        match format {
            OutputFormat::Json => {
                #[derive(Serialize)]
                struct TxResult {
                    success: bool,
                    error: String,
                }
                let result = TxResult {
                    success: false,
                    error: "Already in a transaction".to_string(),
                };
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Text => {
                print_error("Already in a transaction. Commit or rollback first.");
            }
        }
        return Ok(());
    }

    // Start transaction
    let tx = TransactionState::new(name.clone(), &stateful.document)?;
    stateful.state_mut().transaction = Some(tx);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct TxResult {
                success: bool,
                transaction_name: Option<String>,
            }
            let result = TxResult {
                success: true,
                transaction_name: name,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if let Some(n) = name {
                print_success(&format!("Transaction '{}' started", n));
            } else {
                print_success("Transaction started");
            }
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn commit(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    let tx = match stateful.state_mut().transaction.take() {
        Some(t) => t,
        None => {
            match format {
                OutputFormat::Json => {
                    #[derive(Serialize)]
                    struct TxResult {
                        success: bool,
                        error: String,
                    }
                    let result = TxResult {
                        success: false,
                        error: "No active transaction".to_string(),
                    };
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                OutputFormat::Text => {
                    print_warning("No active transaction to commit");
                }
            }
            return Ok(());
        }
    };

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct TxResult {
                success: bool,
                transaction_name: Option<String>,
                savepoints_count: usize,
            }
            let result = TxResult {
                success: true,
                transaction_name: tx.name,
                savepoints_count: tx.savepoints.len(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if let Some(n) = tx.name {
                print_success(&format!("Transaction '{}' committed", n));
            } else {
                print_success("Transaction committed");
            }
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn rollback(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    let tx = match stateful.state_mut().transaction.take() {
        Some(t) => t,
        None => {
            match format {
                OutputFormat::Json => {
                    #[derive(Serialize)]
                    struct TxResult {
                        success: bool,
                        error: String,
                    }
                    let result = TxResult {
                        success: false,
                        error: "No active transaction".to_string(),
                    };
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                OutputFormat::Text => {
                    print_warning("No active transaction to rollback");
                }
            }
            return Ok(());
        }
    };

    // Restore original document
    stateful.document = tx.get_original_document()?;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct TxResult {
                success: bool,
                transaction_name: Option<String>,
            }
            let result = TxResult {
                success: true,
                transaction_name: tx.name,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if let Some(n) = tx.name {
                print_success(&format!("Transaction '{}' rolled back", n));
            } else {
                print_success("Transaction rolled back");
            }
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn savepoint(
    input: Option<String>,
    output: Option<String>,
    name: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    // Check if transaction exists first
    if stateful.state().transaction.is_none() {
        match format {
            OutputFormat::Json => {
                #[derive(Serialize)]
                struct TxResult {
                    success: bool,
                    error: String,
                }
                let result = TxResult {
                    success: false,
                    error: "No active transaction".to_string(),
                };
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Text => {
                print_error("No active transaction. Start a transaction first.");
            }
        }
        return Ok(());
    }

    // Create the savepoint - clone the document for serialization
    let doc_clone = stateful.document.clone();
    let tx = stateful.state_mut().transaction.as_mut().unwrap();
    tx.create_savepoint(name.clone(), &doc_clone)?;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct TxResult {
                success: bool,
                savepoint: String,
            }
            let result = TxResult {
                success: true,
                savepoint: name,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Savepoint '{}' created", name));
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}
