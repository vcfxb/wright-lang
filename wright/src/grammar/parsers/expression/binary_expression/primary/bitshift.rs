use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::parse_bitshift_operator;
use crate::grammar::parsers::expression::binary_expression::primary::arithmetic::{
    arithmetic1, arithmetic1_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use nom::branch::alt;
use nom::IResult;

/// Subexpressions of a bitshift expression.
pub fn bitshift_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((arithmetic1, arithmetic1_primary))(input)
}

/// Bitshift expression.
pub fn bitshift(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(bitshift_primary, parse_bitshift_operator)(input)
}
