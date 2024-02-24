//! First pass lexer that gets run on the source code and returns a series of tokens with their associated [Fragment]s.
//!
//! Note that this will strip out comments and whitespace, returning only fragments that match one of the paterns
//! defined for tokens.

use super::fragment::Fragment;
use derive_more::Display;
use std::iter::FusedIterator;
use std::str::Chars;
use std::{iter::Peekable, ptr};
use unicode_ident::{is_xid_continue, is_xid_start};

/// Constant table of single character tokens and the characters that match them.
pub const SINGLE_CHAR_TOKENS: &[(char, TokenTy)] = &[
    ('(', TokenTy::LeftParen),
    (')', TokenTy::RightParen),
    ('[', TokenTy::LeftBracket),
    (']', TokenTy::RightBracket),
    ('{', TokenTy::LeftCurly),
    ('}', TokenTy::RightCurly),
    ('@', TokenTy::At),
    (';', TokenTy::Semi),
    ('?', TokenTy::Question),
    (',', TokenTy::Comma),
    ('#', TokenTy::Hash),
    ('$', TokenTy::Dollar),
];

/// Tokens that can be either a single character or upgraded with an
/// equals sign.
pub const POSSIBLE_EQ_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy)] = &[
    ('!', TokenTy::Bang, TokenTy::BangEq),
    ('%', TokenTy::Mod, TokenTy::ModEq),
    ('^', TokenTy::Xor, TokenTy::XorEq),
    ('*', TokenTy::Star, TokenTy::StarEq),
    ('+', TokenTy::Plus, TokenTy::PlusEq),
    ('/', TokenTy::Div, TokenTy::DivEq),
];

/// Characters that can produce different tokens when followed by an equals sign or themselves.
pub const POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy, TokenTy)] = &[
    ('&', TokenTy::And, TokenTy::AndEq, TokenTy::AndAnd),
    ('|', TokenTy::Or, TokenTy::OrEq, TokenTy::OrOr),
    ('<', TokenTy::Lt, TokenTy::LtEq, TokenTy::LtLt),
    ('>', TokenTy::Gt, TokenTy::GtEq, TokenTy::GtGt),
    (':', TokenTy::Colon, TokenTy::ColonEq, TokenTy::ColonColon),
];

/// Characters that can produce different tokens when followed by an equals sign or
/// a `>` for arrows.
pub const POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy, TokenTy)] = &[
    ('-', TokenTy::Minus, TokenTy::MinusEq, TokenTy::SingleArrow),
    ('=', TokenTy::Eq, TokenTy::EqEq, TokenTy::DoubleArrow),
];

/// The number of rows of the generated prefix table.
pub const PREFIX_TABLE_ROWS: usize = {
    SINGLE_CHAR_TOKENS.len()
        + 2 * POSSIBLE_EQ_UPGRADE_TOKENS.len()
        + 3 * POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS.len()
        + 3 * POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS.len()
};

/// A relationship between a prefix and the token that should be generated when that prefix matches.
#[derive(Copy, Clone, Debug)]
pub struct PrefixToToken {
    /// An array of two chars. In single char tokens, the second one should be a null character (`'\0'`).
    /// the char_length field will be used to slice this buffer to get the actual prefix.
    pub char_buffer: [char; 2],
    /// The byte length of this prefix and all generated tokens by this prefix.
    pub byte_len: usize,
    /// The kind of [Token] generated when this prefix matches.
    pub kind: TokenTy,
}

impl PrefixToToken {
    /// Convenience function to construct a [`PrefixToToken`] by calculating the length of both chars
    /// (and ignoring the second one if it's null).
    pub const fn new(chars: [char; 2], kind: TokenTy) -> Self {
        PrefixToToken {
            char_buffer: chars,

            byte_len: if chars[1] == '\0' {
                chars[0].len_utf8()
            } else {
                chars[0].len_utf8() + chars[1].len_utf8()
            },

            kind,
        }
    }
}

