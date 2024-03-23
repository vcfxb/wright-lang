//! Primary expression parsing in Wright source code. 
//! 
//! Primary expressions are considered the atoms of most expressions most primary expressions are literals,
//! which cannot be broken up into sub-expressions. 

use crate::parser::{ast::AstNode, fragment::Fragment};
use self::integer_literal::{IntegerLiteral, IntegerLiteralParsingError};

pub mod integer_literal;

/// A primary expression in Wright source code. These are the atoms of expressions, and can be combined using operators
/// to form more complicated expressions. 
pub enum PrimaryExpression<'src> {
    IntegerLiteral(IntegerLiteral<'src>)
}

/// Error occuring when someone attempts to parse a primary expression and there is not one. 
#[derive(Clone, Debug)]
pub enum PrimaryExpressionParsingError<'src> {
    /// An attempt was made to parse a primary expression and there was not one available. 
    ExpectedPrimaryExpression {
        /// The location in source code where a primary expression was expected. 
        at: Fragment<'src>
    },

    /// An error in parsing an integer literal besides in-availability. 
    OtherIntegerLiteralParsingError(IntegerLiteralParsingError<'src>),
}

impl<'src> AstNode<'src> for PrimaryExpression<'src> {
    type Error = PrimaryExpressionParsingError<'src>;

    fn fragment(&self) -> Fragment<'src> {
        match self {
            PrimaryExpression::IntegerLiteral(integer_literal) => integer_literal.fragment(),
        }
    }

    fn try_parse(ctx: &mut crate::parser::ast::AstGeneratorContext<'src>) -> Result<Self, Self::Error>
    where Self: Sized 
    {
        // If it's a successful parse, return Ok. `num` errors also fast-return. If it's an unavailability 
        // error, keep trying other types of primary. 
        match IntegerLiteral::try_parse(ctx) {
            Ok(int_lit) => return Ok(PrimaryExpression::IntegerLiteral(int_lit)),

            Err(num_err @ IntegerLiteralParsingError::NumParsingError { .. }) => {
                return Err(PrimaryExpressionParsingError::OtherIntegerLiteralParsingError(num_err));
            },

            Err(IntegerLiteralParsingError::ExpectedIntegerLiteral { .. }) => {}
        }
        
        // If we get to the end of the function, it's an error. 
        Err(PrimaryExpressionParsingError::ExpectedPrimaryExpression { at: ctx.peek_fragment() })
    }
}
