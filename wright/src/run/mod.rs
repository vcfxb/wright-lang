use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, Write};
use super::interpreter;

struct IOError {
    info: String,
    error_type: String,
    level: String,
}

impl IOError {
    fn new(i: String, l: String) -> IOError {
        IOError{ info: i, error_type: "IOError".to_string(), level: l}
    }
    fn panic(&self) -> i32 {
        println!("
{}:{}:
    {}
        ", self.level, self.error_type, self.info);
        return 1;
    }
}

/// Interprets the Wright file at the file name passed into the argument.
/// Returns the operating system exit code (Generally 0 for a success, 1 for a failure.).
pub fn interpret_file(input_file: String) -> i32 { // the i32 is exit code
    let file_error = IOError::new("Could not open or read input file.".to_string(), "Fatal".to_string());
    let mut input_f = if let Ok(n) = File::open(input_file.clone()) {
        n
    } else {
        return file_error.panic();
    };
    let mut input_file_contents = String::new();
    if let Ok(_) = input_f.read_to_string(&mut input_file_contents) {} else {
        return file_error.panic();
    };
    let call: interpreter::Interpreter = interpreter::Interpreter::new(input_file, input_file_contents);
    // todo: CallHandler
    return 0;
    // assume success.
}