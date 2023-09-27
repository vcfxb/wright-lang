//! Parsers module, for all the parsers implemented by wright and necessary to parse wright source code.

use std::ops::Range;
use codespan_reporting::files::Files;
use num::{BigUint, Num};
use crate::filemap::{FileId, FileMap};
use self::{ast::{declaration::Declaration, expression::literal::IntegerLiteral, metadata::AstNodeMeta}, lexer::{IndexedLexer, IndexedToken, tokens::{Token, TokenTy}}};
use std::cmp;

pub mod ast;
pub mod lexer;

/// Parser to transform wright source code into the appropriate series of [AST] (Abstract Syntax Tree) nodes.
///
/// [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
#[derive(Debug, Clone)]
pub struct Parser<'src> {
    /// The file map that this parser's parent file is in.
    file_map: &'src FileMap,
    /// The file handle for the file being parsed.
    file_id: FileId,
    /// Reference to the source code we are parsing.
    source: &'src str,
    /// Underlying indexed lexer feeding tokens to this parser.
    lexer: IndexedLexer<'src>,
}

/// An error that can occur during parsing. 
#[derive(Debug)]
pub struct ParserError {
    /// The byte index range of the offending line in the file being parsed. 
    byte_range: Range<usize>,
    /// The type of error. 
    ty: ParserErrorVariant
}

/// Different types of errors that can be generated duruing parsing. 
#[derive(Debug)]
enum ParserErrorVariant {
    /// Something was expected and wasn't there. 
    Expected(&'static str)
}

impl<'a> Parser<'a> {
    /// Construct a new parser for a given source file.
    ///
    /// ## Panics:
    /// Note that the file id is expected to be in the file map, and this will panic if not.
    pub fn new(file_map: &'a FileMap, file_id: FileId) -> Self {
        // Get the source using the file map.
        let source = file_map
            .source(file_id)
            .expect("file id exists in file map");

        Parser {
            file_map,
            file_id,
            source,
            lexer: IndexedLexer::new(source),
        }
    }
}

impl<'src> Iterator for Parser<'src> {
    type Item = Result<Declaration<'src>, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'src> Parser<'src> {
    /// Replace the internal lexer iterator with a different one that has been used to consume tokens. 
    fn update_lexer(&mut self, new: IndexedLexer<'src>) -> AstNodeMeta<'src> {
        // Construct AST node metadata by subtracting one cursor from the other. 
        let meta = AstNodeMeta { 
            file_id: self.file_id, 
            index: self.lexer.index, 
            matching_source: &self.source[self.lexer.index..(new.index-self.lexer.index)]
        };

        // Replace the internal lexer. 
        self.lexer = new;
        
        // Return constructed metadata.
        return meta;
    }

    /// Parse an integer literal or error. 
    fn parse_integer(&mut self) -> Result<IntegerLiteral<'src>, ParserError> {
        // Clone the current lexer (token cursor) to parse an integer. 
        let mut lexer = self.lexer.clone();
        
        // Take an integer literal token from the lexer or error.
        match lexer.next() {
            Some(IndexedToken { index, token: Token { variant: TokenTy::IntegerLit, length } }) => {
                // Get the matching source of this token. 
                let matching_source = &self.source[index..index+length];
                
                // Check for a prefix
                let prefix = &matching_source[..cmp::max(2, matching_source.len())];

                // Get a radix off the prefix
                let radix = match prefix {
                    "0x" | "0X" => 16,
                    "0b" | "0B" => 2,
                    "0o" => 8,
                    _ => 10,
                };

                // Strip the prefix from the string to get the body of it to parse. 
                let body = if radix != 10 { &matching_source[2..] } else { matching_source }; 

                // Parse it.
                let value = BigUint::from_str_radix(body, radix)
                    // Panic here as the lexer should check for this.
                    .expect("lexer checks integer literal format");

                Ok(IntegerLiteral { meta: self.update_lexer(lexer), value })
            },

            _ => Err(ParserError { byte_range: self.lexer.index..lexer.index, ty: ParserErrorVariant::Expected("integer literal") })
        }
    }
}
