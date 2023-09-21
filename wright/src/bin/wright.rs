//! Command line interface for wright.

use anyhow::Result;
use clap::{Parser, Subcommand};
use codespan_reporting::files::SimpleFile;
use std::{fs, path::PathBuf};
use wright::parser::lexer::Lexer;

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
        // Printing token debug information.
        Some(Commands::Debug {
            command: DebugCommands::Tokens { file, pretty },
        }) => {
            let source_str = fs::read_to_string(&file)?;
            let source = SimpleFile::new(file.to_string_lossy(), &source_str);

            if pretty {
                Lexer::debug_pretty_print(&source)?;
            } else {
                for token in Lexer::new(&source_str) {
                    println!("{}", token);
                }
            }

            Ok(())
        }

        _ => unimplemented!(),
    }
}
