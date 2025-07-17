//! Implementation related to parsing keywords and identifiers.

use super::{Lexer, token::Token, token::TokenTy};
use crate::source_tracking::fragment::Fragment;
use std::str::Chars;
use unicode_ident::{is_xid_continue, is_xid_start};

/// Try to match a fragment recognized to be an identifier or keyword to
/// a keyword or return [TokenTy::Identifier].
fn identifier_or_keyword(fragment: Fragment) -> TokenTy {
    use TokenTy::*;

    match fragment.as_str() {
        "record" => KwRecord,
        "type" => KwType,
        "enum" => KwEnum,
        "union" => KwUnion,
        "func" => KwFunc,
        "pure" => KwPure,
        "unsafe" => KwUnsafe,
        "naked" => KwNaked,
        "repr" => KwRepr,
        "impl" => KwImpl,
        "constrain"  => KwConstrain,
        "constraint" => KwConstraint,
        "references" => KwReferences,
        "trait" => KwTrait,
        "const" => KwConst,
        "where" => KwWhere,

        "use" => KwUse,
        "as" => KwAs,
        "mod" => KwMod,
        "pub" => KwPub,

        "if" => KwIf,
        "else" => KwElse,
        "match" => KwMatch,

        "for" => KwFor,
        "in" => KwIn,
        "while" => KwWhile,
        "loop" => KwLoop,

        "let" => KwLet,
        "var" => KwVar,

        "true" => KwTrue,
        "false" => KwFalse,

        "bool" => KwBool,
        "u8" => KwU8,
        "i8" => KwI8,
        "u16" => KwU16,
        "i16" => KwI16,
        "u32" => KwU32,
        "i32" => KwI32,
        "f32" => KwF32,
        "u64" => KwU64,
        "i64" => KwI64,
        "f64" => KwF64,
        "char" => KwChar,

        "_" => Underscore,

        _ => Identifier,
    }
}

/// Attempt to consume a keyword/[identifier](TokenTy::Identifier)/[underscore](TokenTy::Underscore) from the lexer.
pub fn try_consume_keyword_or_identifier(lexer: &mut Lexer) -> Option<Token> {
    // Get a character iterator that we can pull from.
    let mut chars: Chars = lexer.remaining.chars();
    // Get the next character from the iterator, consider it the first char of any potential match.
    // Make sure it's a valid identifier start (includes start to all keywords) or is an underscore.
    // If it does not exist or match predicates, return None.
    let next: char = chars.next().filter(|c| is_xid_start(*c) || *c == '_')?;
    // Store/track the number of bytes consumed so far.
    let mut bytes_consumed: usize = next.len_utf8();

    // Take remaining chars and add to sum.
    bytes_consumed += chars
        .take_while(|c| is_xid_continue(*c))
        .map(char::len_utf8)
        .sum::<usize>();

    // Split the token and the new remaining fragment.
    // VALIDITY: The character iterator should guarantee that we land on a valid character boundary within the bounds
    // of the fragment.
    let (token_fragment, new_remaining): (Fragment, Fragment) =
        lexer.remaining.split_at_unchecked(bytes_consumed);

    // Get the variant of token to produce.
    let variant: TokenTy = identifier_or_keyword(token_fragment.clone());

    // Update the lexer's remaining fragment.
    lexer.remaining = new_remaining;

    // Return the token.
    Some(Token {
        variant,
        fragment: token_fragment,
    })
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenTy};

    #[test]
    fn identifiers_and_keywords() {
        let mut lexer = Lexer::new_test("const TEST");

        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::KwConst);
        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::Whitespace);
        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::Identifier);
    }
}
