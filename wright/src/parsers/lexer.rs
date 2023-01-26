//! The wright lexer. This module is responsible for lexical analysis and initial processing of source code.

use std::{iter::Peekable, str::Chars};

/// Token of Wright source code.
#[derive(Clone, Copy, Debug)]
pub struct Token {
    /// What type of token is it?
    pub variant: TokenTy,
    /// How many bytes of source code long is it? Note this doesn't necessarily mean how many characters long it is.
    pub length: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenTy {
    // Operators and parentheses
    LeftParen,      // (
    RightParen,     // )
    Bang,           // !
    BangEq,         // !=
    Tilde,          // ~
    At,             // @
    Mod,            // %
    ModEq,          // %=
    Xor,            // ^
    XorEq,          // ^=
    And,            // &
    AndEq,          // &=
    AndAnd,         // &&
    Or,             // |
    OrEq,           // |=
    OrOr,           // ||
    Star,           // *
    StarEq,         // *=
    Plus,           // +
    PlusEq,         // +=
    Minus,          // -
    MinusEq,        // -=
    Gt,             // >
    GtEq,           // >=
    Lt,             // <
    LtEq,           // <=
    Eq,             // =
    EqEq,           // ==
    Div,            // /
    DivEq,          // /=
    Semi,           // ;
    Colon,          // :
    Question,       // ?
    Dot,            // .
    Range,          // ..
    RangeInclusive, // ..=
    Comma,          // ,
    LeftSquare,     // [
    RightSquare,    // ]
    LeftBracket,    // {
    RightBracket,   // }

    /// Whitespace of any kind and length.
    Whitespace,

    /// Single line comment started with `#`. Optionally `## ` for documentation.
    SingleLineComment {
        is_doc: bool,
    },

    /// Multiline comment between `#*` and `*#`. Starts with `##*` for documentation.
    MultilineComment {
        is_doc: bool,
    },

    /// Integer literal. This is a literal integer in source code. May include underscores after the leading digit
    /// as visual seperators. May also include a prefix such as `0x` or `0o` for hex or octal.
    IntegerLit,

    /// A string literal in source code.
    StringLit,

    /// A character literal in source code.
    CharLit,

    /// A identifier in source code (such as a variable name). At this stage keywords (such as 'struct') are
    /// also considered identifiers.
    Identifier,

    /// Unknown character for the lexer.
    Unknown,

    /// End of input/file.
    End,
}

/// Lexical analyzer for wright code. This struct host functions that produce tokens from wright source.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Iterator over the indexed input characters tied to the lifetime of the source code.
    iterator: Peekable<Chars<'a>>,

    /// Lexer output.
    output: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Consume and return the next item from this object's iterator.
    fn next(&mut self) -> Option<char> {
        self.iterator.next()
    }

    /// Consume a character from the iterator if it is equal to the one passed to this function. Return the number of
    /// bytes consumed from the iterator.
    fn consume_if_eq(&mut self, c: char) -> usize {
        if let Some(_) = self.iterator.next_if(|next| *next == c) {
            c.len_utf8()
        } else {
            0
        }
    }

    /// Add a token on to the output vector.
    fn emit_token(&mut self, variant: TokenTy, bytes: usize) {
        self.output.push(Token {
            variant,
            length: bytes,
        });
    }

    /// Add a token to the output vector with a length of 1.
    fn emit_single_byte_token(&mut self, variant: TokenTy) {
        self.emit_token(variant, 1);
    }

    // Assignment versions of operators are very common (e.g. + and +=, - and -=).
    // This function will check for the equals sign and emit the correct token as necessary.
    // This assumes the first character has already been consumed from the iterator and is 1 byte.
    fn possible_eq_upgrade(&mut self, without: TokenTy, with: TokenTy) {
        if self.consume_if_eq('=') == 1 {
            self.emit_token(with, 2);
        } else {
            self.emit_single_byte_token(without);
        }
    }

    /// The & and | operators can be combined with a '=' for an assignment operator or can be doubled to be
    /// short-circuiting. This function checks for assumes that a single byte character has already been consumed from
    /// the iterator and checks to see if it's followed by either the supplied character `c` or the character '='. If it's followed
    /// by the supplied character then this function emits the `doubled` token. If it's followed by an equals sign, the `with_eq` token is emitted.
    /// Otherwise the `without` token is emitted.
    fn possible_eq_or_double(
        &mut self,
        c: char,
        without: TokenTy,
        with_eq: TokenTy,
        doubled: TokenTy,
    ) {
        if self.consume_if_eq(c) > 0 {
            self.emit_token(doubled, c.len_utf8() + 1)
        } else if self.consume_if_eq('=') > 0 {
            self.emit_token(with_eq, 2)
        } else {
            self.emit_single_byte_token(without)
        }
    }

