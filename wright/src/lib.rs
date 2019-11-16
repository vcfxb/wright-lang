#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

//! The Wright programming language crate.
//!

use codespan::{
    Files,
    FileId,
};

use exitcode::ExitCode;

use std::collections::HashSet;
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

#[derive(Debug, Clone)]
/// Interpreter instance.
pub struct Wright {
    files: Files,
    handles: Vec<FileId>,
    verbose: bool,
    emits: HashSet<Emit>,
}

impl Wright {
    /// Construct a new instance of the Wright interpreter.
    pub fn new(verbose: bool) -> Self {
        Wright {
            files:   Files::new(),
            handles: Vec::new(),
            verbose,
            emits: HashSet::default()
        }
    }

    /// Add source to the Wright Interpreter.
    pub fn add_source(&mut self, name: impl Into<String>, content: impl Into<String>) -> &mut Self {
        let handle = self.files.add(name, content);
        self.handles.push(handle);
        if self.verbose { println!("Loaded {}.", name); }
        self
    }

    /// Add several files to this Wright Interpreter.
    pub fn add_files(&mut self, filenames: Vec<&str>) -> std::io::Result<&mut Self> {
        for fname in filenames {
            let mut f= File::open(fname)?;
            let mut source = String::new();
            f.read_to_string(&mut source)?;
            self.add_source(fname, source);
        }
        Ok(self)
    }

    /// Toggle certain emit information.
    pub fn set_emit(&mut self, emit: Emit, value: bool) -> &mut Self {
        if value {
            self.emits.insert(emit);
        } else {
            self.emits.remove(&emit);
        }
        self
    }

    pub fn call(self) {

    }
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