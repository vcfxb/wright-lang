use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use super::errors::*;
use super::preproc;

struct IOError {
    info: String,
    error_type: String,
    level: WrightErrorLevels,
}

impl WrightError for IOError {
    fn new(i: String, l: WrightErrorLevels) -> IOError {
        IOError{ info: i, error_type: "IOError".to_string(), level: l}
    }
    fn get_info(&self) -> String {
        self.info.clone()
    }
    fn get_type(&self) -> String {
        self.error_type.clone()
    }
    fn get_level(&self) -> WrightErrorLevels {
        self.level.clone()
    }
}

pub fn interactive() -> i32 { // the i32 is exit code
    // interactive_session lifetime for
    let prompt: String = "Wright".to_string();
    let mut line_number: u64 = 1;
    let output_error = IOError::new("Could not write to Standard Output.".to_string(), WrightErrorLevels::Fatal);
    let input_error = IOError::new("Could not read from Standard Input.".to_string(), WrightErrorLevels::Fatal);
    let stdin = io::stdin();
    'interactive_session: loop {
        print!("{}:{} >>> ", prompt, line_number);
        if let Ok(_) = io::stdout().flush() {
            // Do nothing
        } else {
            return output_error.panic();
        }
        let mut input_buffer = String::new();
        if let Ok(_) = stdin.read_line(&mut input_buffer) {
            // do nothing.
            // discard returned value
        } else {
            return input_error.panic();
        }
        // put analysis code here
        // todo : analysis of interactive input.
        let preprocessed_input = preproc::preproc(input_buffer);
        print!("{}", preprocessed_input[0].line);
        // end analysis here
        if let Ok(_) = io::stdout().flush() {
            // Do nothing
        } else {
            return output_error.panic();
        }
        line_number += 1;
    }
}

pub fn interpret(input_file: String) -> i32 { // the i32 is exit code
    let file_error = IOError::new("Could not open or read input file.".to_string(), WrightErrorLevels::Fatal);
    let mut input_f = if let Ok(n) = File::open(input_file) {
        n
    } else {
        return file_error.panic();
    };
    let mut input_file_contents = String::new();
    if let Ok(_) = input_f.read_to_string(&mut input_file_contents) {
        // do nothing and discard the number of bytes read.
    } else {
        return file_error.panic();
    };
    0
    // todo : analyse file in interpreted mode
}