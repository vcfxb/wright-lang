//! This parser module is responsible for turning the stream of [Token]s from the [Lexer] into a tree of [AST] nodes.
//!
//! [AST]: crate::ast
//! [Token]: crate::lexer::token::Token

use error::{ParserError, ParserErrorKind};

use super::lexer::Lexer;
use crate::{
    lexer::token::{Token, TokenTy},
    source_tracking::fragment::Fragment,
};
use std::collections::VecDeque;

pub mod error;
mod identifier;
mod literal;
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

    /// Get the next [Token] from this [Parser]. This may be a token that's already been peeked.
    /// Return an error if a [Token] with [TokenTy::Unknown] is encountered.
    pub fn next_token(&mut self) -> Result<Option<Token>, ParserError> {
        let token = self
            .lookahead
            .pop_front()
            .or_else(|| self.lexer.next_token());

        // Check for unknown tokens, which should always convert to an error.
        if let Some(Token {
            variant: TokenTy::Unknown,
            fragment,
        }) = token
        {
            Err(ParserError {
                kind: ParserErrorKind::EncounteredUnknownToken,
                location: fragment,
                help: None,
            })
        } else {
            Ok(token)
        }
    }

    /// Advance this [Parser] by `n` [Token]s. If this [Parser] runs out of [Token]s, panic.
    ///
    /// Panics
    /// - If `n` is greater than the number of remaining tokens.
    pub fn advance(&mut self, n: usize) {
        // Add tokens to the lookahead buffer until we have enough to split off.
        while self.lookahead.len() < n {
            let token = self
                .lexer
                .next_token()
                .expect("advance: `n` <= number of remaining tokens");

            self.lookahead.push_back(token);
        }

        // Split them off.
        self.lookahead = self.lookahead.split_off(n);
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

    /// Similar to [Parser::lookahead] but instead returns a slice of `n` [Token]s, starting with the next [Token].
    ///
    /// Returns [None] if `n` is greater than the number of remaining [Token]s for this [Parser].
    pub fn lookahead_window(&mut self, n: usize) -> Option<&[Token]> {
        while self.lookahead.len() < n {
            self.lookahead.push_back(self.lexer.next_token()?);
        }

        // Use make contiguous here to get a unified/single slice.
        Some(&self.lookahead.make_contiguous()[..n])
    }

    /// Get the next [Token] from this parser if its [Token::variant] is the given `token_ty`.
    pub fn next_if_is(&mut self, token_ty: TokenTy) -> Option<Token> {
        // Peeking successfully first means that the lookahead vec will never be empty here.
        (self.peek()?.variant == token_ty)
            // SAFETY: We just peeked a token to check its variant so this unwrap is alway ok.
            .then(|| unsafe { self.lookahead.pop_front().unwrap_unchecked() })
    }

    /// Peek at the next [Token]s of this [Parser] and determine if the [Token::variant]s match this
    /// sequence of [TokenTy]s.
    pub fn matches(&mut self, seq: &[TokenTy]) -> bool {
        // Use the rare let-else to ensure there are at minimum, the given number of tokens remaining.
        let Some(lookahead_window) = self.lookahead_window(seq.len()) else {
            return false;
        };

        // Use a zipped iterator to compare all the token variants.
        lookahead_window
            .iter()
            .zip(seq.iter())
            .all(|(token, matches)| token.variant == *matches)
    }

    /// Peek at the next [Token], remove it if it's a [TokenTy::Whitespace].
    pub fn ignore_whitespace(&mut self) {
        self.next_if_is(TokenTy::Whitespace);
    }
}