/// A full table generated at compile time using all the token tables 
/// ([SINGLE_CHAR_TOKENS], [POSSIBLE_EQ_UPGRADE_TOKENS], [POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS], 
/// [POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS]). 
/// 
/// This table can be iterated on in order when trying to match a token at the start of a fragment of source code. 
#[rustfmt::skip]
pub const PREFIX_TABLE: [PrefixToToken; PREFIX_TABLE_ROWS] = {
    // Make a mutable table with dummy values to replace with actual values. 
    let mut table: [PrefixToToken; PREFIX_TABLE_ROWS] = 
        [PrefixToToken { char_buffer: ['\0'; 2], byte_len: 0, kind: TokenTy::Unknown }; PREFIX_TABLE_ROWS];

    // Current index to insert into table at.
    let mut write_index: usize = 0;

    // Index used for reading from various tables. 
    let mut read_index: usize = 0;

    // Iterate first over all the single char tokens. 
    while read_index < SINGLE_CHAR_TOKENS.len() {
        // Get row from source table.
        let (c, token_kind) = SINGLE_CHAR_TOKENS[read_index];

        // Put row in destination table.
        table[write_index] = PrefixToToken::new([c, '\0'], token_kind);

        // Increment both indices. 
        read_index += 1;
        write_index += 1;
    }

    // Then do all the tokens that can be upgraded with an equals sign. 
    // Add the row for the token with the equals sign first so that when we iterate over this table in order,
    // the version without the equals sign does not match prematurely. 
    read_index = 0;
    while read_index < POSSIBLE_EQ_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq) = POSSIBLE_EQ_UPGRADE_TOKENS[read_index];

        table[write_index]     = PrefixToToken::new([c, '='], with_eq);
        table[write_index + 1] = PrefixToToken::new([c, '\0'], without_eq);

        read_index += 1;
        write_index += 2;
    }

    // Do the same for the tokens that can be upgraded with an equals sign or doubled. 
    read_index = 0;
    while read_index < POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq, doubled) = POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS[read_index];

        table[write_index]     = PrefixToToken::new([c, c], doubled);
        table[write_index + 1] = PrefixToToken::new([c, '='], with_eq);
        table[write_index + 2] = PrefixToToken::new([c, '\0'], without_eq);

        read_index += 1;
        write_index += 3;
    }

    // Do the same for possible eq or arrow upgrades.
    read_index = 0;
    while read_index < POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq, with_arrow) = POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS[read_index];

        table[write_index]     = PrefixToToken::new([c, '>'], with_arrow);
        table[write_index + 1] = PrefixToToken::new([c, '='], with_eq);
        table[write_index + 2] = PrefixToToken::new([c, '\0'], without_eq);

        read_index += 1;
        write_index += 3;
    }

    table
};

/// The pattern that begins any single line comments (including doc comments).
pub const SINGLE_LINE_COMMENT_PREFIX: &str = "//";

/// The pattern that starts any multi-line comments (including doc comments).
pub const MULTI_LINE_COMMENT_START: &str = "/*";

/// The pattern that ends any multi-line comments (including doc comments).
pub const MULTI_LINE_COMMENT_END: &str = "*/";

/// The lexical analyser for wright. This produces a series of tokens that make up the larger elements of the language.
#[derive(Debug, Clone, Copy)]
pub struct Lexer<'src> {
    /// The remaining source code that has not been processed and returned as a token from the iterator yet.
    pub remaining: Fragment<'src>,
}

/// A token in wright source code.
#[derive(Debug, Display)]
#[display(fmt = "\"{}\" ({:?})", "fragment.inner", variant)]
pub struct Token<'src> {
    /// What type of token this is.
    pub variant: TokenTy,
    /// The matching fragment of source code -- this contains the location and length data for the token.
    pub fragment: Fragment<'src>,
}

