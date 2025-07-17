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

mod decl;
pub mod error;
mod identifier;
mod literal;
mod path;
mod ty;

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

        // SAFETY: It's certain that if this function reaches this line, this access is infallible.
        Some(unsafe { self.lookahead.get(k).unwrap_unchecked() })
    }

    /// Peek at the next token from the [Lexer] (cached in the lookahead queue if peeked before).
    pub fn peek(&mut self) -> Option<&Token> {
        self.lookahead(0)
    }
    
    /// Peek the [Fragment] of the next [Token].
    pub fn peek_fragment(&mut self) -> Option<&Fragment> {
        self.peek().map(|token| &token.fragment)
    }

    /// Peek the [TokenTy] of the next [Token].
    pub fn peek_variant(&mut self) -> Option<TokenTy> {
        self.peek().map(|token| token.variant)
    }

    /// Similar to [Parser::lookahead] but instead returns a slice of `n` [Token]s, starting with the next [Token].
    ///
    /// Returns [None] if `n` is greater than the number of remaining [Token]s for this [Parser].
    pub fn lookahead_window(&mut self, n: usize) -> Option<&[Token]> {
        // Pull tokens from lexer
        while self.lookahead.len() < n {
            self.lookahead.push_back(self.lexer.next_token()?);
        }

        // Use make contiguous here to get a unified/single slice.
        Some(&self.lookahead.make_contiguous()[..n])
    }

    /// Peek the next token that's not whitespace.
    pub fn peek_next_not_whitespace(&mut self) -> Option<&Token> {
        // There's no way to do this in safe rust, despite the memory accesses being fine,
        // so we do it unsafely here 
        
        for i in 0.. {
            let peek = self.lookahead(i)?;

            if peek.variant != TokenTy::Whitespace {
                // This bit prevents the rust compiler from thinking we're breaking 
                // lifetime/aliasing rules by mutating the internal state in the next 
                // iteration of the loop while still holding a reference to the peeked token.
                unsafe {
                    let const_ref = &raw const *peek;
                    let upcast = &*const_ref;
                    return Some(upcast);
                }
            }
        }

        // Safety: For large enough values of `i`, self.lookahead eventually has to return `None`
        unsafe { std::hint::unreachable_unchecked() }
    }

    /// Get the number of remaining bytes on this parser. This is potentially useful for checking
    /// if a parser has advanced between two calls (or checking if a parser has reached end of input).
    pub fn bytes_remaining(&self) -> usize {
        let bytes_remaining_in_lookahead_buffer = self
            .lookahead
            .iter()
            .map(|t| t.fragment.len())
            .sum::<usize>();

        let bytes_remaining_in_lexer = self.lexer.bytes_remaining();

        bytes_remaining_in_lexer + bytes_remaining_in_lookahead_buffer
    }

    /// Get the next [Token] from this [Parser]. This may be a token that's already been peeked.
    ///
    /// Skips any non-document comments encountered via the lexer implementation.
    ///
    /// Return an error if a [Token] with [TokenTy::Unknown] is encountered.
    pub fn next_token(&mut self) -> Result<Option<Token>, ParserError> {
        let token = self
            .lookahead
            .pop_front()
            .or_else(|| self.lexer.next_token());

        // Check for unknown tokens, which should always convert to an error.
        if let Some(ref t) = token && t.variant == TokenTy::Unknown {
            // Clone here is avoidable but this code path is super unlikely and
            // probably optimized heavily. 
            Err(ParserErrorKind::EncounteredUnknownToken.at(t.fragment.clone()))
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

    /// Peek the [Fragment] of the next [Token] and clone it or return a clone of the
    /// remainder [Fragment] of the internal [Lexer]
    /// (which will be empty, since there wasn't a [Token] to peek).
    ///
    /// This is likely only useful for error reporting -- a clone of a potentially empty fragment is
    /// rarely ever useful otherwise.
    pub fn peek_fragment_or_rest_cloned(&mut self) -> Fragment {
        match self.peek() {
            Some(Token { fragment, .. }) => fragment.clone(),
            None => {
                let rest = self.lexer.remaining.clone();

                // Assert that we're making the right assumptions about the remaining fragment.
                // These are (unidiomatically) done using debug_assert -- perhaps that changes eventually
                // however it should be fine for now, since this can only produce logic bugs (never memory or
                // concurrency bugs).
                debug_assert!(rest.is_valid());
                debug_assert!(rest.is_empty());
                debug_assert!(rest.is_empty_at_end_of_source());

                rest
            }
        }
    }

    /// Get the next [Token] from this parser if its [Token::variant] is the given `token_ty`.
    pub fn next_if_is(&mut self, token_ty: TokenTy) -> Option<Token> {
        // Peeking successfully first means that the lookahead vec will never be empty here.
        (self.peek_variant()? == token_ty)
            // SAFETY: We just peeked a token to check its variant so this unwrap is always ok.
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
            .zip(seq)
            .all(|(token, matches)| token.variant == *matches)
    }

    /// Consume & remove all whitespace tokens from the front of the parser.
    pub fn consume_optional_whitespace(&mut self) {
        // Iterate until the next token is not whitespace.
        while self.next_if_is(TokenTy::Whitespace).is_some() {}
    }

    /// Require a whitespace from the [Parser]. Do not advance if the next [Token] is not a whitespace.
    pub fn consume_at_least_one_whitespace(&mut self) -> Result<(), ParserError> {
        if self.next_if_is(TokenTy::Whitespace).is_some() {
            self.consume_optional_whitespace();
            Ok(())
        } else {
            Err(ParserErrorKind::ExpectedWhitespace.at(self.peek_fragment_or_rest_cloned()))
        }
    }
}
