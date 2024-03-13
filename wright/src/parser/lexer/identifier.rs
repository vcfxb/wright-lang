//! Implementation related to parsing keywords and identifiers.

use super::{token::Token, token::TokenTy, Lexer};
use crate::parser::fragment::Fragment;
use std::str::Chars;
use unicode_ident::{is_xid_continue, is_xid_start};

/// Try to match a fragment recognized to be an identifier or keyword to
/// a keyword or return [TokenTy::Identifier].
fn identifier_or_keyword(fragment: Fragment) -> TokenTy {
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

/// Attempt to consume a keyword/[identifier](TokenTy::Identifier)/[underscore](TokenTy::Underscore) from the lexer.
pub fn try_consume_keyword_or_identifier<'src>(lexer: &mut Lexer<'src>) -> Option<Token<'src>> {
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
    // SAFETY: The character iterator should guaruntee that we land on a valid character boundary within the bounds
    // of the fragment.
    let (token_fragment, new_remaining): (Fragment, Fragment) =
        unsafe { lexer.remaining.split_at_unchecked(bytes_consumed) };

    // Get the variant of token to produce.
    let variant: TokenTy = identifier_or_keyword(token_fragment);
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
        let mut lexer = Lexer::new("const TEST");

        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::KwConst);
        assert_eq!(lexer.next_token().unwrap().variant, TokenTy::Identifier);
    }
}
