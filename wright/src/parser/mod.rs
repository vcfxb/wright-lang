//! Parser module for Wright.
//! Includes preliminary lexer function.
//extern crate regex;
//use self::regex::Regex;
use lexer::Lexer;
use lexer::error::LexerError;
use position::Position;

/// Abstract Syntax Tree Representation.
pub mod ast;
use parser::ast::*;

/// Error module.
pub mod error;
use parser::error::*;

/// This module contains functions for parsing tokens into representations.
pub mod parse_into;
use parser::parse_into::*;

#[derive(Debug, Clone)]
/// Parser struct.
pub struct Parser {
    pub module: ast::Module,
    source: String,
    lexer: Lexer,
}

/// Constant list of structural keywords.
pub const STRUCTURAL : [&'static str; 6]= [
    "class", "enum", "func", "trait", "impl", "const",
];

/// Constant list of keywords.
pub const KEYWORDS : [&'static str; 13] = [
    "pub",
    "class", "enum", "func", "trait", "impl", "const",
    "var", "let",
    "for", "while", "in",
    "requires",
];

impl Parser {
    /// Parses `self` from source to an AST (`ast::Module`).
    /// Lexes source to tokens internally.
    /// Returns an error containing an empty vector if there was en error while lexing.
    /// Prints any lexer errors.
    pub fn parse(&mut self) -> Result<(), Vec<ParserError>> {
        if let Err(e) = self.lex() {
            println!("{}", e);
            return Err(vec![]);
        }
        // no lexing error, go into the parsing process
        let mut error_vec: Vec<ParserError> = vec![];
        // parsing done here.
        let mut current_visibility = Visibility::Private;
        let mut current_line: String = String::new();
        let mut current_position = Position::new();
        let mut previous_position: Position;
        let mut tokens = self.lexer.tokens.clone();
        //println!("{:?}", tokens);
        tokens.reverse();
        'parse : while let Some(token) = tokens.pop() {
            previous_position = current_position;
            current_position.increment_column_by(token.len());
            if token.contains('\n'){current_position.increment_line_by(token.matches('\n').count())}
            current_line.push_str(token.as_str());
            if token == "pub" {
                    if current_visibility == Visibility::Public {
                        'take_line: while let Some(s) = tokens.pop() {
                            if s.contains('\n') {tokens.push(s); break 'take_line;}
                            current_line.push_str(s.as_str());
                        }
                        error_vec.push(ParserError::new(self.module.id.id.clone(),
                                previous_position,
                                current_line.clone(),
                                current_position, None)
                            .set_info_as_vec(STRUCTURAL.iter().map(|s| s.to_string()).collect(),
                                Some("pub".to_string())));
                    } else {current_visibility = Visibility::Public;}
            } else if token == KEYWORDS[1]  { //class
                if let Some(name) = tokens.pop() {
                    println!("{:?}", name);
                    previous_position = current_position;
                    current_position.increment_column_by(name.len());
                    if name.contains('\n'){
                        current_position.increment_line_by(name.matches('\n').count());
                        error_vec.push(ParserError::new(self.module.id.id.clone(),
                            previous_position,
                            current_line.clone(),
                            current_position,None)
                            .set_info("<class name>".to_string(),Some(name.clone())));
                    }
                    current_line.push_str(name.as_str());
                } else {}
            } else if token == KEYWORDS[2]  { //enum

            } else if token == KEYWORDS[3]  { //func

            } else if token == KEYWORDS[4]  { //trait

            } else if token == KEYWORDS[5]  { //impl

            } else if token == KEYWORDS[6]  { //const

            }
        }
        // post-parsing
        //println!("{:?}", error_vec);
        if error_vec.is_empty() {
            return Ok(());
        } else {
            return Err(error_vec);
        }
    }
    /// Constructor for parser representation.
    pub fn new(name: String, content: String) -> Self {
        Parser {
            module: ast::Module::new(name),
            source: content.clone(),
            lexer: Lexer::new(content),
        }
    }
    /// Transforms source into tokens, saving them into `self.tokens`.
    /// Returns `Ok(())` if there are no errors while lexing.
    /// If there is an error, it will be returned.
    pub fn lex(&mut self) -> Result<(),LexerError> {
        match self.lexer.lex() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}