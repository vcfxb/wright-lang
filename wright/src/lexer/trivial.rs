//! Trivial tokens and their implementation.

use super::{
    Lexer,
    token::{Token, TokenTy},
};

/// Trivial tokens that are two ASCII characters and can be matched directly
/// against the input source code.
pub const TWO_ASCII_TRIVIAL_TOKENS: &[(&[u8; 2], TokenTy)] = &[
    (b"->", TokenTy::SingleArrow),
    (b"-=", TokenTy::MinusEq),
    (b"=>", TokenTy::DoubleArrow),
    (b"==", TokenTy::EqEq),
    (b"&&", TokenTy::AndAnd),
    (b"||", TokenTy::OrOr),
    (b"<<", TokenTy::LtLt),
    (b">>", TokenTy::GtGt),
    (b"::", TokenTy::ColonColon),
    (b"|=", TokenTy::OrEq),
    (b"&=", TokenTy::AndEq),
    (b":=", TokenTy::ColonEq),
    (b">=", TokenTy::GtEq),
    (b"<=", TokenTy::LtEq),
    (b"!=", TokenTy::BangEq),
    (b"%=", TokenTy::ModEq),
    (b"^=", TokenTy::XorEq),
    (b"*=", TokenTy::StarEq),
    (b"+=", TokenTy::PlusEq),
    (b"/=", TokenTy::DivEq),
];

/// Single ASCII character trivial tokens that can be matched directly against
/// the source code.
pub const SINGLE_ASCII_CHAR_TRIVIAL_TOKENS: &[(u8, TokenTy)] = &[
    (b'(', TokenTy::LeftParen),
    (b')', TokenTy::RightParen),
    (b'[', TokenTy::LeftBracket),
    (b']', TokenTy::RightBracket),
    (b'{', TokenTy::LeftCurly),
    (b'}', TokenTy::RightCurly),
    (b'@', TokenTy::At),
    (b';', TokenTy::Semi),
    (b'?', TokenTy::Question),
    (b',', TokenTy::Comma),
    (b'#', TokenTy::Hash),
    (b'$', TokenTy::Dollar),
    (b'>', TokenTy::Gt),
    (b'<', TokenTy::Lt),
    (b'-', TokenTy::Minus),
    (b':', TokenTy::Colon),
    (b'!', TokenTy::Bang),
    (b'=', TokenTy::Eq),
    (b'&', TokenTy::And),
    (b'|', TokenTy::Or),
    (b'/', TokenTy::Div),
    (b'+', TokenTy::Plus),
    (b'^', TokenTy::Xor),
    (b'*', TokenTy::Star),
    (b'%', TokenTy::Mod),
];

/// Attempt to consume a "trivial" token from the start of the [Lexer]'s [Lexer::remaining] fragment.
///
/// Leave the lexer unmodified if one is not available.
pub fn try_consume_trivial_token(lexer: &mut Lexer) -> Option<Token> {
    // Get the number of bytes remaining, since we need at least 1 to parse anything.
    let bytes_remaining: usize = lexer.bytes_remaining();

    // No token if there are no bytes of source left.
    if bytes_remaining == 0 {
        return None;
    }

    // Attempt to match any two-byte ASCII trivial tokens.
    // This must be done before single-ascii byte tokens since matching is greedy.
    if bytes_remaining >= 2 {
        // Get the first two bytes of the remaining fragment.
        // SAFETY: We just checked length.
        let bytes: &[u8] = unsafe { lexer.remaining.as_str().as_bytes().get_unchecked(0..2) };

        // Match against each possible token pattern.
        for (pattern, kind) in TWO_ASCII_TRIVIAL_TOKENS {
            if bytes == *pattern {
                // We have already done bounds checking, and this cannot be a character
                // boundary since we just matched against ASCII characters.
                return Some(lexer.split_token_unchecked(2, *kind));
            }
        }
    }

    // Do the same for single byte patterns.
    // SAFETY: We checked that the number of bytes remaining is not 0 above.
    let byte: &u8 = unsafe { lexer.remaining.as_str().as_bytes().get_unchecked(0) };

    for (pattern, kind) in SINGLE_ASCII_CHAR_TRIVIAL_TOKENS {
        if byte == pattern {
            // If we matched, then the first byte is ASCII, and therefore we don't have to worry
            // about bounds or unicode boundaries.
            return Some(lexer.split_token_unchecked(1, *kind));
        }
    }

    // If nothing else has matched, there is no trivial token available.
    None
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenTy};

    #[test]
    fn plus_and_plus_eq_tokens() {
        let mut plus = Lexer::new_test("+");
        let mut plus_eq = Lexer::new_test("+=");

        let plus_token = plus.next_token().unwrap();
        let plus_eq_token = plus_eq.next_token().unwrap();

        assert_eq!(plus.bytes_remaining(), 0);
        assert_eq!(plus_eq.bytes_remaining(), 0);
        assert_eq!(plus_token.variant, TokenTy::Plus);
        assert_eq!(plus_eq_token.variant, TokenTy::PlusEq);
    }

    #[test]
    fn plus_one_token() {
        let mut plus_one = Lexer::new_test("+1");
        let plus_token = plus_one.next_token().unwrap();
        assert_eq!(plus_one.bytes_remaining(), 1);
        assert_eq!(plus_token.variant, TokenTy::Plus);
        assert_eq!(plus_token.fragment.len(), 1);
    }
}
