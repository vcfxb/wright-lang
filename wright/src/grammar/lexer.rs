#![allow(missing_docs)]

use codespan::{Span, Index, ByteIndex, Files, FileId};

use codespan_reporting::diagnostic::{
    Severity, Label, Diagnostic
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// The type of a token in wright source code.
pub enum TokenType {
    // one character
    LeftCurly, RightCurly, LeftBracket, RightBracket, LeftParen, RightParen,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Bang, Tilde, Amp, Bar,
    GreaterThan, LessThan, Equals, Underscore, Carrot, Percent, Cash, Pound,
    At, Question, Colon,
    // two chars
    EQ, NEQ, GEQ, LEQ, DoubleColon, Returns, Cast,
    // literals
    Integer, String, Character, True, False, SelfVal,
    // keywords
    Fn, Class, Struct, Trait, SelfType, Type, Const, Mut, Let, If, Then, Else,
    Pub, Import, Mod, Match,
    // other
    End
}

#[derive()]
/// A token in wright source code.
pub struct Token {
    pub span: Span,
    pub ty: TokenType,
}

/// A tool to scan and get tokens from Wright source code.
pub struct Lexer<'s> {
    span: Span,
    index: ByteIndex,
    tokens: Vec<Token>,
    files_ref: &'s Files,
    handle: FileId,
    source: &'s str,

}

impl<'s> Lexer<'s> {
    fn advance(&mut self) -> char {
        let start = self.index;
        self.index += 1;
        while self.source.is_char_boundary(self.index-self.span.start()-1) {
            self.index += 1;
        }
        
    }

    /// Construct a new Lexer for an item in a Files database.
    pub fn new(files: &'s Files, handle: FileId) -> Self {
        let mut s = Lexer {
            span: files.source_span(handle),
            index: ByteIndex::default(),
            tokens: Vec::new(),
            files_ref: files,
            handle,
            source: files.source(handle)
        };
        s.index = s.span.start();
        s
    }

    /// Check if this lexer is at the end of input.
    pub fn is_at_end(&self) -> bool {
        self.index >= self.span.end()
    }

    /// Call this lexer instance.
    pub fn call(mut self) -> Result<Vec<Token>, Diagnostic> {
        unimplemented!()
    }
}