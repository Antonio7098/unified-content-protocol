//! UCP CLI - Command-line interface for Unified Content Protocol
//!
//! This CLI provides comprehensive access to UCP functionality including
//! document management, block operations, UCL execution, and agent traversal.

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod commands;
mod error;
mod output;
mod state;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize tracing based on verbosity
    init_tracing(cli.verbose, cli.trace);

    // Run the CLI
    cli.run()
}

fn init_tracing(verbose: bool, trace: bool) {
    let filter = if trace {
        EnvFilter::new("trace")
    } else if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("warn")
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
}
