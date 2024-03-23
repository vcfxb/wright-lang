//! Expression parsing in Wright source code.

use crate::parser::fragment::Fragment;

use self::primary::{PrimaryExpression, PrimaryExpressionParsingError};

use super::AstNode;

pub mod primary;

/// An expression in Wright source code is anything that can be evaluated to a value.
pub enum Expression<'src> {
    Primary(PrimaryExpression<'src>),
}

/// An error that occurs while parsing an expression.
#[derive(Debug, Clone)]
pub enum ExpressionParsingError<'src> {
    /// An expression was expected but not found.
    ExpectedExpression {
        /// Where the expression was expected.
        at: Fragment<'src>,
    },

    /// An error parsing a primary expression not caused by inavailability.
    PrimaryExpressionParsingError(PrimaryExpressionParsingError<'src>),
}

impl<'src> AstNode<'src> for Expression<'src> {
    type Error = ExpressionParsingError<'src>;

    fn fragment(&self) -> Fragment<'src> {
        match self {
            Expression::Primary(primary) => primary.fragment(),
        }
    }

    fn try_parse(ctx: &mut super::AstGeneratorContext<'src>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // We need to go in reverse order of strength here (from weakest to strongest) to avoid under parsing.
        // (i.e. parsing a primary when the primary expression was the left side of a binary expression).

        // Try parsing a binary expression.
        match PrimaryExpression::try_parse(ctx) {
            // If we get a primary, early return it.
            Ok(primary) => return Ok(Expression::Primary(primary)),

            // If we get an error that is not unavailability, return it early too.
            Err(err @ PrimaryExpressionParsingError::OtherIntegerLiteralParsingError(_)) => {
                return Err(ExpressionParsingError::PrimaryExpressionParsingError(err));
            }

            // If we get an error that is unavailability, just ignore it and keep going.
            Err(PrimaryExpressionParsingError::ExpectedPrimaryExpression { .. }) => {}
        }

        // If we get to the end of the function with no parse sucessful, we error.
        Err(ExpressionParsingError::ExpectedExpression {
            at: ctx.peek_fragment(),
        })
    }
}
