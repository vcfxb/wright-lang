//! Interpreter module.
//!
use super::parser;

#[derive(Debug)]
/// Interpreter struct
pub struct Interpreter {
    file_name: String,
    content: String,
    imports: Box<Vec<Interpreter>>,
    parser: parser::Parser,
}

impl Interpreter {
    /// Constructor.
    pub fn new(arg_file_name: String, arg_content: String,) -> Interpreter {
        Interpreter {
            file_name: arg_file_name,
            content: arg_content,
            imports: Box::new(vec![]),
        }
    }
    /// File name accessor.
    pub fn get_name(&self) -> String { return self.file_name.clone();}

}