/// The different types of tokens in wright source.
#[rustfmt::skip] // Turn off auto reformat. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenTy {
    LeftCurly, RightCurly,
    LeftBracket, RightBracket,
    LeftParen, RightParen,

    Plus, PlusEq,
    Star, StarEq,
    Div, DivEq,
    Xor, XorEq,
    Mod, ModEq,
    Bang, BangEq,

    Minus, MinusEq, SingleArrow,
    Eq, EqEq, DoubleArrow,

    Lt, LtEq, LtLt,
    Gt, GtEq, GtGt,
    And, AndEq, AndAnd,
    Or, OrEq, OrOr,
    Colon, ColonEq, ColonColon,

    At,
    Tilde,
    Semi,
    Dot,
    Comma,
    Hash,
    Question,
    Dollar,
    
    // Not in the same group as the other ones there since it can be used at the start of identifiers.
    Underscore,

    Identifier,

    OuterDocComment, OuterBlockDocComment,
    InnerDocComment, InnerBlockDocComment,
    
    /// Indicates a block style comment without termination. 
    UnterminatedBlockComment,


    KwRecord,
    KwType,
    KwEnum,
    KwUnion,
    KwFunc,
    KwRepr,
    KwImpl,
    KwConstraint,
    KwReferences,
    KwTrait,
    KwUse,
    KwAs,
    KwConst,
    KwMod,
    KwIf,
    KwElse,
    KwFor,
    KwIn,
    KwWhile,
    KwTrue,
    KwFalse,
    KwLoop,
    KwWhere,

    IntegerLiteral,

    /// Unknown character in lexer fragment. 
    Unknown
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

    /// Try to match a fragment recognized to be an identifier or keyword to
    /// a keyword or return [TokenTy::Identifier].
    fn identifier_or_keyword(fragment: Fragment<'src>) -> TokenTy {
        use TokenTy::*;

        match fragment.inner {
            "record" => KwRecord,
            "type" => KwType,
            "enum" => KwEnum,
            "union" => KwUnion,
            "func" => KwFunc,
            "repr" => KwRepr,
            "impl" => KwImpl,
            "constraint" => KwConstraint,
            "references" => KwReferences,
            "trait" => KwTrait,
            "const" => KwConst,
            "where" => KwWhere,

            "use" => KwUse,
            "as" => KwAs,
            "mod" => KwMod,

            "if" => KwIf,
            "else" => KwElse,

            "for" => KwFor,
            "in" => KwIn,
            "while" => KwWhile,
            "loop" => KwLoop,

            "true" => KwTrue,
            "false" => KwFalse,

            "_" => Underscore,

            _ => Identifier,
        }
    }

    /// Make a token by splitting a given number of bytes off of the `self.remaining` fragment
    /// and labeling them with the given kind.
    fn split_token(&mut self, bytes: usize, kind: TokenTy) -> Token<'src> {
        let (token_fragment, new_remaining_fragment) = self.remaining.split(bytes);
        self.remaining = new_remaining_fragment;

        Token {
            variant: kind,
            fragment: token_fragment,
        }
    }

    /// "Fork" this lexer, creating a new [`Lexer`] at the same position as this one that can be used for
    /// failable parsing. This can be compared to the original lexer it was forked from using [Fragment::offset_from]
    /// on the underlying `remaining` fragments.
    fn fork(&self) -> Self {
        *self
    }

    /// Remove and ignore any whitespace at the start of the remaining fragment.
    fn ignore_whitespace(&mut self) {
        // Get a reference to the slice of the string past any whitespace at the start.
        let without_whitespace: &str = self.remaining.inner.trim_start();

        // If the references aren't equal, update the remaining fragment.
        if !ptr::eq(without_whitespace, self.remaining.inner) {
            self.remaining.inner = without_whitespace;
        }
    }

    /// Check if a pattern matches at the start of the remaining fragment, and if so return the number of bytes.
    fn matches(&self, pattern: &str) -> bool {
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

    /// Attempt to read/handle a single line comment from the start of the
    /// remaining fragment. If there's a doc-style single line comment, return a [`Token`],
    /// otherwise return [`None`].
    ///
    /// Generally I'm trying to follow the [rust comment spec] here.
    ///
    /// [rust comment spec]: https://doc.rust-lang.org/reference/comments.html
    fn handle_single_line_comment(&mut self) -> Option<Token<'src>> {
        // Fork the lexer to attempt to consume a single line comment.
        let mut fork: Self = self.fork();

        // Try to consume the single line comment prefix from the fork.
        if fork.consume(SINGLE_LINE_COMMENT_PREFIX) {
            // We consumed it successfully, read through a newline or the end of the forked lexer if we get there.

            // First determine if this is a doc comment of some kind.
            let is_inner_doc: bool = fork.matches("/") && !fork.matches("//");
            let is_outer_doc: bool = fork.matches("!");

            // The consume until a newline, carraige return, or the end of the source fragment.
            while !fork.remaining.is_empty() && !fork.matches("\r") && !fork.matches("\n") {
                fork.consume_any();
            }

            // Determine the kind of token to produce (if any).
            let variant: Option<TokenTy> = match (is_inner_doc, is_outer_doc) {
                (true, false) => Some(TokenTy::InnerDocComment),
                (false, true) => Some(TokenTy::OuterDocComment),
                (false, false) => None,
                (true, true) => unreachable!("Lexer should not match multiple comment types at once."),
            };

            // Map the variant to a token to return.
            let token: Option<Token> = variant.map(|kind| {
                // Get the number of bytes we have consumed using `offset_from`.
                let bytes_consumed: usize = fork.remaining.offset_from(&self.remaining);
                // Split this token from `self` rather than `fork` since self is still in an unmodified position.
                self.split_token(bytes_consumed, kind)
            });

            // Update this lexer to match the state of the forked lexer.
            *self = fork;
            // Consume any outstanding whitespace.
            self.ignore_whitespace();
            // Return any token produced.
            return token;
        }

        // If there was no comment prefix, there is no comment immediately available.
        None
    }

    /// Attempt to read/consume a multi-line comment from the start of the `remaining` fragment. 
    fn handle_multi_line_comment(&mut self) -> Option<Token<'src>> {
        // Handle corner cases here so we don't have to below. 
        // These are both considered empty non-documenting comments.
        if self.consume("/***/") {
            return None;
        }

        if self.consume("/**/") {
            return None;
        }

        // Make a fork of the lexer to avoid modifying this lexer if we fail to parse. 
        let mut fork: Self = self.fork();

        // Try to parse the start of a multi-line comment. 
        if fork.consume(MULTI_LINE_COMMENT_START) {
            // Check if this is a doc comment. 
            let is_outer_doc: bool = fork.matches("!");
            // Use this to indicate that more than one following asterix is not a doc comment. 
            let is_inner_doc: bool = fork.matches("*") && !fork.matches("**");

            // Consume until we see the end of the doc comment. If we run out of characters, consider the 
            // comment unterminated. 
            while !fork.matches(MULTI_LINE_COMMENT_END) {
                // Handle nested comments here: 
                if fork.matches(MULTI_LINE_COMMENT_START) { 
                    // Discard the output -- don't care about doc comments in other comments. 
                    fork.handle_multi_line_comment();
                    continue;
                }

                // Handle unterminated comments here.
                if fork.remaining.is_empty() {
                    // If we have not hit a "*/" before the end of the input, return an unterminated block comment. 
                    let bytes_consumed: usize = fork.remaining.offset_from(&self.remaining);
                    // Split the token and return it. 
                    return Some(self.split_token(bytes_consumed, TokenTy::UnterminatedBlockComment));
                }
                
                // If there's still input, and not a nested comment, consume it. 
                fork.consume_any();
            }

            // If we get here, the comment was terminated. Consume the terminating characters, and return. 
            // Use debug assert here to make sure that the comment is actually terminated. 
            debug_assert!(fork.consume(MULTI_LINE_COMMENT_END), "comment is actually terminated");

            // Determine the kind of token to produce (if any).
            let variant: Option<TokenTy> = match (is_inner_doc, is_outer_doc) {
                (true, false) => Some(TokenTy::InnerBlockDocComment),
                (false, true) => Some(TokenTy::OuterBlockDocComment),
                (false, false) => None,
                (true, true) => unreachable!("Lexer should not match multiple comment types at once."),
            };

            // Make the token to return. 
            let token: Option<Token> = variant.map(|kind| {
                // Get the number of bytes we have consumed using `offset_from`.
                let bytes_consumed: usize = fork.remaining.offset_from(&self.remaining);
                // Split this token from `self` rather than `fork` since self is still in an unmodified position.
                self.split_token(bytes_consumed, kind)
            });

            // Update this lexer to match the state of the fork.
            *self = fork;
            // Return token if there was one.
            return token;
        }

        // If the fork did not consume a multi-line comment start, return None and do 
        // not update this lexer. 
        None
    }

    /// Get the next token from the lexer.
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        // Ignore any whitespace at the start of the lexer.
        self.ignore_whitespace();

        // Grab a copy of the initial lexer to compare and check when progress has been made.
        let initial_lexer: Self = self.fork();

        // Attempt to parse a single line comment. Return it if it's documentation.
        // Rerun this function if there was a comment and it was ignored successfully.
        
        match self.handle_single_line_comment() {
            // There was a single line comment ignored or no single line comment.
            None => {
                // Check if the remaining fragment changed.
                if !self.remaining.ptr_eq(&initial_lexer.remaining) {
                    // If so, re-run this function.
                    return self.next_token();
                }

                // If the lexer was unchanged, then there was no comment -- keep trying to match tokens.
            }

            // If there was some token, return it.
            token => return token,
        }

        // If the remaining input is empty, there is no token.
        if self.remaining.is_empty() {
            return None;
        }

        // Try to handle a multi-line comment if there is one. 
        match self.handle_multi_line_comment() {
            // There was an ignored comment or no comment. 
            None => {
                // If the lexer was changed, restart this function. 
                if !self.remaining.ptr_eq(&initial_lexer.remaining) {
                    return self.next_token();
                }
            }

            // If there was a block style doc-comment, or an unterminated multi-line comment
            // return. 
            token => return token,
        }

        // To attempt to match a token from the prefix table, make a char iterator
        // and get two chars from it to test equality. None of the tokens start with a
        // null character so use that as a single of an unavailable char.
        {
            let mut char_iter: Chars = self.remaining.chars();
            let char_array: [char; 2] = [
                // Unchecked unwrap here since we know there's at least one char.
                unsafe { char_iter.next().unwrap_unchecked() },
                char_iter.next().unwrap_or('\0'),
            ];

            // Next iterate through the prefix table to try to get any tokens that are covered there.
            for prefix_meta in PREFIX_TABLE.iter() {
                // If it's a single char comparison, only compare the first chars.
                if prefix_meta.char_buffer[1] == '\0' && prefix_meta.char_buffer[0] == char_array[0]
                {
                    return Some(self.split_token(prefix_meta.byte_len, prefix_meta.kind));
                }

                // Otherwise compare the whole slices.
                if prefix_meta.char_buffer == char_array {
                    return Some(self.split_token(prefix_meta.byte_len, prefix_meta.kind));
                }
            }
        }

        // Next attempt to match a keyword or identifier.
        {
            let mut chars: Chars = self.remaining.chars();
            // The unsafe is fine here -- we've established that this lexer has bytes remaining.
            let next: char = unsafe { chars.next().unwrap_unchecked() };

            if is_xid_start(next) || next == '_' {
                let mut bytes_consumed: usize = next.len_utf8();

                // Take remaining chars and add to sum.
                bytes_consumed += chars
                    .take_while(|c| is_xid_continue(*c))
                    .map(char::len_utf8)
                    .sum::<usize>();

                // Split the number of bytes we consumed.
                let (ident_frag, new_remaining) = self.remaining.split(bytes_consumed);
                // Get the token kind to produce for this fragment.
                let variant = Lexer::identifier_or_keyword(ident_frag);
                // Update the lexers remaining fragment.
                self.remaining = new_remaining;
                // Return the identifier, keyword, or underscore.
                return Some(Token {
                    variant,
                    fragment: ident_frag,
                });
            }
        }

        // Next attempt to parse a numerical literal.
        {
            let mut chars: Peekable<Chars> = self.remaining.chars().peekable();
            // The unsafe is fine here -- we've established that this lexer has bytes remaining.
            let next: char = unsafe { chars.next().unwrap_unchecked() };

            if next.is_ascii_digit() {
                // Accumulate the number of bytes consumed in the numeric literal.
                let mut acc: usize = 1;
                // Track the radix
                let mut radix: u32 = 10;

                // Change the radix if necessary
                if next == '0' {
                    if let Some(prefix) = chars.next_if(|x| ['x', 'o', 'b', 'X', 'B'].contains(x)) {
                        // All the possible prefix chars are 1 byte ascii characters.
                        acc += 1;

                        radix = match prefix {
                            'x' | 'X' => 16,
                            'b' | 'B' => 2,
                            'o' => 8,
                            _ => unreachable!("the prefix byte is checked above"),
                        };
                    }
                }

                // Add the rest of the integer literal.
                acc += chars
                    .take_while(|c| c.is_digit(radix) || *c == '_')
                    .map(char::len_utf8)
                    .sum::<usize>();

                return Some(self.split_token(acc, TokenTy::IntegerLiteral));
            }
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
        // Lexers cannot return multiple tokens for a single byte.
        (0, Some(self.bytes_remaining()))
    }
}

