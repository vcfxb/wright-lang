// //! Command line interface for wright.

// use anyhow::Result;
// use clap::{Parser, Subcommand};
// use codespan_reporting::files::Files;
// use std::{path::PathBuf, time::Instant};
// use wright::{
//     filemap::{FileId, FileMap},
//     parser::lexer::{token::Token, Lexer},
//     repl,
// };

// /// The wright cli.
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None, arg_required_else_help = true)]
// struct Cli {
//     /// The subcommand passed to the wright cli.
//     #[command(subcommand)]
//     command: Option<Commands>,

//     /// Output elapsed timing information at the end of the command.
//     #[arg(short, long)]
//     timed: bool,
// }

// /// Different sub-commands that the wright cli supports.
// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Subcommand for debugging wright's source code and interpreter.
//     Debug {
//         #[command(subcommand)]
//         command: DebugCommands,
//     },

//     /// Subcommand to start an interactive repl.
//     Repl,
// }

// /// Different sub-commands that the debug sub-command supports.
// #[derive(Subcommand, Debug)]
// enum DebugCommands {
//     /// Debug the tokens/lexemes for a source file.
//     Tokens {
//         /// A file of wright source code.
//         file: PathBuf,
//         // /// Pretty print the source code with the tokens lined under them.
//         // /// If not used, a list of tokens will be printed with their metadata.
//         // #[arg(short, long)]
//         // pretty: bool,
//     },
// }

// fn main() -> Result<()> {
//     // Parse the command line arguments.
//     let cli: Cli = Cli::parse();
//     // Get the start time to track duration if asked.
//     let start: Instant = Instant::now();

//     match cli.command {
//         // Start an interactive repl.
//         Some(Commands::Repl) => {
//             repl::start()?;
//         }

//         // Print all the tokens for a given file.
//         Some(Commands::Debug {
//             command: DebugCommands::Tokens { file },
//         }) => {
//             let mut file_map: FileMap = FileMap::new();
//             // Add the given file to the file map.
//             let file_id: FileId = file_map.add_file(file)?;
//             // Make a lexer over the entirety of the given file.
//             // Use unwrap here, since we know we just added the file.
//             let lexer: Lexer = Lexer::new(file_map.source(file_id).unwrap());
//             // Get all the tokens from the lexer and print them each.
//             lexer.for_each(|token: Token| println!("{token}"));
//         }

//         _ => unimplemented!(),
//     }

//     // Handle timing info.
//     if cli.timed {
//         println!("\nTotal time elapsed since parsing arguments: {:?}", Instant::now() - start);
//     }

//     Ok(())
// }

fn main() {
    todo!("Wright frontend undergoing overhaul");
}
