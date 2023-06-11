//! The wright lexer. This module is responsible for lexical analysis and initial processing of source code.

mod pretty_print;

use derive_more::Display;
use std::{
    iter::{FusedIterator, Peekable},
    str::Chars,
};

/// Token of Wright source code.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{} ({}b)", variant, length)]
pub struct Token {
    /// What type of token is it?
    pub variant: TokenTy,
    /// How many bytes of source code long is it? Note this doesn't necessarily mean how many characters long it is.
    pub length: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum TokenTy {
    // Operators and parentheses
    LeftParen,      // (
    RightParen,     // )
    Bang,           // !
    BangEq,         // !=
    Tilde,          // ~
    TildeArrow,     // ~>
    TildeEq,        // ~=
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
    SingleArrow,    // ->
    Gt,             // >
    GtEq,           // >=
    ShiftRight,     // >>
    Lt,             // <
    LtEq,           // <=
    ShiftLeft,      // <<
    Eq,             // =
    EqEq,           // ==
    DoubleArrow,    // =>
    Div,            // /
    DivEq,          // /=
    Semi,           // ;
    Colon,          // :
    ColonColon,     // ::
    ColonEq,        // :=
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
    #[display(fmt = "W")]
    Whitespace,

    /// Single line comment started with `#`. Optionally `## ` or `##! ` for documentation.
    #[display(fmt = "Single line {} comment", comment_type)]
    SingleLineComment {
        comment_type: CommentTy,
    },

    /// Multiline comment between `#*` and `*#`. Starts with `#**` or `#*!` for documentation.
    #[display(
        fmt = "Multiline {} comment (terminated = {})",
        comment_type,
        is_terminated
    )]
    MultilineComment {
        comment_type: CommentTy,
        /// Is this comment terminated? If not raise an error before parsing the tokens.
        is_terminated: bool,
    },

    /// Integer literal. This is a literal integer in source code. May include underscores after the leading digit
    /// as visual seperators. May also include a prefix such as `0x`, `0o`, or `0b` for hex, octal, or binary.
    IntegerLit,

    /// A string literal in source code.
    #[display(
        fmt = "StringLit (fmt = {}, terminated = {})",
        is_format,
        is_terminated
    )]
    StringLit {
        /// For format strings (backticks instead of double quotes)
        is_format: bool,
        /// Is this string terminated?
        is_terminated: bool,
    },

    /// A character literal in source code.
    #[display(fmt = "CharLit (terminated = {})", is_terminated)]
    CharLit {
        /// Is the char lit terminated?
        is_terminated: bool,
    },

    /// A identifier in source code (such as a variable name). At this stage keywords (such as 'struct') are
    /// also considered identifiers.
    #[display(fmt = "ID")]
    Identifier,

    /// Unknown character for the lexer.
    #[display(fmt = "?")]
    Unknown,
}

/// Different types of comments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum CommentTy {
    /// Normal comment that does not get used in documentation.
    Normal,
    /// Documentation for a declaration in the file.
    InnerDoc,
    /// Documentation for the file itself.
    OuterDoc,
}

