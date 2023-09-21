//! Parsers module, for all the parsers implemented by wright and necessary to parse wright source code.

use codespan_reporting::files::Files;

use crate::filemap::{FileId, FileMap};

use self::{ast::declaration::Declaration, lexer::IndexedLexer};

pub mod ast;
pub mod lexer;

/// Parser to transform wright source code into the appropriate series of [AST] (Abstract Syntax Tree) nodes.
///
/// [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
#[derive(Debug)]
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
    type Item = Declaration<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
