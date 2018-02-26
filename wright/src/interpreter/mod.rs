//! Interpreter module.
//!


pub mod interpreter_error;
use interpreter::interpreter_error::*;

use super::parser::Parser;
use std::io;
use std::fs::File;
use super::errors;

#[derive(Debug, Copy, Clone)]
/// Interpreter struct.
pub struct Interpreter<'source> {
    /// Name of source file.
    pub file_name: &'source str,
    /// String of content read from source file.
    content:   &'source str,
}

impl<'source> Interpreter<'source> {
    /// Creates a new interpreter, reading the contents of the argument file.
    pub fn new(arg_file_name: &'source str) -> Interpreter<'source> {
        Interpreter {
            file_name: arg_file_name,
            content: {
                match File::open(arg_file_name) {
                    Ok(file_handle) => {
                        unimplemented!()
                    },
                    Err(_) => {
                        unimplemented!()
                    }
                }
            },
        }
    }
    /// Interpreter execution function
    pub fn run(&mut self) -> i32 {
        unimplemented!()
    }
}