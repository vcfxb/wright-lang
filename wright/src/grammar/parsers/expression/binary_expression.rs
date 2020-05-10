use crate::grammar::ast::{eq::AstEq, BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::expression::binary_expression::primary::parse_binary_expr;
use crate::grammar::tracing::trace_result;
use nom::IResult;

/// Operator parsing implementation.
mod operator;

/// Primary parsing functions used in manual recursive descent parsing.
pub(self) mod primary;

/// Re-export the base-primary for use in the general expression parser.
pub(super) use primary::atom;

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
    /// Despite the return type being `Expression`, this function should
    /// always return a binary expression.
    ///
    /// ## Operator precedence:
    /// Wright binary operators are parsed internally using a precedence
    /// climbing algorithm. The operator precedences are documented
    /// [here](https://github.com/Wright-Language-Developers/docs/blob/master/syntax/operator-precedence.md).
    pub fn parse(input: I) -> IResult<I, Expression<I>> {
        trace_result(
            Self::TRACE_NAME,
            parse_binary_expr(input.trace_start_clone(Self::TRACE_NAME)),
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
