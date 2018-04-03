//! Interpreter module.
//! Contains struct and other items for representing an instance of the wright interpreter.

extern crate regex; 
use self::regex::Regex;
pub mod interpreter_error;
use interpreter::interpreter_error::*;
use errors::Error;
use std::io::Read;
use std::fs::File;

/// Enum for different levels of optimization.
/// See variant documentation for more detail.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimizations. Good for debugging. (Default)
    Debug,
    /// Normal, non-intensive optimizations. Should be adequate for most use cases involving performance.
    Release,
    /// Anything that can be done statically will be, including some (if not all)
    /// evaluation and execution.
    SuperCompiler,
}

impl Default for OptimizationLevel {
    fn default() -> Self { OptimizationLevel::Debug }
}

#[derive(Debug)]
/// Interpreter struct.
pub struct Interpreter<'source> {
    /// Name of source file.
    pub file_name  :   Option<&'source str>,
    /// String of content read from source file.
    pub contents   :   String,
    /// Whether or not the interpreter is in REPL mode.
    pub interactive:   bool,
    /// Optimization level to be used.  Will always be [`OptimizationLevel::Debug`] for REPLs.
    ///
    /// [`OptimizationLevel::Debug`]: ./enum.OptimizationLevel.html#variant.Debug
    pub level      :   OptimizationLevel,
    /// File handle of output file if there is one. ([`File`])
    ///
    /// [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
    pub output     :   Option<File>,
}

impl<'source> Interpreter<'source> {
    /// Creates a new interpreter reading from interactive input.
    pub fn new_interactive() -> Interpreter<'source> {
        Interpreter {
            interactive: true,
            file_name:   None,
            contents:    String::from(""),
            level:       OptimizationLevel::Debug,
            output:      None,
        }
    }

    /// Creates a new interpreter, reading the contents of the argument file.
    pub fn new(file_name: &'source str, level: OptimizationLevel, out: Option<&'source str>) -> Option<Interpreter<'source>> {    
        let mut buf: String = String::new();
        match File::open(file_name) {
            Ok(mut file_handle) => {
                match file_handle.read_to_string(&mut buf) {
                    Ok(_)  => {},
                    Err(_) => {
                        InterpreterError {
                            file_name,
                            reason:   "Could not read input file. (Was it valid UTF-8?)",
                        }.display();
                        return None;
                    }
                };
            },
            Err(_) => {
                InterpreterError {
                    file_name, reason: "Could not open input file. (Does it exist?)",
                }.display();
                return None;
            },
        };
        let mut output_name: String = match out {
            Some(n) => n,
            None    => file_name,
        }.to_string();
        
        let out_re = Regex::new(r"[[:word:]].class$").unwrap();
        
        if !out_re.is_match(&output_name) {
            output_name.push_str(".class");
        };
        
        let out_file: File;

        match File::create(&output_name) {
            Ok(f)  => {out_file = f;},
            Err(_) => {
                InterpreterError {
                    file_name: &output_name,
                    reason:   "Could not create or open output file.",
                }.display();
                return None;
            },
        };

        Some(Interpreter {
            contents: buf, 
            file_name: Some(file_name),
            interactive: false,
            level,
            output: Some(out_file),
        })
    }
    /// Interpreter execution function
    ///
    /// # Panics:
    /// Always; this function is not yet implemented.
    pub fn run(&self) -> i32 { unimplemented!() }
}