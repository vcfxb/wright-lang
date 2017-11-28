use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::io::{self, Write};
use super::interpreter;

/// Interprets the Wright file at the file name passed into the argument.
/// Returns the operating system exit code (Generally 0 for a success, 1 for a failure.).
pub fn interpret_file(input_file: String) -> i32 { // the i32 is exit code
    // set input file if it can be opened
    let mut input_f = if let Ok(n) = File::open(input_file.clone()) {
        n
    } else {
        println!("Could not open {}.", input_file);
        return 1;
    };
    let mut input_file_contents = String::new();
    if let Ok(_) = input_f.read_to_string(&mut input_file_contents) {} else {
        println!("Could not read from {}.", input_file);
        return 1;
    };
    // make call module from file name of module.
    let mut call: interpreter::Interpreter = interpreter::Interpreter::
        new(Path::new(&input_file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        input_file_contents);
    return call.run();
    // assume success.
}