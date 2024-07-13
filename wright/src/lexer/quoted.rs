//! Lexing implementation for quoted literals.

use super::{token::Token, token::TokenTy, Lexer};
use std::str::Chars;

/// Attempt to parse a quoted literal. This includes [TokenTy::StringLiteral], [TokenTy::CharLiteral], and
/// [TokenTy::FormatStringLiteral].
pub fn try_consume_quoted_literal(lexer: &mut Lexer) -> Option<Token> {
    // Make a chars iterator to lex from.
    let mut chars: Chars = lexer.remaining.chars();
    // Get the first char from the character iterator.
    // Return none if the first character doesn't exist or is not one of the quote terminating characters.
    let first: char = chars.next().filter(|c| ['\'', '"', '`'].contains(c))?;
    // Track number of bytes consumed.
    let mut bytes_consumed: usize = first.len_utf8();
    // Track whether the quoted literal is terminated.
    let mut is_terminated: bool = false;

    // Consume from the iterator while possible.
    while let Some(consumed) = chars.next() {
        // Update the number of bytes consumed.
        bytes_consumed += consumed.len_utf8();

        // Check if the character matches the starting char.
        // If so, record the literal as terminated and break this loop.
        if consumed == first {
            is_terminated = true;
            break;
        }

        // If the character we just consumed is a backslash.
        // We only handle escaped terminators here, rather than parsing actual meaning.
        // Consume the next character if there is one, regardless of what it is.
        // This prevents an escaped terminator from ending the literal.
        if consumed == '\\' {
            // If there is no next char, do not add anything to the number of bytes consumed.
            bytes_consumed += chars.next().map(char::len_utf8).unwrap_or(0);
        }
    }

    // Return when we have either reached a terminator or run out of characters.
    // First determine the variant to return.
    let variant: TokenTy = match first {
        '\'' => TokenTy::CharLiteral {
            terminated: is_terminated,
        },

        '\"' => TokenTy::StringLiteral {
            terminated: is_terminated,
        },

        '`' => TokenTy::FormatStringLiteral {
            terminated: is_terminated,
        },

        _ => unreachable!("There are no other quoted literals"),
    };

    // SAFETY: Summing char lengths from the iterator should never give us an invalid or out of bounds index.
    Some(unsafe { lexer.split_token_unchecked(bytes_consumed, variant) })
}

#[cfg(test)]
mod tests {
    use super::super::{token::TokenTy, Lexer};

    #[test]
    fn string_literal() {
        let mut lexer = Lexer::new_test(r#""Test string literal""#);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.variant, TokenTy::StringLiteral { terminated: true });
        assert_eq!(token.fragment.as_str(), "\"Test string literal\"");
    }
}
