//! Implementation for lexing integer literals.

use super::{
    token::{Token, TokenTy},
    Lexer,
};
use std::{iter::Peekable, str::Chars};

/// Attempt to lex and consume an [TokenTy::IntegerLiteral] from the lexer.
pub fn try_consume_integer_literal(lexer: &mut Lexer) -> Option<Token> {
    // Make a peekable character iterator.
    let mut chars: Peekable<Chars> = lexer.remaining.chars().peekable();
    // Get the first character from the iterator. We can only continue lexing if one exists and is an ascii
    // decimal digit.
    let next: char = chars.next().filter(char::is_ascii_digit)?;
    // Track the number of bytes consumed. We use the length of the parsed first char here but we could probably
    // assume it to be 1.
    let mut bytes_consumed: usize = next.len_utf8();
    // Track the radix
    let mut radix: u32 = 10;

    // Change the radix if necessary
    if next == '0' {
        if let Some(prefix) = chars.next_if(|x| ['x', 'o', 'b', 'X', 'B'].contains(x)) {
            // All the possible prefix chars are 1 byte ascii characters.
            bytes_consumed += 1;

            radix = match prefix {
                'x' | 'X' => 16,
                'b' | 'B' => 2,
                'o' => 8,
                _ => unreachable!("the prefix byte is checked above"),
            };
        }
    }

    // Add the rest of the integer literal.
    bytes_consumed += chars
        .take_while(|c| c.is_digit(radix) || *c == '_')
        .map(char::len_utf8)
        .sum::<usize>();

    Some(lexer.split_token(bytes_consumed, TokenTy::IntegerLiteral))
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenTy};

    #[test]
    fn integer_literal() {
        let mut lexer = Lexer::new_test("123_456_789.");

        let token = lexer.next_token().unwrap();

        assert_eq!(token.fragment.as_str(), "123_456_789");
        assert_eq!(token.variant, TokenTy::IntegerLiteral);
    }
}
