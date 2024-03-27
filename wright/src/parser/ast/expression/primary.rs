//! Primary expression parsing in Wright source code.
//!
//! Primary expressions are considered the atoms of most expressions most primary expressions are literals,
//! which cannot be broken up into sub-expressions.

use self::{integer_literal::{IntegerLiteral, IntegerLiteralParsingError}, parens::{ParensExpression, ParensParsingError}};
use crate::parser::{ast::AstNode, fragment::Fragment};

pub mod integer_literal;
pub mod parens;

/// A primary expression in Wright source code. These are the atoms of expressions, and can be combined using operators
/// to form more complicated expressions.
#[derive(Debug)]
pub enum PrimaryExpression<'src> {
    /// An integer literal in wright source code. 
    IntegerLiteral(IntegerLiteral<'src>),

    /// An expression in parentheses.
    ParensExpression(ParensExpression<'src>),
}

/// Error occuring when someone attempts to parse a primary expression and there is not one.
#[derive(Clone, Debug)]
pub enum PrimaryExpressionParsingError<'src> {
    /// An attempt was made to parse a primary expression and there was not one available.
    ExpectedPrimaryExpression {
        /// The location in source code where a primary expression was expected.
        at: Fragment<'src>,
    },

    /// An error in parsing an integer literal besides in-availability.
    OtherIntegerLiteralParsingError(IntegerLiteralParsingError<'src>),

    /// An error parsing an expression in parentheses besides lack of an opening parenthese. 
    OtherParensExpressionParsingError(ParensParsingError<'src>),
}

impl<'src> AstNode<'src> for PrimaryExpression<'src> {
    type Error = PrimaryExpressionParsingError<'src>;

    fn fragment(&self) -> Fragment<'src> {
        match self {
            PrimaryExpression::IntegerLiteral(integer_literal) => integer_literal.fragment(),
            PrimaryExpression::ParensExpression(parens_expr) => parens_expr.fragment(),
        }
    }

    #[rustfmt::skip] // Do not auto-reformat this block -- the match arms get too mangled. 
    fn try_parse(
        ctx: &mut crate::parser::ast::AstGeneratorContext<'src>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // If it's a successful parse, return Ok. `num` errors also fast-return. If it's an unavailability
        // error, keep trying other types of primary.
        match IntegerLiteral::try_parse(ctx) {
            Ok(int_lit) => return Ok(PrimaryExpression::IntegerLiteral(int_lit)),

            Err(num_err @ IntegerLiteralParsingError::NumParsingError { .. }) => {
                return Err(PrimaryExpressionParsingError::OtherIntegerLiteralParsingError(
                    num_err,
                ));
            }

            Err(IntegerLiteralParsingError::ExpectedIntegerLiteral { .. }) => {}
        }

        // Do the same with a parens expression. 
        match ParensExpression::try_parse(ctx) {
            Ok(parens_expr) => return Ok(PrimaryExpression::ParensExpression(parens_expr)),

            Err(err @ (
                | ParensParsingError::ClosingParenNotFound { .. }  
                | ParensParsingError::ErrorInParentheses { .. }
            )) => return Err(PrimaryExpressionParsingError::OtherParensExpressionParsingError(err)),

            // Do nothing -- try parsing other primaries, or let this become an "expected primary expression error". 
            Err(ParensParsingError::ExpectedParensExpression { .. }) => {}
        }

        // If we get to the end of the function, it's an error.
        Err(PrimaryExpressionParsingError::ExpectedPrimaryExpression {
            at: ctx.peek_fragment(),
        })
    }
}
