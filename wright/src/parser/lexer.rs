//! First pass lexer that gets run on the source code and returns a series of tokens with their associated [Fragment]s.
//!
//! Note that this will strip out comments and whitespace, returning only fragments that match one of the paterns
//! defined for tokens.

use self::comments::{try_match_block_comment, try_match_single_line_comment};
use self::integer_literal::try_consume_integer_literal;
use self::quoted::try_consume_quoted_literal;

use super::fragment::Fragment;
use std::iter::FusedIterator;
use std::ptr;
use std::str::Chars;
use token::{Token, TokenTy};

pub mod comments;
pub mod identifier;
pub mod integer_literal;
pub mod quoted;
pub mod token;
pub mod trivial;

/// The lexical analyser for wright. This produces a series of tokens that make up the larger elements of the language.
#[derive(Debug, Clone, Copy)]
pub struct Lexer<'src> {
    /// The remaining source code that has not been processed and returned as a token from the iterator yet.
    pub remaining: Fragment<'src>,
}

impl<'src> Lexer<'src> {
    /// Get the number of bytes remaining that we need to transform into tokens.
    pub const fn bytes_remaining(&self) -> usize {
        self.remaining.len()
    }

    /// Construct a new lexer over a given reference to a source string.
    pub const fn new(source: &'src str) -> Self {
        Lexer {
            remaining: Fragment { inner: source },
        }
    }

    /// Make a token by splitting a given number of bytes off of the `self.remaining` fragment
    /// and labeling them with the given kind.
    ///
    /// # Panics:
    /// - Panics if the number of bytes lands out of bounds or in the middle of a character.
    fn split_token(&mut self, bytes: usize, kind: TokenTy) -> Token<'src> {
        let (token_fragment, new_remaining_fragment) = self.remaining.split_at(bytes);
        self.remaining = new_remaining_fragment;

