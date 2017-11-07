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
            file_name: arg_file_name,
            content: arg_content,
            imports: vec![],
            parser: Parser::new()
        }
    }
    /// File name accessor.
    pub fn get_name(&self) -> String { return self.file_name.clone();}

}