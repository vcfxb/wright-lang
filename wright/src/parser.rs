//! Parsers module, for all the parsers implemented by wright and necessary to parse wright source code.

use self::{
    ast::{declaration::Declaration, metadata::AstNodeMeta},
    lexer::IndexedLexer,
};
use crate::filemap::{FileId, FileMap};
use codespan_reporting::files::Files;
use std::ops::Range;

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
    ty: ParserErrorVariant,
}

/// Different types of errors that can be generated duruing parsing.
#[derive(Debug)]
enum ParserErrorVariant {
    /// Something was expected and wasn't there.
    Expected(&'static str),
}

/// Parser version of [`Result`].
pub type ParserResult<T> = Result<T, ParserError>;

impl<'src> Iterator for Parser<'src> {
    type Item = Result<Declaration<'src>, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'src> Parser<'src> {
    /// Construct a new parser for a given source file.
    ///
    /// # Panics:
    /// If the file ID is not in the file map.
    pub fn new(file_map: &'src FileMap, file_id: FileId) -> Self {
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

    /// Replace the internal lexer iterator with an updated one that has been used to consume tokens.
    ///
    /// Return a node metadata representing the change in lexer state.
    fn update_lexer(&mut self, new: IndexedLexer<'src>) -> AstNodeMeta<'src> {
        // Construct AST node metadata by slicing from one cursor to the next.
        let meta = AstNodeMeta {
            file_id: self.file_id,
            index: self.lexer.index,
            matching_source: &self.source[self.lexer.index..new.index],
            file_map: self.file_map,
        };

        // Replace the internal lexer.
        self.lexer = new;

        // Return constructed metadata.
        return meta;
    }
}