        Token {
            variant: kind,
            fragment: token_fragment,
        }
    }

    /// Unsafe version of [Lexer::split_token].
    ///
    /// # Safety
    /// - This function matches the safety guarantees of [Fragment::split_at_unchecked].
    unsafe fn split_token_unchecked(&mut self, bytes: usize, kind: TokenTy) -> Token<'src> {
        let (token_fragment, new_remaining_fragment) = self.remaining.split_at_unchecked(bytes);
        self.remaining = new_remaining_fragment;

        Token {
            variant: kind,
            fragment: token_fragment,
        }
    }

    /// "Fork" this lexer, creating a new [`Lexer`] at the same position as this one that can be used for
    /// failable parsing. This can be compared to the original lexer it was forked from using [Lexer::offset_from].
    pub const fn fork(&self) -> Self {
        *self
    }

    /// Get the number of bytes between the origin's [remaining](Lexer::remaining) and
    /// this [Lexer]'s [remaining](Lexer::remaining) using [`Fragment::offset_from`].
    ///
    /// # Panics
    /// - This function panics under the same conditions as [`Fragment::offset_from`].
    /// - Generally the best way to avoid panics is to only call this function on
    ///     [Lexer]s created using [Lexer::fork] on the `origin` lexer.
    #[inline]
    pub fn offset_from(&self, origin: &Self) -> usize {
        self.remaining.offset_from(&origin.remaining)
    }

    /// Remove and ignore any whitespace at the start of the [Lexer::remaining] [Fragment].
    pub fn ignore_whitespace(&mut self) {
        // Get a reference to the slice of the string past any whitespace at the start.
        let without_whitespace: &str = self.remaining.inner.trim_start();

        // If the references aren't equal, update the remaining fragment.
        if !ptr::eq(without_whitespace, self.remaining.inner) {
            self.remaining.inner = without_whitespace;
        }
    }

    /// Check if a pattern matches at the start of the [Lexer::remaining] [Fragment].
    pub fn matches(&self, pattern: &str) -> bool {
        self.remaining.inner.starts_with(pattern)
    }

    /// If the remaining fragment starts with the given `pattern`, strip it from the remaining fragment and return
    /// true. Otherwise return false.
    fn consume(&mut self, pattern: &str) -> bool {
        if let Some(stripped) = self.remaining.inner.strip_prefix(pattern) {
            self.remaining.inner = stripped;
            true
        } else {
            false
        }
    }

    /// Remove a character from the start of the `remaining` [`Fragment`], return the character
    /// consumed if there was a character available to consume.
    fn consume_any(&mut self) -> Option<char> {
        // Make a character iterator.
        let mut chars: Chars = self.remaining.chars();

        if let Some(c) = chars.next() {
            // Consumed a char, update the remaining fragment of this lexer.
            let char_bytes: usize = c.len_utf8();
            // SAFETY: we know that this is not on a char boundary and does not exceed the length of the slice,
            // since we just pulled it from a `Chars` iterator.
            self.remaining.inner = unsafe { self.remaining.inner.get_unchecked(char_bytes..) };
            // Return the character.
            Some(c)
        } else {
            // No characters available, return nothing.
            None
        }
    }

    /// Advance this lexer by the specified number of bytes.
    ///
    /// # Panics
    /// - If the lexer is not on a unicode character boundary after advancing.
    /// - If the number of bytes is greater than the length of the [remaining](Lexer::remaining) fragment.
    fn advance(&mut self, bytes: usize) {
        self.remaining.inner = &self.remaining.inner[bytes..];
    }

    /// Unsafe version of [Lexer::advance].
    /// Advances this lexer by the specified number of bytes.
    ///
    /// # Safety
    /// - This lexer will be left in an invalid/undefined state if the number of bytes is greater than the length
    ///     of the [Lexer::remaining] fragment.
    /// - This lexer will be left in an invalid/undefined state if after advancing, the next byte in the
    ///     [Lexer::remaining] fragment is not the start of a unicode code point.
    unsafe fn advance_unchecked(&mut self, bytes: usize) {
        self.remaining.inner = self.remaining.inner.get_unchecked(bytes..);
    }

    /// Get the next token from the lexer.
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        // Ignore any whitespace at the start of the lexer.
        self.ignore_whitespace();

        // If the remaining input is empty, there is no token.
        if self.remaining.is_empty() {
            return None;
        }

        // Attempt to parse a single line comment and then attempt a multi-line comment.
        for comment_match_fn in [try_match_single_line_comment, try_match_block_comment] {
            // Attempt to parse a comment using the given match function. Return it if it's documentation or unterminated.
            // Get a new token and return that if there was a comment and it was ignored successfully.
            match (comment_match_fn)(self) {
                // A comment was parsed, consume and return it.
                (bytes, Some(comment_variant)) => {
                    // Split the token.
                    let token: Token = self.split_token(bytes, comment_variant);
                    // Return it.
                    return Some(token);
                }

                // There was a comment, advance the lexer and ignore it. Re-start this function.
                (bytes @ 1.., None) => {
                    self.advance(bytes);
                    return self.next_token();
                }

                // There was no comment, keep trying to match other tokens.
                (0, None) => {}
            }
        }

        // Handle a trivial token if there is one.
        if let Some(token) = trivial::try_consume_trivial_token(self) {
            return Some(token);
        }

        // Next attempt to match a keyword or identifier.
        if let Some(token) = identifier::try_consume_keyword_or_identifier(self) {
            return Some(token);
        }

        // Next attempt to parse an integer literal.
        if let Some(integer_lit) = try_consume_integer_literal(self) {
            return Some(integer_lit);
        }

        // Next attempt to parse a quoted literal.
        if let Some(quoted_lit) = try_consume_quoted_literal(self) {
            return Some(quoted_lit);
        }

        // If we haven't matched at this point, produce a token marked as "Unknown".
        // The unsafe is fine -- we know from above that there are remaining characters.
        let unknown_char = unsafe { self.remaining.chars().next().unwrap_unchecked() };
        return Some(self.split_token(unknown_char.len_utf8(), TokenTy::Unknown));
    }
}

/// Lexers can be considered token iterators.
impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Lexers should not return multiple tokens for a single byte.
        (0, Some(self.bytes_remaining()))
    }
}

// Lexers are fused -- they cannot generate tokens infinitely.
impl<'src> FusedIterator for Lexer<'src> {}
