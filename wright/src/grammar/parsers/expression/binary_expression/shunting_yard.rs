use crate::grammar::ast::{Expression, BinaryOp, BinaryExpression};
use crate::grammar::model::Fragment;

/// Shunting yard algorithm for structuring binary expressions. Takes
pub fn shunting_yard<'s>(expressions: Vec<Expression<'s>>, ops: Vec<BinaryOp>, frag: Fragment<'s>) -> BinaryExpression<'s> {
    unimplemented!()
}