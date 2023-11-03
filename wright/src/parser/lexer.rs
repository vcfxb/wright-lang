//! The wright lexer. This module is responsible for lexical analysis and initial processing of source code.

mod pretty_print;
pub mod tokens;

use std::{
    iter::{FusedIterator, Peekable},
    str::CharIndices,
};

use self::tokens::{CommentTy, Token, TokenTy};

/// Lexical analyzer for wright code. This struct host functions that produce tokens from wright source.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    /// Iterator over the indexed input characters tied to the lifetime of the source code.
    iterator: Peekable<CharIndices<'a>>,
    /// The source code passed to the lexer. This is used to check for keywords.
    source: &'a str,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer that iterates on a given source string.
    pub fn new(source: &'a str) -> Self {
        Lexer {
            iterator: source.char_indices().peekable(),
            source,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // Get the next character out of the iterator.
        let (start_index, next) = self.iterator.next()?;

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
            ('#', TokenTy::Pound),
            ('$', TokenTy::Dollar),
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
        ];

        for (c, no_eq, with_eq) in possible_eq_upgrades {
            if next == c {
                return match self.iterator.next_if(|&(_, x)| x == '=') {
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
            ('/', TokenTy::Div, TokenTy::DivEq, TokenTy::DivDiv),
        ];

        for (c, alone, with_eq, doubled) in possible_eq_or_double {
            if next == c {
                return match self.iterator.next_if(|&(_, x)| x == '=' || x == c) {
                    // Followed by `=`
                    Some((_, '=')) => Some(Token {
                        variant: with_eq,
                        length: 2,
                    }),

                    // Followed by itself.
                    Some(_) => Some(Token {
                        variant: doubled,
                        length: 2,
                    }),

                    // Single char token
                    None => Some(Token {
                        variant: alone,
                        length: 1,
                    }),
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
                return match self.iterator.next_if(|&(_, x)| x == '=' || x == '>') {
                    Some((_, '=')) => Some(Token {
                        variant: with_eq,
                        length: 2,
                    }),
                    Some((_, '>')) => Some(Token {
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
            return match self.iterator.next_if(|&(_, x)| x == '.') {
                None => Some(Token {
                    variant: TokenTy::Dot,
                    length: 1,
                }),
                Some(_) => match self.iterator.next_if(|&(_, x)| x == '=') {
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
            while let Some((_, consumed)) = self.iterator.next_if(|&(_, x)| x.is_whitespace()) {
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
            while let Some((_, consumed)) = self
                .iterator
                .next_if(|&(_, x)| unicode_ident::is_xid_continue(x))
            {
                acc += consumed.len_utf8();
            }

            // Get the matching source code to check for reserved words.
            let range = start_index..start_index + acc;
            let matching_source = &self.source[range];

            // Match on reserved words.
            let variant: TokenTy = match matching_source {
                // Declaration keywords
                "class" => TokenTy::Class,
                "struct" => TokenTy::Struct,
                "record" => TokenTy::Record,
                "trait" => TokenTy::Trait,
                "fn" => TokenTy::Fn,
                "enum" => TokenTy::Enum,
                "union" => TokenTy::Union,
                "mod" => TokenTy::Module,
                "import" => TokenTy::Import,

                // Visibility keywords
                "public" => TokenTy::Public,
                "package" => TokenTy::Package,
                "private" => TokenTy::Private,

                // Boolean literals
                "true" => TokenTy::True,
                "false" => TokenTy::False,

                // Other keywords.
                "constraint" => TokenTy::Constraint,
                "constrain" => TokenTy::Constrain,
                "relation" => TokenTy::Relation,
                "unsafe" => TokenTy::Unsafe,
                "unchecked" => TokenTy::Unchecked,
                "impl" => TokenTy::Impl,
                "Self" => TokenTy::SelfUpper,
                "self" => TokenTy::SelfLower,
                "type" => TokenTy::Type,
                "const" => TokenTy::Const,
                "var" => TokenTy::Var,
                "if" => TokenTy::If,
                "else" => TokenTy::Else,
                "match" => TokenTy::Match,
                "is" => TokenTy::Is,
                "as" => TokenTy::As,
                "on" => TokenTy::On,
                "in" => TokenTy::In,
                "not" => TokenTy::Not,
                "dyn" => TokenTy::Dyn,
                "try" => TokenTy::Try,

                _ => TokenTy::Identifier,
            };

            return Some(Token {
                variant,
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
                if let Some((_, prefix)) = self
                    .iterator
                    .next_if(|(_, x)| ['x', 'o', 'b', 'X', 'B'].contains(x))
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
                .next_if(|&(_, x)| x.is_digit(radix) || x == '_')
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
            while let Some((_, consumed)) = self.iterator.next() {
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
                        if let Some((_, escaped)) = self.iterator.next() {
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
            if self.iterator.next_if(|&(_, x)| x == '*').is_some() {
                acc += 1;

                // Check if it's a doc comment.
                let comment_type = match self.iterator.next_if(|&(_, x)| x == '*' || x == '!') {
                    Some((_, '*')) => {
                        acc += 1;
                        CommentTy::InnerDoc
                    }

                    Some((_, '!')) => {
                        acc += 1;
                        CommentTy::OuterDoc
                    }

                    None => CommentTy::Normal,

                    _ => unreachable!(),
                };

                // Read the rest of the multi-line comment
                while let Some((_, consumed)) = self.iterator.next() {
                    acc += consumed.len_utf8();
                    if consumed == '*' && matches!(self.iterator.peek(), Some((_, '#'))) {
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
            if self.iterator.next_if(|&(_, x)| x == '#').is_some() {
                acc += 1;
                comment_type = CommentTy::InnerDoc;

                // Check for outer doc comment
                if self.iterator.next_if(|&(_, x)| x == '!').is_some() {
                    acc += 1;
                    comment_type = CommentTy::OuterDoc;
                }
            }

            // Read to end of line/file for rest of comment. Include line ending in consumed bytes.
            for (_, consumed) in self.iterator.by_ref() {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Get the size hint of the internal iterator.
        let (inner_lower, upper) = self.iterator.size_hint();
        // If there are any characters left, then there is at least one token remaining.
        ((inner_lower > 0) as usize, upper)
    }
}

impl<'a> FusedIterator for Lexer<'a> {}

/// A token with an index in a piece of source code.
#[derive(Copy, Clone, Debug)]
pub struct IndexedToken {
    /// The byte index into the source code that this token starts on.
    pub index: usize,
    /// The token itself.
    pub token: Token,
}

/// An iterator over the tokens in the source code with byte indices attached.
#[derive(Debug, Clone)]
pub struct IndexedLexer<'src> {
    /// The current index in source code -- the number of bytes currently consumed by the iterator.
    pub index: usize,
    /// The underlying lexer iterator.
    lexer: Lexer<'src>,
}

impl<'src> IndexedLexer<'src> {
    /// Construct a new indexed lexer.
    pub fn new(source: &'src str) -> Self {
        Self {
            index: 0,
            lexer: Lexer::new(source),
        }
    }
}

impl<'a> Iterator for IndexedLexer<'a> {
    type Item = IndexedToken;

    fn next(&mut self) -> Option<Self::Item> {
        // Pull a token from the iterator.
        let token = self.lexer.next()?;

        // If available, add the current index to it to return.
        let indexed_token = IndexedToken {
            index: self.index,
            token,
        };

        // Update the current index with the length of the token.
        self.index += token.length;

        return Some(indexed_token);
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.lexer.size_hint()
    }
}

impl<'a> FusedIterator for IndexedLexer<'a> {}