// Lexers are fused -- they cannot generate tokens infinitely.
impl<'src> FusedIterator for Lexer<'src> {}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::PREFIX_TABLE;
    use crate::parser::lexer::TokenTy;

    #[test]
    #[ignore = "this test is just used for debugging the prefix table"]
    /// Run this with `cargo test manual_debug_prefix_table -- --nocapture --ignored`.
    fn manual_debug_prefix_table() {
        dbg!(PREFIX_TABLE);
    }

    #[test]
    fn plus_and_plus_eq_tokens() {
        let mut plus = Lexer::new("+");
        let mut plus_eq = Lexer::new("+=");

        let plus_token = plus.next_token().unwrap();
        let plus_eq_token = plus_eq.next_token().unwrap();

        assert_eq!(plus.bytes_remaining(), 0);
        assert_eq!(plus_eq.bytes_remaining(), 0);
        assert_eq!(plus_token.variant, TokenTy::Plus);
        assert_eq!(plus_eq_token.variant, TokenTy::PlusEq);
    }

    #[test]
    fn plus_one_token() {
        let mut plus_one = Lexer::new("+1");
        let plus_token = plus_one.next_token().unwrap();
        assert_eq!(plus_one.bytes_remaining(), 1);
        assert_eq!(plus_token.variant, TokenTy::Plus);
        assert_eq!(plus_token.fragment.len(), 1);
    }

    #[test]
    fn identifiers_and_keywords() {
        let mut lexer = Lexer::new("const TEST");

        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::KwConst);
        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::Identifier);
    }

    #[test]
    fn intger_literal() {
        let mut lexer = Lexer::new("123_456_789.");

        let token = lexer.next_token().unwrap();

        assert_eq!(token.fragment.inner, "123_456_789");
        assert_eq!(token.variant, TokenTy::IntegerLiteral);
    }

    #[test]
    fn ignored_single_line_comment() {
        let mut lexer = Lexer::new("// test comment ");
        assert!(lexer.next_token().is_none());
        assert_eq!(lexer.remaining.len(), 0);
    }
}
