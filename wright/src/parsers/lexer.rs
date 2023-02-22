//! The wright lexer. This module is responsible for lexical analysis and initial processing of source code.

use derive_more::Display;
use std::{cmp, iter::Peekable, str::Chars};

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
    #[display(fmt = "Multiline {} comment (terminated = {})", comment_type, terminated)]
    MultilineComment {
        comment_type: CommentTy,
        /// Is this comment terminated? If not raise an error before parsing the tokens.
        terminated: bool,
    },

    /// Integer literal. This is a literal integer in source code. May include underscores after the leading digit
    /// as visual seperators. May also include a prefix such as `0x`, `0o`, or `0b` for hex, octal, or binary.
    IntegerLit,

    /// A string literal in source code.
    StringLit,

    /// A character literal in source code.
    CharLit,

    /// A identifier in source code (such as a variable name). At this stage keywords (such as 'struct') are
    /// also considered identifiers.
    #[display(fmt = "ID")]
    Identifier,

    /// Unknown character for the lexer.
    #[display(fmt = "?")]
    Unknown,

    /// End of input/file.
    End,
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
        self.output.push(Token { variant, length: bytes });
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
    fn possible_eq_or_double(&mut self, c: char, without: TokenTy, with_eq: TokenTy, doubled: TokenTy) {
        if self.consume_if_eq(c) > 0 {
            self.emit_token(doubled, c.len_utf8() + 1)
        } else if self.consume_if_eq('=') > 0 {
            self.emit_token(with_eq, 2)
        } else {
            self.emit_single_byte_token(without)
        }
    }

    /// Read through the next occurrence of `c`. Return the total number of bytes consumed from the iterator. If `c` is
    /// not found, read to the end of the iterator.
    fn read_through(&mut self, c: char) -> usize {
        let mut acc = 0;
        while let Some(next) = self.next() {
            acc += next.len_utf8();
            if next == c {
                break;
            }
        }
        return acc;
    }

    /// Make the lexer read through the next `*#` or to the end of the file. If a closing pattern (`*#`) is not found,
    /// the boolean is set to false in the return.  
    fn read_through_end_of_multiline_comment(&mut self) -> (usize, bool) {
        let mut acc = 0;
        let mut seen_star = false;
        while let Some(c) = self.next() {
            // Add consumed character to accumulator.
            acc += c.len_utf8();

            // If we see a pound just after a star, return Ok.
            if c == '#' && seen_star {
                return (acc, true);
            }

            // If we see a star, set the flag, otherwise, unset it.
            if c == '*' {
                seen_star = true;
            } else {
                seen_star = false;
            }
        }
        // If we finish the loop with no return, we have hit the end of the file.
        return (acc, false);
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

                // Tokens that can possibly be followed by an equal sign.
                '!' => lexer.possible_eq_upgrade(TokenTy::Bang, TokenTy::BangEq),
                '%' => lexer.possible_eq_upgrade(TokenTy::Mod, TokenTy::ModEq),
                '^' => lexer.possible_eq_upgrade(TokenTy::Xor, TokenTy::XorEq),
                '*' => lexer.possible_eq_upgrade(TokenTy::Star, TokenTy::StarEq),
                '+' => lexer.possible_eq_upgrade(TokenTy::Plus, TokenTy::PlusEq),
                '/' => lexer.possible_eq_upgrade(TokenTy::Div, TokenTy::DivEq),

                // Tokens that can be followed by themselves or an equal sign.
                '&' => lexer.possible_eq_or_double('&', TokenTy::And, TokenTy::AndEq, TokenTy::AndAnd),
                '|' => lexer.possible_eq_or_double('|', TokenTy::Or, TokenTy::OrEq, TokenTy::OrOr),
                '<' => lexer.possible_eq_or_double('<', TokenTy::Lt, TokenTy::LtEq, TokenTy::ShiftLeft),
                '>' => lexer.possible_eq_or_double('>', TokenTy::Gt, TokenTy::GtEq, TokenTy::ShiftRight),

                // Arrows
                c if c == '=' || c == '-' || c == '~' => {
                    let next_if_eq_or_arrow = lexer
                        .iterator
                        .peek()
                        .filter(|peeked| **peeked == '=' || **peeked == '>')
                        .is_some()
                        .then(|| lexer.iterator.next().unwrap());
                    match (c, next_if_eq_or_arrow) {
                        ('=', Some('=')) => lexer.emit_token(TokenTy::EqEq, 2),
                        ('=', Some('>')) => lexer.emit_token(TokenTy::DoubleArrow, 2),
                        ('=', None) => lexer.emit_single_byte_token(TokenTy::Eq),
                        ('-', Some('=')) => lexer.emit_token(TokenTy::MinusEq, 2),
                        ('-', Some('>')) => lexer.emit_token(TokenTy::SingleArrow, 2),
                        ('-', None) => lexer.emit_single_byte_token(TokenTy::Minus),
                        ('~', Some('=')) => lexer.emit_token(TokenTy::TildeEq, 2),
                        ('~', Some('>')) => lexer.emit_token(TokenTy::TildeArrow, 2),
                        ('~', None) => lexer.emit_single_byte_token(TokenTy::Tilde),
                        // No other combination should be possible here.
                        _ => unreachable!(),
                    }
                }

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
                    while lexer.iterator.peek().filter(|c| c.is_whitespace()).is_some() {
                        // Add the byte length of the consumed character to the consumed size.
                        size += lexer.next().unwrap().len_utf8();
                    }
                    // Emit the whitespace token.
                    lexer.emit_token(TokenTy::Whitespace, size);
                }

                // Comments. There are a few variants as follows.
                // `#...` - single line comment
                // `#*...*#` - multiline comment
                // `##...` - sinlge line inner doc comment
                // `##!...` - single line outer doc comment
                // `#**...*#` - multiline inner doc comment
                // `#*!...*#` - multiline outer doc comment
                // If a multiline comment is not terminated by the end of the file then just mark it as such in the
                // produced token. A seperate token error handling layer will raise that outside of this function.
                '#' => {
                    // If the second character is a star, this is a multiline comment.
                    if lexer.consume_if_eq('*') > 0 {
                        if lexer.consume_if_eq('*') > 0 {
                            // Inner doc comment
                            let (consumed, terminated) = lexer.read_through_end_of_multiline_comment();
                            // Add 3 for consumed `#**`.
                            lexer.emit_token(
                                TokenTy::MultilineComment {
                                    comment_type: CommentTy::InnerDoc,
                                    terminated,
                                },
                                consumed + 3,
                            );
                        } else if lexer.consume_if_eq('!') > 0 {
                            // Outer doc comment
                            let (consumed, terminated) = lexer.read_through_end_of_multiline_comment();
                            // Add 3 for consumed `#*!`.
                            lexer.emit_token(
                                TokenTy::MultilineComment {
                                    comment_type: CommentTy::OuterDoc,
                                    terminated,
                                },
                                consumed + 3,
                            );
                        } else {
                            // Normal multiline comment.
                            let (consumed, terminated) = lexer.read_through_end_of_multiline_comment();
                            // Add two to the bytes for the prefix `#*`.
                            lexer.emit_token(
                                TokenTy::MultilineComment {
                                    comment_type: CommentTy::Normal,
                                    terminated,
                                },
                                consumed + 2,
                            );
                        }
                    } else if lexer.consume_if_eq('#') > 0 {
                        // If the second character is a `#` then this is a single line doc comment.
                        if lexer.consume_if_eq('!') > 0 {
                            // This is an outer doc comment. Add the three bytes already read to the number before the
                            // end of the line.
                            let consumed = 3 + lexer.read_through('\n');
                            lexer.emit_token(
                                TokenTy::SingleLineComment {
                                    comment_type: CommentTy::OuterDoc,
                                },
                                consumed,
                            );
                        } else {
                            // Read to the end of the line and emit the inner doc comment.
                            let consumed = 2 + lexer.read_through('\n');
                            lexer.emit_token(
                                TokenTy::SingleLineComment {
                                    comment_type: CommentTy::InnerDoc,
                                },
                                consumed,
                            );
                        }
                    } else {
                        // Normal single line comment.
                        // Read to end of line/file. Add one for the `#` already read.
                        let consumed = 1 + lexer.read_through('\n');
                        lexer.emit_token(
                            TokenTy::SingleLineComment {
                                comment_type: CommentTy::Normal,
                            },
                            consumed,
                        );
                    }
                }

                // Identifiers must start with either a unicode XID start character or an underscore.
                // the rest of them must be unicode XID continue characters.
                c if unicode_ident::is_xid_start(c) || c == '_' => {
                    // Save the size of this first character consumed.
                    let mut size = c.len_utf8();
                    // Consume all unicode identifier continue characters.
                    while lexer
                        .iterator
                        .peek()
                        .filter(|c| unicode_ident::is_xid_continue(**c))
                        .is_some()
                    {
                        // Add the length of the consumed char to the consumed size.
                        size += lexer.next().unwrap().len_utf8();
                    }
                    // Emit the identifier token.
                    lexer.emit_token(TokenTy::Identifier, size);
                }

                // Numerical literals
                c if c.is_digit(10) => {
                    // Save the size of the consumed value.
                    let mut size = c.len_utf8();
                    let mut radix = 10;

                    // Check if we need to change radix
                    for r in ['x', 'o', 'b', 'X', 'B'] {
                        if lexer.consume_if_eq(r) > 0 {
                            size += 1;
                            radix = match r {
                                'X' | 'x' => 16,
                                'B' | 'b' => 2,
                                'o' => 8,
                                _ => unreachable!(),
                            }
                        }
                    }

                    // Consume available characters.
                    while lexer
                        .iterator
                        .peek()
                        .filter(|n| n.is_digit(radix) || **n == '_')
                        .is_some()
                    {
                        size += lexer.next().unwrap().len_utf8();
                    }

                    // Emit the integer literal token.
                    lexer.emit_token(TokenTy::IntegerLit, size);
                }

                // Emit an unknown token for all not caught above.
                other => lexer.emit_token(TokenTy::Unknown, other.len_utf8()),
            }
        }

        // Push end token,
        lexer.emit_token(TokenTy::End, 0);
        return lexer.output;
    }

    /// Print in pretty format the source code and the tokens it matched to under it.
    pub fn debug_pretty_print(source: &str) {
        // This could eventually perhaps be upgraded with codespan but we'll do it manually for now.

        // Get the tokens for the source code.
        let tokens = Lexer::lex(source);
        // Start the byte index of the cursor at 0.
        let mut byte_index: usize = 0;
        let mut line_index: usize = 0;
        let mut line_pair = [String::new(), String::new()];
        // Iterate through tokens and add them to the output.
        for token in tokens.iter() {
            // Get the matching source code for the token.
            let mut matching_source = source[byte_index..(byte_index + token.length)].to_owned();
            // Count the newlines to add after processing this token.
            let newline_count = matching_source.chars().filter(|c| *c == '\n').count();
            // Check if there's a newline in the source token. Also include the end of source code so that we get everything.
            let contains_newline = newline_count > 0 || token.variant == TokenTy::End;
            // Get the display string of the token.
            let token_str = token.to_string();
            // Replace certain characters in the matching source to avoid pretty printing issues.
            matching_source = matching_source
                // Replace tabs with 4 spaces for consistency
                .replace("\t", "    ")
                // Replace newline characters with spaces to avoid printing extra lines.
                .replace("\n", " ")
                .replace("\r", " ");

            // Get the width of the display as the max of the two string character (not byte) lengths. Add two to the
            // token length to represent the square brackets added later.
            let width: usize = cmp::max(token_str.chars().count() + 2, matching_source.chars().count());

            // Add line numbers if the strings are empty.
            if line_pair[0].is_empty() {
                for s in line_pair.iter_mut() {
                    s.push_str(format!("{:03} ({:#010x}): ", line_index, byte_index).as_str());
                }
            }

            // Add source to first line and token info to second line as appopriate. Add two to the source with for the
            // square brackets.
            line_pair[0].push_str(format!("{matching_source:<width$}").as_str());
            line_pair[1].push_str(format!("[{token_str:<0$}]", width - 2).as_str());

            if contains_newline {
                println!("{}\n{}", line_pair[0], line_pair[1]);
                line_pair = [String::new(), String::new()];
                // Ensure we add all the newlines in a multiline comment.
                line_index += newline_count;
            }

            byte_index += token.length;
        }
    }
}
