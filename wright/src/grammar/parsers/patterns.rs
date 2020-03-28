use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::Pattern;
use crate::grammar::model::{Fragment, HasFragment};
use std::mem::discriminant;

pub(crate) mod underscore;

impl<'s> HasFragment<'s> for Pattern<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Pattern::*;
        match self {
            UnderscorePattern(p) => p.get_fragment(),
        }
    }
}

impl<'s> AstEq for Pattern<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        // shorthand fn
        fn aeq<T: AstEq>(a: &T, b: &T) -> bool {
            AstEq::ast_eq(a, b)
        }

        use Pattern::*;
        match (fst, snd) {
            (UnderscorePattern(a), UnderscorePattern(b)) => aeq(a, b),
            _ => unreachable!(),
        }
    }
}
