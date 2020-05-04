use crate::grammar::model::{HasFragment, Fragment};
use crate::grammar::ast::{Name, AstEq, Expression, ScopedName, Identifier};
use std::mem::discriminant;
use crate::grammar::parsers::expression::ToExpression;
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;

impl<'s> Name<'s> {
    /// Parse a name in source code. This tries to parse a scoped name, and
    /// if that fails, parses an identifier.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            map(ScopedName::parse, Name::ScopedName),
            map(Identifier::parse, Name::Identifier)
        ))(input)
    }
}

impl<'s> HasFragment<'s> for Name<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        use Name::*;
        match self {
            ScopedName(n) => n.get_fragment(),
            Identifier(n) => n.get_fragment(),
        }
    }
}

impl<'s> ToExpression<'s> for Name<'s> {
    fn create_expr(self) -> Expression<'s> {
        use Name::*;
        match self {
            Identifier(n) => n.create_expr(),
            ScopedName(n) => n.create_expr(),
        }
    }
}

impl<'s> AstEq for Name<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        // shorthand fn
        fn aeq<T: AstEq>(a: T, b: T) -> bool {
            AstEq::ast_eq(&a, &b)
        }

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

        use Name::*;
        match (fst, snd) {
            (Identifier(a), Identifier(b)) => aeq(a,b),
            (ScopedName(a), ScopedName(b)) => aeq(a,b),
            (_, _) => unimplemented!()
        }
    }
}