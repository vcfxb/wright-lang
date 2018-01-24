//! Interpreter module.
//!
use super::parser::Parser;

extern crate ansi_term;
use self::ansi_term::Color;
use self::ansi_term::Color::*;

/// Color code for errors used throughout entire error reporting system.
pub const ERROR_COLORS: [Color; 4] = [
    Red, Cyan, Green, Cyan,
];

#[derive(Debug)]
/// Interpreter struct
pub struct Interpreter {
    file_name: String,
    content: String,
    imports: Vec<Box<Interpreter>>,
    parser: Parser,
}

impl Interpreter {
    /// Constructor.
    pub fn new(arg_file_name: String, arg_content: String,) -> Interpreter {
        Interpreter {
            file_name: arg_file_name.clone(),
            content: arg_content.clone(),
            imports: vec![],
            parser: Parser::new(arg_file_name, arg_content.clone()),
        }
    }
    /// File name accessor.
    pub fn get_name(&self) -> String { return self.file_name.clone();}
    /// Interpreter execution function
    pub fn run(&mut self) -> i32 {
        //println!("{}:", self.file_name);
        let parser_result = self.parser.parse();
        //println!("{:?}", parser_result);
        match parser_result {
            Ok(_) => 0,
            Err(vec) => {
                for e in vec {
                    println!("{}", e);
                }
                return 1;
            },
        }
    }
}