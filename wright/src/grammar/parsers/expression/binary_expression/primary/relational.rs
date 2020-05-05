use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operator::parse_relational_operator;
use crate::grammar::parsers::expression::binary_expression::primary::bitshift::{
    bitshift, bitshift_primary,
};
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use nom::branch::alt;
use nom::IResult;

/// Parser for sub expressions of a relational expression.
pub fn relational_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((bitshift, bitshift_primary))(input)
}

/// Parse a relational expression.
pub fn relational(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(relational_primary, parse_relational_operator)(input)
}
