//! Interpreter module.
//!
use super::parser::Parser;
use super::lexer::Lexer;

#[derive(Debug)]
/// Interpreter struct
pub struct Interpreter {
    file_name: String,
    content: String,
    imports: Vec<Box<Interpreter>>,
    parser: Option<Parser>,
    lexer: Lexer,
}

impl Interpreter {
    /// Constructor.
    pub fn new(arg_file_name: String, arg_content: String,) -> Interpreter {
        Interpreter {
            file_name: arg_file_name.clone(),
            content: arg_content.clone(),
            imports: vec![],
            parser: None,
            lexer: Lexer::new(arg_content),
        }
    }
    /// File name accessor.
    pub fn get_name(&self) -> String { return self.file_name.clone();}
    /// Interpreter execution function
    pub fn run(&mut self) -> i32 {
        if let Err(mut e) = self.lexer.lex() {
            e.module_name = self.file_name.clone();
            1
        } else {0}
    }
}