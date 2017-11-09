use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::io::{self, Write};
use super::interpreter;

struct IOError {
    info: String,
}

impl IOError {
    fn new(i: String) -> IOError {
        IOError{info: i}
    }
    fn panic(&self) -> i32 {
        println!("\nArgumentError:\n\t{}", self.info);
        return 1;
    }
}

/// Interprets the Wright file at the file name passed into the argument.
/// Returns the operating system exit code (Generally 0 for a success, 1 for a failure.).
pub fn interpret_file(input_file: String) -> i32 { // the i32 is exit code
    let file_error = IOError::new("Could not read input file.".to_string());
    // set input file if it can be opened
    let mut input_f = if let Ok(n) = File::open(input_file.clone()) {
        n
    } else {
        return file_error.panic();
    };
    let mut input_file_contents = String::new();
    if let Ok(_) = input_f.read_to_string(&mut input_file_contents) {} else {
        return file_error.panic();
    };
    // make call module from file name of module.
    let mut call: interpreter::Interpreter = interpreter::Interpreter::new(Path::new(&input_file)
                                                                               .file_name()
                                                                               .unwrap()
                                                                               .to_str()
                                                                               .unwrap()
                                                                               .to_string(),
                                                                           input_file_contents);
    return call.run();
    // assume success.
}