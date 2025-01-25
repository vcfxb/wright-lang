//! Command line interface for wright.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use wright::{
    lexer::Lexer,
    source_tracking::{source::Source, SourceMap, SourceRef},
};

/// The wright cli.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Whether the output should be only ASCII characters (default auto-detected, if `supports-unicode`
    /// crate is compiled in).
    ///
    /// This option does nothing if the `supports-unicode` crate was not enabled at compile time (in that case all
    /// output will be ASCII regardless).
    #[arg(short = 'A', long = "ascii")]
    force_ascii: bool,
    /// The subcommand passed to the wright cli.
    #[command(subcommand)]
    command: Command,
}

/// Different sub-commands that the wright cli supports.
#[derive(Subcommand, Debug)]
enum Command {
    /// Subcommand for debugging wright's source code and interpreter.
    Debug {
        #[command(subcommand)]
        command: DebugCommand,
    },

    /// Subcommand for showing information about this version of wright.
    Show {
        #[command(subcommand)]
        command: ShowCommand,
    },
}

/// Different sub-commands that the debug sub-command supports.
#[derive(Subcommand, Debug)]
enum DebugCommand {
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

/// Different subcommands that can be used to get info about a copy of the wright CLI/compiler/etc.
#[derive(Subcommand, Debug)]
enum ShowCommand {
    /// Get the version string of this copy of the wright compiler.
    Version,

    /// Get the full list of feature names/strings that were enabled when this copy of wright was compiled.
    Features,
}

fn main() -> Result<()> {
    // Parse the command line arguments.
    let cli: Cli = Cli::parse();

    #[cfg(feature = "supports-unicode")]
    {
        wright::util::supports_unicode::set_force_ascii(cli.force_ascii);
    }

    match cli.command {
        // Print all the tokens for a given file.
        Command::Debug {
            command: DebugCommand::Tokens { file },
        } => {
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

        Command::Show {
            command: ShowCommand::Version,
        } => {
            println!("wright {}", wright::build_info::PKG_VERSION);
        }

        Command::Show {
            command: ShowCommand::Features,
        } => {
            for feature in wright::build_info::FEATURES {
                println!("{feature}");
            }
        }
    }

    Ok(())
}
