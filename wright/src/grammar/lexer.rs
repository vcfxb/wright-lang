use codespan::{Files, FileId, Span};
use codespan_reporting::diagnostic::Diagnostic;

/// Module for Wright's first pass over source code, which differentiates
/// characters into symbol groups.
pub mod symbols;
use symbols::*;

/// Module for recognizing escape sequences in wright source code.
pub mod escapes;

/// Module for second pass tokens. Used to differentiate words,
/// strings, numerical literals, and comments.
pub mod tokens;
use tokens::*;

/// Module for third pass lexemes. Used to differentiate keywords, identifiers,
/// document comments, and do parentheses grouping.
pub mod lexemes;
use lexemes::*;


/// A Lexer that transforms wright source code into tokens.
#[derive(Debug)]
pub struct Lexer<'s> {
    /// Reference to the source files database.
    files: &'s Files,
    /// The handle of the source being parsed in the files database.
    handle: FileId,
    span: Span,
    source: &'s str,
    syms: Vec<Sym>,
    tokens: Vec<Token>,
    lexemes: Vec<Lexeme>,
}

impl<'s> Lexer<'s> {
    /// Construct a new Lexer for a given source item in a Files database.
    pub fn new(files: &'s Files, handle: FileId) -> Self {
        let src = files.source(handle);
        let span = files.source_span(handle);
        let syms = do_pass(src.chars(), span);
        Self {
            files,
            handle,
            span,
            source: src,
            syms,
            tokens: Vec::new(),
            lexemes: Vec::new(),
        }
    }

    /// Do the token pass, returning either a modified lexer with the tokens
    /// parsed in or a diagnostic of what went wrong.
    pub fn tokens_pass(self) -> Result<Self, Diagnostic> {
        unimplemented!()
    }

    /// Do the Lexeme pass, returning either a modified lexer with the lexemes
    /// parsed and stored or a diagnostic of what went wrong.
    /// If the token pass has not been done yet then self will be returned
    /// unmodified.
    pub fn lexemes_pass(self) -> Result<Self, Diagnostic> {
        unimplemented!()
    }
}