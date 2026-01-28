use std::{io, path::PathBuf};

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, generate_to, Shell};

use crate::cli::Cli;

pub fn generate(shell: Shell, directory: Option<PathBuf>) -> Result<()> {
    let mut command = Cli::command();

    if let Some(dir) = directory {
        std::fs::create_dir_all(&dir)?;
        let path = generate_to(shell, &mut command, "ucp", dir)?;
        eprintln!("Generated completion: {}", path.display());
    } else {
        let mut stdout = io::stdout();
        generate(shell, &mut command, "ucp", &mut stdout);
    }

    Ok(())
}
