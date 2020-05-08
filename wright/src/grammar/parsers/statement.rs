use crate::grammar::ast::eq::{ast_eq, AstEq};
use crate::grammar::ast::{ExpressionStatement, Statement};
use crate::grammar::model::{Fragment, HasFragment};
use nom::combinator::map;
use nom::IResult;
use std::mem::discriminant;

/// Expression statement parser.
pub(crate) mod expression_statement;

impl<'s> Statement<'s> {
    /// Statement terminator. Probably should never change.
    pub const TERMINATOR: char = ';';

    /// Parses any statement.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Statement<'s>> {
        map(ExpressionStatement::parse, Statement::ExpressionStatement)(input)
    }
}

impl<'s> AstEq for Statement<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        use Statement::*;
        // discriminant is a function from std::mem
        // (https://doc.rust-lang.org/std/mem/fn.discriminant.html)
        // that returns an opaque type represents which variant of an enum
        // a value uses.
        // this check allows us to return `unreachable!()` at the bottom of
        // the match block instead of false. This will help us to catch bugs at
        // testing time.
        if discriminant(fst) != discriminant(snd) {
            return false;
        }

        match (fst, snd) {
            (ExpressionStatement(a), ExpressionStatement(b)) => ast_eq(a, b),
            _ => unimplemented!(),
        }
    }
}

impl<'s> HasFragment<'s> for Statement<'s> {
    fn get_fragment_reference(&self) -> &Fragment<'s> {
        use Statement::*;
        match self {
            ExpressionStatement(s) => s.get_fragment_reference(),
        }
    }
}
