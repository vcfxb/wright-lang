use crate::grammar::ast::{eq::AstEq, BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::expression::binary_expression::primary::parse_binary_expr;
use crate::grammar::tracing::{
    trace_result,
    parsers::map,
};
use nom::IResult;
use nom::combinator::verify;

/// Operator parsing implementation.
mod operator;

/// Primary parsing functions used in manual
/// precedence climbing parsing.
pub mod primary;


impl<T: Clone + std::fmt::Debug> BinaryExpression<T> {
    /// The name of this expression when it appears in traces.
    pub const TRACE_NAME: &'static str = "BinaryExpression";
}

impl<I: WrightInput> BinaryExpression<I> {
    fn new(
        source: I,
        left: impl Into<Expression<I>>,
        op: BinaryOp,
        right: impl Into<Expression<I>>,
    ) -> Self {
        Self {
            source,
            left: Box::new(left.into()),
            op,
            right: Box::new(right.into()),
        }
    }

    /// Parse a binary expression in source code.
    ///
    /// ## Operator precedence:
    /// Wright binary operators are parsed internally using a precedence
    /// climbing algorithm. The operator precedences are documented
    /// [here](https://github.com/Wright-Language-Developers/docs/blob/master/syntax/operator-precedence.md).
    pub fn parse(input: I) -> IResult<I, BinaryExpression<I>> {
        trace_result(
            Self::TRACE_NAME,
            map(
                verify(
                    parse_binary_expr,
                    |node| {
                        if let Expression::BinaryExpression(_) = node {
                            true
                        } else {false}
                    }
                ),
                |expr| {
                    if let Expression::BinaryExpression(e) = expr {
                        e
                    } else {unreachable!()}
                }
            )(input.trace_start_clone(Self::TRACE_NAME))
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for BinaryExpression<I> {
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for BinaryExpression<I> {
    fn into(self) -> Expression<I> {
        Expression::BinaryExpression(self)
    }
}

impl<T: Clone + std::fmt::Debug + PartialEq> AstEq for BinaryExpression<T> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op
            && AstEq::ast_eq(&*fst.left, &*snd.left)
            && AstEq::ast_eq(&*fst.right, &*snd.right)
    }
}
