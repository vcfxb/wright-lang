
use std::fmt::Debug;

use crate::grammar::lexer::*;
use codespan_reporting::diagnostic::*;
use codespan::Span;

/// Trait for any keywords in wright's grammar. Anything that
/// implements this will automatically have Parser implemented for it.
pub trait Keyword: Default + Debug + Clone + Copy + Eq {
    /// this parser's rule.
    const RULE: &'static str;
    /// The keyword to be parsed.
    const BASE: &'static str;
}

//impl<T: Keyword> Parser for T {
//    const RULE: &'static str = <T as Keyword>::RULE;
//
//    fn try_parse(lexer: &mut Lexer) -> Option<Token> {
//        let mut can_match: bool = true;
//        let start = lexer.get_index();
//        for (n, ch) in T::BASE.chars().enumerate() {
//            can_match &= lexer.lookahead(n) == ch && ch != '\0';
//        }
//        if can_match {
//            lexer.advance(T::BASE.len());
//            Some(Token::new_stateless(start, lexer.get_index(), T::RULE))
//        } else {
//            None
//        }
//    }
//
//    fn do_parse(lexer: &mut Lexer) -> Result<Token, Diagnostic> {
//        Self::try_parse(lexer).ok_or(
//            Diagnostic::new_error(
//                "PARSING ERROR",
//                Label::new(lexer.handle,
//                           Span::new(lexer.get_index(), (lexer.get_index().0)+1),
//                           format!("Expected {}.", Self::BASE)))
//            )
//    }
//}

macro_rules! keyword_parser {
    ($i: ident, $s: expr) => {
        #[allow(missing_docs)]
        #[allow(bad_style)]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
        pub struct $i;
        impl Keyword for $i {
            const RULE: &'static str = stringify!($i);
            const BASE: &'static str = $s;
        }
    };
}

keyword_parser!(CLASS, "class");

