//! Parser module for Wright.
//! Includes preliminary lexer function.
//extern crate regex;
//use self::regex::Regex;
/// Abstract Syntax Tree Representation
pub mod ast;
use super::lexer::*;

#[derive(Debug, Clone)]
/// Parser struct.
pub struct Parser {
    pub module: ast::Module,
    pub original_content: String,
    pub lexer: Lexer,
}

impl Parser {
    // todo: docs
    ///
    pub fn parse(&mut self) {
        // todo:
        self.lex();
    }
    // todo: docs
    ///
    pub fn new(name: String, content: String) -> Self {
        Parser {
            module: ast::Module::new(name),
            original_content: content.clone(),
            lexer: Lexer::new(content),
        }
    }
    // todo: docs
    ///
    pub fn lex(&mut self) {
        self.lexer.lex();
    }
}

pub enum ParserError {
    UnexpectedToken(String, u64, u64, u64),
    MissingExpectedToken(String, u64, u64, u64),
    InvalidAssignmentTarget(String, u64, u64, u64)
}

