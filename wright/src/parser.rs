//! Parsers module, for all the parsers implemented by wright and necessary to parse wright source code.

use self::{lexer::{tokens::Token, IndexedLexer}, ast::Declaration};

pub mod ast;
pub mod lexer;

/// Parser to transform wright source code into the appropriate series of [AST] (Abstract Syntax Tree) nodes.
/// 
/// [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree 
pub struct Parser<'src> {
    /// Reference to the source code we are parsing. 
    source: &'src str,
    /// Underlying indexed lexer feeding tokens to this parser. 
    lexer: IndexedLexer<'src>
}

/// An error in the source code caught by the parser. 
pub struct ParserError<'src> {
    /// The source code that the error was in. 
    source: &'src str,
    
}

impl<'a> Parser<'a> {
    /// Construct a new parser for a given source string. 
    pub fn new(source: &'a str) -> Self { 
        Parser { 
            source, 
            lexer: IndexedLexer::new(source)
        }
    }
}


impl<'src> Iterator for Parser<'src> {
    type Item = Declaration<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
