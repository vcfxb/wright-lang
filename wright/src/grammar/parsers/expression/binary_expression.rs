use crate::grammar::ast::{
    eq::AstEq, BinaryExpression, BinaryOp, Block, BooleanLit, CharLit, Conditional, Expression,
    Name, NumLit, Parens, SelfLit, StringLit,
};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// [Shunting Yard](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
/// algorithm implementation.
mod shunting_yard;
#[cfg(test)]
mod shunting_yard_tests;

/// Operator implementation stuff. Used with shunting yard.
pub mod operators;

impl<'s> BinaryExpression<'s> {
    fn new(
        frag: Fragment<'s>,
        left: impl ToExpression<'s>,
        op: BinaryOp,
        right: impl ToExpression<'s>,
    ) -> Self {
        Self {
            frag,
            left: Box::new(left.create_expr()),
            op,
            right: Box::new(right.create_expr()),
        }
    }

    /// Parse a binary expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        todo!("binary expression parser")
    }

    /// Parse a binary terminal symbol.
    pub fn primary(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression> {
        alt((
            map(Conditional::parse, Expression::Conditional),
            // commented out until implemented
            // map(UnaryExpression::parse, Expression::UnaryExpression),
            // map(IndexExpression::parse, Expression::IndexExpression),
            map(Block::parse, Expression::Block),
            map(Parens::parse, Expression::Parens),
            map(SelfLit::parse, Expression::SelfLit),
            map(StringLit::parse, Expression::StringLit),
            map(CharLit::parse, Expression::CharLit),
            map(NumLit::parse, Expression::NumLit),
            map(BooleanLit::parse, Expression::BooleanLit),
            map(Name::parse, Expression::Name),
        ))(input)
    }
}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for BinaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::BinaryExpression(self)
    }
}

impl<'s> AstEq for BinaryExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op
            && AstEq::ast_eq(&*fst.left, &*snd.left)
            && AstEq::ast_eq(&*fst.right, &*snd.right)
    }
}
