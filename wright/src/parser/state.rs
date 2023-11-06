//! Parser state structure and implementation.

use super::{ast::metadata::AstNodeMeta, lexer::IndexedLexer};
use crate::filemap::{FileId, FileMap};
use codespan_reporting::files::Files;

/// The state of the [`Parser`] used to transform wright source code into the
/// appropriate series of [AST] (Abstract Syntax Tree) nodes.
///
/// [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
#[derive(Debug, Clone)]
pub struct ParserState<'src> {
    /// The file map that this parser's parent file is in.
    file_map: &'src FileMap,
    /// The file handle for the file being parsed.
    file_id: FileId,
    /// Reference to the source code we are parsing.
    pub source: &'src str,
    /// Underlying indexed lexer feeding tokens to this parser.
    pub lexer: IndexedLexer<'src>,
}

impl<'src> ParserState<'src> {
    /// Construct a new parser for a given source file.
    ///
    /// # Panics:
    /// If the file ID is not in the file map.
    pub fn new(file_map: &'src FileMap, file_id: FileId) -> Self {
        // Get the source using the file map.
        let source = file_map
            .source(file_id)
            .expect("file id exists in file map");

        ParserState {
            file_map,
            file_id,
            source,
            lexer: IndexedLexer::new(source),
        }
    }

    /// Make a new [`AstNodeMeta`] object using this [`Parser`]'s [`FileMap`] and [`FileId`].
    /// The byte index and byte length in source code are supplied as arguments, usually from the
    /// [`IndexedToken`] pulled from this [Parser]'s internal [`IndexedLexer`].
    pub fn make_ast_node_meta(&self, index: usize, length: usize) -> AstNodeMeta<'src> {
        AstNodeMeta {
            file_map: self.file_map,
            file_id: self.file_id,
            index,
            matching_source: &self.source[index..index + length],
        }
    }
}
