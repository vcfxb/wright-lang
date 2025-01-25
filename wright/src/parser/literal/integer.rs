//! Integer literal parsing implementation.

use num::{BigUint, Num};

use crate::parser::error::{ParserError, ParserErrorKind};
use crate::parser::Parser;
use crate::{ast::literal::IntegerLiteral, lexer::token::TokenTy};

impl IntegerLiteral {
    /// Parse an integer literal from the given [Parser].
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        // Get the token containing the integer literal from the parser.
        let Some(int_lit_token) = parser.next_if_is(TokenTy::IntegerLiteral) else {
            return match parser.peek_fragment() {
                Some(frag) => Err(ParserError {
                    kind: ParserErrorKind::ExpectedIntegerLiteral,
                    location: frag.clone(),
                    help: None,
                }),

                None => Err(ParserError {
                    kind: ParserErrorKind::ExpectedIntegerLiteral,
                    location: parser.lexer.remaining.clone(),
                    help: Some("found end of source".into()),
                }),
            };
        };

        // Get the string to pass to num for the rest of parsing.
        let mut parse_str: &str = int_lit_token.fragment.as_str();
        let mut chars = parse_str.chars();

        // Unwrap: Integer literals must be at minimum 1 character, enforced by the lexer.
        // use null byte as a sentinel value for the second one, since we're just using the prefix to check for
        // a radix to pass to num.
        let prefix: [char; 2] = [chars.next().unwrap(), chars.next().unwrap_or('\0')];

        // Determine the radix and remove any prefix in the process.
        let radix: u32 = match prefix {
            // Hexidecimal.
            ['0', 'x' | 'X'] => {
                parse_str = &parse_str[2..];
                16
            }

            // Binary.
            ['0', 'b' | 'B'] => {
                parse_str = &parse_str[2..];
                2
            }

            // Octal
            ['0', 'o'] => {
                parse_str = &parse_str[2..];
                8
            }

            // All other patterns are not radix-prefixes.
            _ => 10,
        };

        // Pass the remainder of parsing off to num.
        let value = BigUint::from_str_radix(parse_str, radix)
            // We can use expect here for now since we have validated the format of the string
            // on our own before passing it off.
            .expect("num should successfully parse");

        Ok(IntegerLiteral {
            fragment: int_lit_token.fragment,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use num::BigUint;

    use crate::{ast::literal::IntegerLiteral, lexer::Lexer, parser::Parser};

    #[test]
    fn normal() {
        let mut parser = Parser::new(Lexer::new_test("1000"));

        let int_lit = IntegerLiteral::parse(&mut parser).unwrap();

        assert_eq!(int_lit.value, BigUint::new(vec![1000]));
        assert_eq!(parser.lexer.remaining.as_str(), "");
        assert_eq!(int_lit.fragment.as_str(), "1000");
    }

    // #[test]
    // fn ingore_underscores
}
