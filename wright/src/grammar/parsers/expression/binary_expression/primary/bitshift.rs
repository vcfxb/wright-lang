use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::Expression;
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::parsers::expression::binary_expression::operator::parse_bitshift_operator;

/// Subexpressions of a bitshift expression.
pub fn bitshift_primary(input: Fragment) -> IResult<Fragment, Expression> {
    todo!()
}

/// Bitshift expression.
pub fn bitshift(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(bitshift_primary, parse_bitshift_operator)(input)
}