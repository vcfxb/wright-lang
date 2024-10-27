// //! Command line interface for wright.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use wright::{
    lexer::Lexer, source_tracking::{source::Source, SourceMap, SourceRef}
};

/// The wright cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
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
        // /// Pretty print the source code with the tokens lined under them.
        // /// If not used, a list of tokens will be printed with their metadata.
        // #[arg(short, long)]
        // pretty: bool,
    },
}

fn main() -> Result<()> {
    // Parse the command line arguments.
    let cli: Cli = Cli::parse();

    match cli.command {
        // Print all the tokens for a given file.
        Some(Commands::Debug {
            command: DebugCommands::Tokens { file },
        }) => {
            let source_map: SourceMap = SourceMap::new();
            // Add the given file to the file map.
            let source_ref: SourceRef = source_map.add(Source::new_mapped_from_disk(file)?);
            // Make a lexer over the entirety of the given file.
            let mut lexer: Lexer = Lexer::new(source_ref);
            // Get all the tokens from the lexer and print them each.
            while let Some(token) = lexer.next_token() {
                println!("{token}");
            }
        }

        _ => unimplemented!(),
    }
    
    Ok(())
}
