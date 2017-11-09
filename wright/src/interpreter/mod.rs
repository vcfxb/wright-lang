//! Interpreter module.
//!
use super::parser::Parser;

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
        println!("{}:", self.file_name);
        let lexer_result = self.parser.lex();
        match lexer_result {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}