/// Lexical analyzer for wright code. This struct host functions that produce tokens from wright source.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Iterator over the indexed input characters tied to the lifetime of the source code.
    iterator: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // Get the next character out of the iterator.
        let next = self.iterator.next()?;

        // Handle single character tokens first.
        let single_char_tokens = [
            ('(', TokenTy::LeftParen),
            (')', TokenTy::RightParen),
            ('[', TokenTy::LeftSquare),
            (']', TokenTy::RightSquare),
            ('{', TokenTy::LeftBracket),
            ('}', TokenTy::RightBracket),
            ('@', TokenTy::At),
            (';', TokenTy::Semi),
            ('?', TokenTy::Question),
            (',', TokenTy::Comma),
        ];

        for (c, variant) in single_char_tokens {
            if next == c {
                return Some(Token { variant, length: 1 });
            }
        }

        // Next handle tokens that can possibly be followed by an equal sign.
        let possible_eq_upgrades = [
            ('!', TokenTy::Bang, TokenTy::BangEq),
            ('%', TokenTy::Mod, TokenTy::ModEq),
            ('^', TokenTy::Xor, TokenTy::XorEq),
            ('*', TokenTy::Star, TokenTy::StarEq),
            ('+', TokenTy::Plus, TokenTy::PlusEq),
            ('/', TokenTy::Div, TokenTy::DivEq),
        ];

        for (c, no_eq, with_eq) in possible_eq_upgrades {
            if next == c {
                return match self.iterator.next_if_eq(&'=') {
                    Some(_) => Some(Token {
                        variant: with_eq,
                        length: 2,
                    }),
                    None => Some(Token {
                        variant: no_eq,
                        length: 1,
                    }),
                };
            }
        }

        // Next handle tokens that can be doubled or have an equals sign.
        let possible_eq_or_double = [
            ('&', TokenTy::And, TokenTy::AndEq, TokenTy::AndAnd),
            ('|', TokenTy::Or, TokenTy::OrEq, TokenTy::OrOr),
            ('<', TokenTy::Lt, TokenTy::LtEq, TokenTy::ShiftLeft),
            ('>', TokenTy::Gt, TokenTy::GtEq, TokenTy::ShiftRight),
            (':', TokenTy::Colon, TokenTy::ColonEq, TokenTy::ColonColon),
        ];

        for (c, alone, with_eq, doubled) in possible_eq_or_double {
            if next == c {
                return match self.iterator.next_if(|&x| x == '=' || x == c) {
                    Some('=') => Some(Token {
                        variant: with_eq,
                        length: 2,
                    }),
                    Some(x) if x == c => Some(Token {
                        variant: doubled,
                        length: 2,
                    }),
                    None => Some(Token {
                        variant: alone,
                        length: 1,
                    }),
                    _ => unreachable!(),
                };
            }
        }

        // Next deal with arrows
        let arrows = [
            ('-', TokenTy::Minus, TokenTy::MinusEq, TokenTy::SingleArrow),
            ('=', TokenTy::Eq, TokenTy::EqEq, TokenTy::DoubleArrow),
            ('~', TokenTy::Tilde, TokenTy::TildeEq, TokenTy::TildeArrow),
        ];

        for (c, alone, with_eq, as_arrow) in arrows {
            if next == c {
                return match self.iterator.next_if(|&x| x == '=' || x == '>') {
                    Some('=') => Some(Token {
                        variant: with_eq,
                        length: 2,
                    }),
                    Some('>') => Some(Token {
                        variant: as_arrow,
                        length: 2,
                    }),
                    None => Some(Token {
                        variant: alone,
                        length: 1,
                    }),
                    _ => unreachable!(),
                };
            }
        }

        // Dot and range operators.
        if next == '.' {
            return match self.iterator.next_if_eq(&'.') {
                None => Some(Token {
                    variant: TokenTy::Dot,
                    length: 1,
                }),
                Some(_) => match self.iterator.next_if_eq(&'=') {
                    None => Some(Token {
                        variant: TokenTy::Range,
                        length: 2,
                    }),
                    Some(_) => Some(Token {
                        variant: TokenTy::RangeInclusive,
                        length: 3,
                    }),
                },
            };
        }

        // Whitespace.
        if next.is_whitespace() {
            // Accumulate the number of bytes of whitespace consumed.
            let mut acc = next.len_utf8();
            // Use while-let instead of take-while to avoid consuming the whole iterator.
            while let Some(consumed) = self.iterator.next_if(|&x| x.is_whitespace()) {
                acc += consumed.len_utf8();
            }

            return Some(Token {
                variant: TokenTy::Whitespace,
                length: acc,
            });
        }

        // Identifiers
        if unicode_ident::is_xid_start(next) || next == '_' {
            // Accumulate the number of bytes consumed in the identifier.
            let mut acc = next.len_utf8();
            // Consume the rest of the identifier.
            while let Some(consumed) = self
                .iterator
                .next_if(|&x| unicode_ident::is_xid_continue(x))
            {
                acc += consumed.len_utf8();
            }

            return Some(Token {
                variant: TokenTy::Identifier,
                length: acc,
            });
        }

        // Numerical literals.
        if next.is_ascii_digit() {
            // Accumulate the number of bytes consumed in the numeric literal.
            // All ascii is 1 byte wide so avoid the extra call to `.len_utf8()`.
            let mut acc = 1;
            // Track the radix
            let mut radix = 10;

            // Change the radix if necessary
            if next == '0' {
                if let Some(prefix) = self
                    .iterator
                    .next_if(|x| ['x', 'o', 'b', 'X', 'B'].contains(x))
                {
                    acc += 1;

                    radix = match prefix {
                        'x' | 'X' => 16,
                        'b' | 'B' => 2,
                        'o' => 8,
                        _ => unreachable!(),
                    };
                }
            }

            // Consume the rest of the integer literal.
            while self
                .iterator
                .next_if(|&x| x.is_digit(radix) || x == '_')
                .is_some()
            {
                // All accepted characters should be ascii, so we can just simplify `.len_utf8()` to 1.
                acc += 1;
            }

            return Some(Token {
                variant: TokenTy::IntegerLit,
                length: acc,
            });
        }

        // String and Character literals.
        if ['\'', '"', '`'].contains(&next) {
            // Accumulator to track number of bytes consumed.
            let mut acc: usize = 1;
            let mut is_terminated = false;

            // Consume characters until the end of the literal
            while let Some(consumed) = self.iterator.next() {
                acc += consumed.len_utf8();

                match consumed {
                    // Ending character is the same as starting character.
                    // Escapes should all be handled, so don't worry about this being escaped.
                    _ if consumed == next => {
                        is_terminated = true;
                        break;
                    }

                    // Escaped pattern.
                    // Only worry about escaped terminators here, since all other escaped
                    // patterns can be dealt with later.
                    '\\' => {
                        // Consume the escaped character regardless of what it is.
                        // It will always be part of the quoted literal.
                        if let Some(escaped) = self.iterator.next() {
                            acc += escaped.len_utf8();
                        }
                    }

                    // Do nothing for non-escaped chars since the quoted literal continues
                    // and we have already recorded the consumed bytes.
                    _ => {}
                }
            }

            // We have finished consuming the literal -- make sure we produce the
            // right variant
            return match next {
                '\'' => Some(Token {
                    variant: TokenTy::CharLit { is_terminated },
                    length: acc,
                }),
                _ => Some(Token {
                    variant: TokenTy::StringLit {
                        is_format: next == '`',
                        is_terminated,
                    },
                    length: acc,
                }),
            };
        }

        // Comments.
        if next == '#' {
            // Use accumulator to track number of bytes consumed.
            let mut acc = 1;

            // There are a few variants as follows.
            // `#...` - single line comment
            // `#*...*#` - multiline comment
            // `##...` - single line inner doc comment
            // `##!...` - single line outer doc comment
            // `#**...*#` - multiline inner doc comment
            // `#*!...*#` - multiline outer doc comment
            // If a multiline comment is not terminated by the end of the file then just mark it as such in the
            // produced token. A seperate token error handling layer will raise that outside of this function.

            // Handle multiline comments
            if self.iterator.next_if_eq(&'*').is_some() {
                acc += 1;

                // Check if it's a doc comment.
                let comment_type = match self.iterator.next_if(|&x| x == '*' || x == '!') {
                    Some('*') => {
                        acc += 1;
                        CommentTy::InnerDoc
                    }

                    Some('!') => {
                        acc += 1;
                        CommentTy::OuterDoc
                    }

                    None => CommentTy::Normal,

                    _ => unreachable!(),
                };

                // Read the rest of the multi-line comment
                while let Some(consumed) = self.iterator.next() {
                    acc += consumed.len_utf8();
                    if consumed == '*' && self.iterator.next_if_eq(&'#').is_some() {
                        acc += 1;
                        return Some(Token {
                            variant: TokenTy::MultilineComment {
                                comment_type,
                                is_terminated: true,
                            },
                            length: acc,
                        });
                    }
                }

                // If we hit the end, the comment is not terminated.
                return Some(Token {
                    variant: TokenTy::MultilineComment {
                        comment_type,
                        is_terminated: false,
                    },
                    length: acc,
                });
            }

            // Handle single line comment.
            let mut comment_type = CommentTy::Normal;

            // Check for inner doc comment
            if self.iterator.next_if_eq(&'#').is_some() {
                acc += 1;
                comment_type = CommentTy::InnerDoc;

                // Check for outer doc comment
                if self.iterator.next_if_eq(&'!').is_some() {
                    acc += 1;
                    comment_type = CommentTy::OuterDoc;
                }
            }

            // Read to end of line/file for rest of comment. Include line ending in consumed bytes.
            for consumed in self.iterator.by_ref() {
                acc += consumed.len_utf8();
                if consumed == '\n' {
                    break;
                }
            }

            return Some(Token {
                variant: TokenTy::SingleLineComment { comment_type },
                length: acc,
            });
        }

        // If we haven't matched by this point, return an unknown token.
        Some(Token {
            variant: TokenTy::Unknown,
            length: next.len_utf8(),
        })
    }
}

impl<'a> FusedIterator for Lexer<'a> {}

impl<'a> Lexer<'a> {
    /// Create a new lexer that iterates on a given source string.
    pub fn new(source: &'a str) -> Self {
        Lexer {
            iterator: source.chars().peekable(),
        }
    }
}
