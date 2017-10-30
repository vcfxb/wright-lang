//! Parser module for Wright.
//! Includes preliminary lexer function.
//extern crate regex;
//use self::regex::Regex;
mod ast;
use super::lexer;
/// Parser struct.
pub struct Parser {
    // todo: parser
}

impl Parser {
    pub fn parse(&mut self) -> Result<ast::Module, Vec<ParserError>> {
        // todo: parser
    }
}

pub enum ParserError {
    UnexpectedToken(String, u64, u64, u64),
    MissingExpectedToken(String, u64, u64, u64),
    InvalidAssignmentTarget(String, u64, u64, u64)
}

