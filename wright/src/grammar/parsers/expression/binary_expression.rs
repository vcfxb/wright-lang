use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression, eq::ASTEq};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::IResult;

/// [Shunting Yard](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
/// algorithm implementation.
mod shunting_yard;

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

impl<'s> ASTEq for BinaryExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {fst.op == snd.op && ASTEq::ast_eq(&*fst.left, &*snd.left)}
}