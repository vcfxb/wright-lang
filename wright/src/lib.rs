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

/// Wright grammar module.
pub mod grammar;

/// Wright virtual machine module.
pub mod vm;

/// Enum of possible intermediate representations which can be emitted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Emit {
    /// Second pass tokens.
    Tokens,
    /// Lexer output.
    Lexemes,
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
    open_repl: bool,
}

impl Wright {
    /// Construct a new instance of the Wright interpreter.
    pub fn new() -> Self {
        Wright {
            files:   Files::new(),
            handles: Vec::new(),
            verbose: false,
            emits: HashSet::default(),
            open_repl: false
        }
    }

    /// Set this interpreters verbosity flag.
    pub fn set_verbose(&mut self, val: bool) -> &mut Self {
        self.verbose = val;
        self
    }

    /// Set this interpreters interactive flag (whether this interpreter will
    /// open a REPL session when called).
    pub fn set_interactive(&mut self, val: bool) -> &mut Self {
        self.open_repl = val;
        self
    }

    /// Add source to the Wright Interpreter.
    pub fn add_source(&mut self, name: impl Into<String> + std::fmt::Debug, content: impl Into<String>) -> &mut Self {
        if self.verbose {println!("Loading {:?}.", name)}
        let handle = self.files.add(name, content);
        self.handles.push(handle);
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

    /// Set certain emit flags.
    pub fn set_emit(&mut self, emit: Emit, value: bool) -> &mut Self {
        if value {
            self.emits.insert(emit);
        } else {
            self.emits.remove(&emit);
        }
        self
    }

    /// Set emit flags to true for all flags in given `emits` Vec,
    /// and false for any others.
    pub fn set_emits(&mut self, emits: Vec<Emit>) -> &mut Self {
        emits.iter().for_each(|e| {self.emits.insert(*e);});
        self
    }

    /// Calls and consumes this wright interpreter. Returns exitcode.
    pub fn call(self) -> ExitCode {
        for handle in self.handles {
            unimplemented!()
        }
        exitcode::OK
    }

}