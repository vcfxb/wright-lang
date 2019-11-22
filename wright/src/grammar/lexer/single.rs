use crate::grammar::lexer::{Parser, Lexer, Token};
use codespan_reporting::diagnostic::*;
use codespan::Span;
use std::fmt::Debug;

/// Trait for any single character parsers in wright's grammar. Anything that
/// implements this will automatically have Parser implemented for it.
pub trait SingleChar: Default + Debug + Clone + Copy + Eq {
    /// this parser's rule.
    const RULE: &'static str;
    /// The character to be parsed.
    const BASE: char;
}

impl<T: SingleChar> Parser for T {
    const RULE: &'static str = <T as SingleChar>::RULE;

    fn try_parse(lexer: &mut Lexer) -> Option<Token> {
        let start = lexer.get_index();
        if lexer.matches(T::BASE) {
            Some(Token::new_stateless(start, lexer.get_index(), Self::RULE))
        } else { None }
    }

    fn do_parse(lexer: &mut Lexer) -> Result<Token, Diagnostic> {
        Self::try_parse(lexer).ok_or_else(|| {
            Diagnostic::new_error(
                "PARSING ERROR",
                Label::new(lexer.handle,
                           Span::new(lexer.get_index(), (lexer.get_index().0)+1),
                           format!("Could not parse with rule {}.", Self::RULE))
            )
        })
    }

}

macro_rules! single_char_parser {
    ($i: ident, $b: expr) => {
        #[allow(missing_docs)]
        #[allow(bad_style)]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
        pub struct $i;
        impl SingleChar for $i {
            const RULE: &'static str = stringify!($i);
            const BASE: char = $b;
        }
    };
}

single_char_parser!(ADD, '+');
single_char_parser!(SUB, '-');
single_char_parser!(MUL, '*');
single_char_parser!(DIV, '/');
single_char_parser!(COMMA, ',');
single_char_parser!(DOT, '.');
single_char_parser!(L_PAREN, '(');
single_char_parser!(R_PAREN, ')');
single_char_parser!(L_BRACKET, '[');
single_char_parser!(R_BRACKET, ']');
single_char_parser!(L_CURLY, '{');
single_char_parser!(R_CURLY, '}');
single_char_parser!(MOD, '%');
single_char_parser!(AT, '@');
single_char_parser!(BANG, '!');
single_char_parser!(POUND, '#');
single_char_parser!(CASH, '$');
single_char_parser!(CARROT, '^');
single_char_parser!(AND, '&');
single_char_parser!(UNDERSCORE, '_');
single_char_parser!(EQUALS, '=');
single_char_parser!(BACKSLASH, '\\');
single_char_parser!(COLON, ':');
single_char_parser!(SEMICOLON, ';');
single_char_parser!(LT, '<');
single_char_parser!(GT, '>');
single_char_parser!(CHAR_QUOTE, '\'');
single_char_parser!(STRING_QUOTE, '\"');
single_char_parser!(TILDE, '~');
