use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, Write};
use super::errors::*;
use super::lexer;

/// Raised when Wright is unable to read from the given file.
pub struct IOError {
    info: String,
    error_type: String,
    level: WrightErrorLevels,
}

impl WrightError for IOError {
    /// Constructor.
    fn new(i: String, l: WrightErrorLevels) -> IOError {
        IOError{ info: i, error_type: "IOError".to_string(), level: l}
    }
    fn get_info(&self) -> String { self.info.clone() }
    fn get_type(&self) -> String { self.error_type.clone() }
    fn get_level(&self) -> WrightErrorLevels { self.level.clone() }
}

/// Interprets the Wright file at the file name passed into the argument.
/// Returns the operating system exit code (Generally 0 for a success, 1 for a failure.).
pub fn interpret(input_file: String) -> i32 { // the i32 is exit code
    let file_error = IOError::new("Could not open or read input file.".to_string(), WrightErrorLevels::Fatal);
    let mut input_f = if let Ok(n) = File::open(input_file) {
        n
    } else {
        return file_error.panic();
    };
    let mut input_file_contents = String::new();
    if let Ok(_) = input_f.read_to_string(&mut input_file_contents) {} else {
        return file_error.panic();
    };
    // work in progress
    println!("{:?}", lexer::lex_lines(input_file_contents.clone()));

    return 0;
    // assume success.
}