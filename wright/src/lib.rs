#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

//! The Wright programming language crate.
//!

#[macro_use]
extern crate pest_derive;

use codespan::{
    Files,
    FileId,
};

use exitcode::ExitCode;

use std::fs::File;
use std::io::Read;

pub mod grammar;

use grammar::parse;

/// Enum of possible intermediate representations which can be emitted.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Emit {
    /// Tokens, or Lexemes. [see wikipedia](https://en.wikipedia.org/wiki/Lexical_analysis)
    Tokens,
    /// The Abstract Syntax Tree. [see wikipedia](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
    AbstractSyntaxTree,
}

/// Call the Wright compiler system on a set of files with given options.
/// Returns system exit code.
pub fn call_files(filenames: Vec<&str>, run: bool, emits: Vec<Emit>, verbose: bool) -> ExitCode {
    let mut files = Files::new();
    let mut handles: Vec<FileId> = Vec::with_capacity(filenames.len());
    for file_name in filenames {
        let mut f = match File::open(file_name) {
            Ok(f) => f,
            Err(e) => {
                println!("Could not open {}.", file_name);
                return exitcode::NOINPUT;
            }
        };
        let mut source = String::new();
        if let Err(_) = f.read_to_string(&mut source) {
            println!("Could not read {}.", file_name);
            return exitcode::NOINPUT;
        };
        let handle = files.add(file_name, source);
        if verbose {eprintln!("Loaded {} with handle {:?}.", file_name, handle);}
        handles.push(handle);
    }
    return call(files, handles, run, emits, verbose);
}

/// Call the Wright compiler on a given input, with given options.
/// Returns system exit code.
pub fn call(files: Files, handles: Vec<FileId>, run: bool, emits: Vec<Emit>, verbose: bool) -> ExitCode {
    for handle in handles {
        parse(files.source(handle))
    }
    return 0;
}