    /// Read a source file and produce a series of tokens (aka lexemes) representing the source code for transformation
    /// into an AST. Return error instead of series of tokens if there is an unfinished sting or character literal.
    pub fn lex(source: &str) -> Vec<Token> {
        // Return no tokens if there is no source.
        if source.is_empty() {
            return Vec::new();
        }

        // Create lexer object to operate on.
        let mut lexer = Lexer {
            output: Vec::new(),
            iterator: source.chars().peekable(),
        };

        // Work our way through the iterator using a `while let` loop to destructructure the items as we work through and
        // make it slightly clearer that we mutate the iterator during the loop if we find the start of a string.
        while let Some(character) = lexer.next() {
            // Figure out what type of token to generate here. This may consume an aditional item from the iterator if possible.
            match character {
                // Single character tokens.
                '(' => lexer.emit_single_byte_token(TokenTy::LeftParen),
                ')' => lexer.emit_single_byte_token(TokenTy::RightParen),
                '[' => lexer.emit_single_byte_token(TokenTy::LeftSquare),
                ']' => lexer.emit_single_byte_token(TokenTy::RightSquare),
                '{' => lexer.emit_single_byte_token(TokenTy::LeftBracket),
                '}' => lexer.emit_single_byte_token(TokenTy::RightBracket),
                '@' => lexer.emit_single_byte_token(TokenTy::At),
                ':' => lexer.emit_single_byte_token(TokenTy::Colon),
                ';' => lexer.emit_single_byte_token(TokenTy::Semi),
                '?' => lexer.emit_single_byte_token(TokenTy::Question),
                ',' => lexer.emit_single_byte_token(TokenTy::Comma),
                '~' => lexer.emit_single_byte_token(TokenTy::Tilde),

                // Tokens that can possibly be followed by an equal sign.
                '!' => lexer.possible_eq_upgrade(TokenTy::Bang, TokenTy::BangEq),
                '%' => lexer.possible_eq_upgrade(TokenTy::Mod, TokenTy::ModEq),
                '^' => lexer.possible_eq_upgrade(TokenTy::Xor, TokenTy::XorEq),
                '*' => lexer.possible_eq_upgrade(TokenTy::Star, TokenTy::StarEq),
                '+' => lexer.possible_eq_upgrade(TokenTy::Plus, TokenTy::PlusEq),
                '-' => lexer.possible_eq_upgrade(TokenTy::Minus, TokenTy::MinusEq),
                '<' => lexer.possible_eq_upgrade(TokenTy::Lt, TokenTy::LtEq),
                '>' => lexer.possible_eq_upgrade(TokenTy::Gt, TokenTy::GtEq),
                '=' => lexer.possible_eq_upgrade(TokenTy::Eq, TokenTy::EqEq),
                '/' => lexer.possible_eq_upgrade(TokenTy::Div, TokenTy::DivEq),

                // Tokens that can be followed by themselves or an equal sign.
                '&' => {
                    lexer.possible_eq_or_double('&', TokenTy::And, TokenTy::AndEq, TokenTy::AndAnd)
                }
                '|' => lexer.possible_eq_or_double('|', TokenTy::Or, TokenTy::OrEq, TokenTy::OrOr),

                // Dot and range tokens which do not follow any other patern.
                '.' => {
                    // Check for `..` or `..=`.
                    if lexer.consume_if_eq('.') == 1 {
                        if lexer.consume_if_eq('=') == 1 {
                            lexer.emit_token(TokenTy::RangeInclusive, 3);
                        } else {
                            lexer.emit_token(TokenTy::Range, 2);
                        }
                    } else {
                        lexer.emit_single_byte_token(TokenTy::Dot);
                    }
                }

                // Whitespace gets consumed and combined into a single token.
                whitespace if whitespace.is_whitespace() => {
                    // Save the starting byte index of the whitespace.
                    let mut size = whitespace.len_utf8();
                    // Consume all the whitespace characters available.
                    while lexer
                        .iterator
                        .peek()
                        .filter(|c| c.is_whitespace())
                        .is_some()
                    {
                        // Add the byte length of the consumed character to the consumed size.
                        size += lexer.next().unwrap().len_utf8();
                    }
                    // Emit the whitespace token.
                    lexer.emit_token(TokenTy::Whitespace, size);
                }

                _ => unimplemented!(),
            }
        }

        // Push end token,
        lexer.emit_token(TokenTy::End, 0);
        return lexer.output;
    }
}
