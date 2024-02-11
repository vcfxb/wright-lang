//! Command line interface for wright.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use wright::repl;

/// The wright cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand passed to the wright cli.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Different sub-commands that the wright cli supports.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Subcommand for debugging wright's source code and interpreter.
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },

    /// Subcommand to start an interactive repl.
    Repl,
}

/// Different sub-commands that the debug sub-command supports.
#[derive(Subcommand, Debug)]
enum DebugCommands {
    /// Debug the tokens/lexemes for a source file.
    Tokens {
        /// A file of wright source code.
        file: PathBuf,

        /// Pretty print the source code with the tokens lined under them.
        /// If not used, a list of tokens will be printed with their metadata.
        #[arg(short, long)]
        pretty: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // Start an interactive repl.
        Some(Commands::Repl) => repl::start(),

        _ => unimplemented!(),
    }
}
