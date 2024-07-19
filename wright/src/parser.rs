//! This parser module is responsible for turning the stream of [Token]s from the [Lexer] into a tree of [AST] nodes.
//!
//! [AST]: crate::ast
//! [Token]: crate::lexer::token::Token

use error::ParserError;
use std::collections::VecDeque;

use super::lexer::Lexer;
use crate::{
    lexer::token::{Token, TokenTy},
    source_tracking::fragment::Fragment,
};

pub mod error;
mod identifier;
mod path;
pub mod whitespace;

/// The [Parser] struct wraps a [Lexer] and adds lookahead and functions that are useful for parsing.
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    lookahead: VecDeque<Token>,
}

impl Parser {
    /// Construct a new parser around a given [Lexer].
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            lookahead: VecDeque::new(),
        }
    }

    /// Get the next [Token] from this [Parser]. This may be a clone of a token that's already been peeked.
    pub fn next_token(&mut self) -> Option<Token> {
        self.lookahead
            .pop_front()
            .or_else(|| self.lexer.next_token())
    }

    /// Peek at the next token from the [Lexer] (cached in the lookahead queue if peeked before).
    pub fn peek(&mut self) -> Option<&Token> {
        if self.lookahead.is_empty() {
            self.lookahead.push_back(self.lexer.next_token()?);
        }

        self.lookahead.front()
    }

    /// Peek the [Fragment] of the next [Token].
    pub fn peek_fragment(&mut self) -> Option<&Fragment> {
        self.peek().map(|token| &token.fragment)
    }

    /// Get the [Lexer] that's wrapped.
    pub fn lexer(&self) -> &Lexer {
        &self.lexer
    }

    /// Lookahead `k` [Token]s.
    ///
    /// If `k == 0` then this is effectively peeking at the next [Token] from the wrapped [Lexer].
    pub fn lookahead(&mut self, k: usize) -> Option<&Token> {
        while self.lookahead.len() <= k {
            self.lookahead.push_back(self.lexer.next_token()?);
        }

        self.lookahead.get(k)
    }

    /// Get the next [Token] from this parser if its [Token::variant] is the given `token_ty`.
    pub fn next_if_is(&mut self, token_ty: TokenTy) -> Option<Token> {
        // Peeking successfully first means that the lookahead vec will never be empty here.
        (self.peek()?.variant == token_ty)
            .then(|| unsafe { self.lookahead.pop_front().unwrap_unchecked() })
    }

    /// Peek at the next [Token], remove it if it's a [TokenTy::Whitespace].
    pub fn ignore_whitespace(&mut self) {
        self.next_if_is(TokenTy::Whitespace);
    }
}

/// Trait implemented by all AST nodes that can be parsed.
pub trait Parse: Sized {
    /// Attempt to parse a tree node of this type from a given [Parser].
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>;
}
