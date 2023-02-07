//! Command line interface for wright.

use std::{path::PathBuf, fs, io};
use clap::{Parser, Subcommand};
use anyhow::Result;
use wright;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Subcommand for debugging wright's source code and interpreter. 
    Debug {
        #[command(subcommand)]
        command: DebugCommands
    }
}

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
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command{ 
        // Printing token debug information.
        Some(Commands::Debug { command: DebugCommands::Tokens { file, pretty }}) => {
            let file = fs::File::open(file)?;
            let reader = io::BufReader::new(file);
            let source = io::read_to_string(reader)?;
            
            
            if pretty {            
                wright::parsers::lexer::Lexer::debug_pretty_print(&source);
            } else {
                for token in wright::parsers::lexer::Lexer::lex(&source) {
                    println!("{}", token);
                }
            }
            
            Ok(())
        },

        _ => unimplemented!()
    }
}
