//! Interpreter module.
//!

pub mod interpreter_error;
use interpreter::interpreter_error::*;
use errors::Error;

use std::io::Read;
use std::fs::File;

#[derive(Debug, Clone)]
/// Interpreter struct.
pub struct Interpreter<'source> {
    /// Name of source file.
    pub file_name: &'source str,
    /// String of content read from source file.
    content:       String,
}

impl<'source> Interpreter<'source> {
    /// Creates a new interpreter, reading the contents of the argument file.
    pub fn new(arg_file_name: &'source str) -> Option<Interpreter<'source>> {    
        let mut buf: String = String::new();
        match File::open(arg_file_name) {
            Ok(mut file_handle) => {
                match file_handle.read_to_string(&mut buf) {
                    Ok(_)  => {},
                    Err(_) => {
                        InterpreterError {
                            file_name: arg_file_name,
                            reasons:   &["Could not read file. (Was it valid UTF-8?)"],
                        }.display();
                        return None;
                    }
                };
            },
            Err(_) => {
                unimplemented!();
                return None;
            },
        };
        return Some(Interpreter {
            file_name: arg_file_name,
            content: buf,
        });
    }
    /// Interpreter execution function
    pub fn run(&mut self) -> i32 {
        unimplemented!()
    }
}