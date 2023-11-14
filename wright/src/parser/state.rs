//! Parser state structure and implementation.

use super::{ast::metadata::AstNodeMeta, lexer::{IndexedLexer, IndexedToken, tokens::{TokenTy, Token}}};
use crate::filemap::{FileId, FileMap};
use codespan_reporting::files::Files;

#[cfg(doc)]
use std::iter::Peekable;
use std::ops::Range;

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
    lexer: IndexedLexer<'src>,
    /// Store up to one "peeked" token, similar to the [`Peekable`] iterator. 
    /// See https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek for design inspiration. 
    peeked_token: Option<Option<IndexedToken>>
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
            peeked_token: None,
        }
    }

    /// Make a new [`AstNodeMeta`] object using this [`ParserState`]'s [`FileMap`] and [`FileId`].
    /// The byte index and byte length in source code are supplied as arguments, usually from the
    /// [`IndexedToken`] pulled from this [ParserState]'s internal [`IndexedLexer`].
    pub fn make_ast_node_meta(&self, index: usize, length: usize) -> AstNodeMeta<'src> {
        AstNodeMeta {
            file_map: self.file_map,
            file_id: self.file_id,
            index,
            matching_source: &self.source[index..index + length],
        }
    }

    /// Peek a token from the internal lexer.
    pub fn peek_token(&mut self) -> Option<&IndexedToken> {
        // Get a mutable reference to the internal iterator.
        let iter = &mut self.lexer;
        // Get the previously peeked token or a new one from the iterator. 
        self.peeked_token.get_or_insert_with(|| iter.next()).as_ref()
    }

    /// Peek the type of the next token. 
    pub fn peek_token_ty(&mut self) -> Option<&TokenTy> {
        self.peek_token().map(|indexed_token| &indexed_token.token.variant)
    }

    /// Get the starting byte index of the next [`IndexedToken`] in source code. 
    pub fn index(&self) -> usize {
        // Check to see if a token has been peeked already. 
        match self.peeked_token.as_ref() {
            // If one has, return its index.
            Some(Some(IndexedToken { index, .. })) => *index,

            // Otherwise return the current lexer index. 
            _ => self.lexer.index
        }
    }

    /// Get the next token from the internal lexer. 
    pub fn next_token(&mut self) -> Option<IndexedToken> {
        match self.peeked_token.take() {
            Some(peeked_token) => peeked_token,
            None => self.lexer.next()
        }
    }

    /// Get the next token if it satisfies a given predicate. 
    pub fn next_token_if_ty_eq(&mut self, token_ty: TokenTy) -> Option<IndexedToken> {
        // Get the next token or consume a previously peeked token. 
        match self.next_token() {
            // Token with matching variant field.
            Some(token @ IndexedToken { token: Token { variant, .. }, .. }) if variant == token_ty => Some(token),

            // Otherwise save the peeked token (the field should have been consumed by calling next_token)
            other => {
                // Sanity check. 
                assert!(self.peeked_token.is_none());
                self.peeked_token = Some(other);
                None
            }
        }
    }

    /// Get the byte range of the next token. If there is no next token in the lexer, return a zero-length 
    /// range of the current index. 
    pub fn peek_byte_range(&mut self) -> Range<usize> {
        if let Some(IndexedToken { index, token: Token { length, .. } }) = self.peek_token() {
            *index..*index+*length
        } else {
            self.index()..self.index()
        }
    }
}
