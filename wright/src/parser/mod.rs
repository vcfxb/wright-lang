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
    pub source: String,
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
            source: content.clone(),
            lexer: Lexer::new(content),
        }
    }
    // todo: docs
    // returns double empty Result for
    ///
    pub fn lex(&mut self) -> Result<(),()> {
        let lex_result = self.lexer.lex();
        if let Ok(_) = lex_result {
            println!("{:?}", self.lexer.tokens);
            return Ok(());
        } else if let Err(mut e) = lex_result {
            e.set_module_name(self.module.id.id.clone());
            print!("{}", e);
            return Err(());
        } else {Err(())}
    }
}

// todo:
pub enum ParserError {
}

