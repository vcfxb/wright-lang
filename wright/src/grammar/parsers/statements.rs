use std::mem::discriminant;
use crate::grammar::ast::eq::ASTEq;
use crate::grammar::ast::Statement;
use crate::grammar::model::{HasFragment, Fragment};

/// Expression statement parser.
pub(crate) mod expression_statement;

impl<'s> Statement<'s> {
    /// Semicolon in source code. Probably should never change.
    pub const SEMICOLON: char = ';';

}

impl<'s> ASTEq for Statement<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        use Statement::*;

        // shorthand fn
        fn aeq<T: ASTEq>(a: &T, b: &T) -> bool {ASTEq::ast_eq(a, b)}

        // discriminant is a function from std::mem
        // (https://doc.rust-lang.org/std/mem/fn.discriminant.html)
        // that returns an opaque type represents which variant of an enum
        // a value uses.
        // this check allows us to return `unimplemented!()` at the bottom of
        // the match block instead of false. This will help us to catch bugs at
        // testing time.
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        match (fst, snd) {
            (ExpressionStatement(a), ExpressionStatement(b)) => aeq(a,b),
            _ => unimplemented!(),
        }
    }
}

impl<'s> HasFragment<'s> for Statement<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Statement::*;
        match self {
            ExpressionStatement(s) => s.get_fragment()
        }
    }
}