//! Parsers module, for all the parsers implemented by wright and necessary to parse wright source code.

use self::lexer::Lexer;

pub mod ast;
pub mod lexer;

/// Parser to transform wright source code into the appropriate series of [AST] (Abstract Syntax Tree) nodes.
/// 
/// [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree 
#[derive(Debug)]
pub struct Parser<'a> {
    /// Reference to the source code we are parsing. 
    source: &'a str,
    /// Underlying lexer feeding tokens to this parser.  
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    
}




