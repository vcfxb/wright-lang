#![allow(missing_docs)]

use codespan::{Span, Index, ByteIndex, Files, FileId, ByteOffset};

use codespan_reporting::diagnostic::{
    Severity, Label, Diagnostic
};
use std::str::Chars;
use std::borrow::Borrow;
use std::iter::Peekable;
use crate::grammar::ast::Type::SelfType;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// The type of a token in wright source code.
pub enum TokenType {
    // one character
    LeftCurly, RightCurly, LeftBracket, RightBracket, LeftParen, RightParen,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Bang, Tilde, Amp, Bar,
    GreaterThan, LessThan, Equals, Underscore, Carrot, Percent, Cash,
    At, Question, Colon,
    // two chars
    EQ, NEQ, GEQ, LEQ, DoubleColon, Returns, Matches, DoubleDiv, OR, AND,

    // literals
    Integer, String, Character, True, False, SelfVal,
    // keywords
    Fn, Class, Struct, Trait, SelfType, Type, Const, Mut, Let, If, Then, Else,
    Pub, Import, Mod, Match,
    // other
    End
}

#[derive(Debug)]
/// A token in wright source code.
pub struct Token {
    pub span: Span,
    pub ty: TokenType,
    pub literal: Option<String>
}

impl Token {
    /// Construct new token.
    fn new(start: ByteIndex, end: ByteIndex, ty: TokenType) -> Self {
        Token {
            span: Span::new(start, end),
            ty,
            literal: None
        }
    }

    /// Set literal field.
    fn literal(mut self, lit: String) -> Self {
        self.literal = Some(lit);
        self
    }
}

/// A tool to scan and get tokens from Wright source code.
pub struct Lexer<'s> {
    span: Span,
    index: ByteIndex,
    tokens: Vec<Token>,
    files_ref: &'s Files,
    handle: FileId,
    source: &'s str,
    chars: Peekable<Chars<'s>>
}

impl<'s> Lexer<'s> {
    /// Construct a new Lexer for an item in a Files database.
    pub fn new(files: &'s Files, handle: FileId) -> Self {
        let source = files.source(handle);
        let chars = source.chars().peekable();
        let mut s = Lexer {
            span: files.source_span(handle),
            index: ByteIndex::default(),
            tokens: Vec::new(),
            files_ref: files,
            handle,
            source,
            chars,
        };
        s.index = s.span.start();
        s
    }

    /// Check if this lexer is at the end of input.
    fn is_at_end(&self) -> bool {
        self.index >= self.span.end()
    }

    /// Call this lexer instance.
    pub fn call(mut self) -> Result<Vec<Token>, Diagnostic> {
        while !self.is_at_end() {
            match self.scan_token() {
                Ok(t) => self.tokens.push(t),
                Err(d) => return Err(d),
            }
        }
        return Ok(self.tokens);
    }

    fn scan_token(&mut self) -> Result<Token, Diagnostic> {
        let start = self.index;
        let c = self.advance();
        match c {
            '!' => {
                if self.matches('=') {
                    Self::ok_token(start, self.index, TokenType::NEQ)
                } else {
                    Self::ok_token(start, self.index, TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    Self::ok_token(start, self.index, TokenType::EQ)
                } else {
                    Self::ok_token(start, self.index, TokenType::Equals)
                }
            }
            '#' => {
                if self.matches('*') { // multiline comment

                }

            }
            _ => unimplemented!()
        }
    }

    // This just makes it easier to return Ok(Token) values
    fn ok_token(start: ByteIndex, end: ByteIndex, ty: TokenType) -> Result<Token, Diagnostic> {
        Ok(Token::new(start, end, ty))
    }

    /// Peek at the next character. returns null character if there is no next char.
    fn peek(&mut self) -> char {
        self.chars.peek().map(|c|*c).unwrap_or('\0')
    }

    /// Conditionally advances if (and only if) the next character is equal to ch.
    fn matches(&mut self, ch: char) -> bool {
        if self.peek() == ch {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Advance the lexer, returning the next character.
    fn advance(&mut self) -> char {
        let c = self.chars.next();
        if c.is_some() {
            let ch = c.unwrap();
            self.index += ByteOffset(ch.len_utf8() as i64);
            ch
        } else {
            self.index = self.span.end();
            '\0'
        }
    }
}