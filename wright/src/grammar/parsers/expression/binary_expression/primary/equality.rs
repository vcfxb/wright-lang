use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::Expression;
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use crate::grammar::parsers::expression::binary_expression::operator::parse_equality_operator;
use nom::branch::alt;
use crate::grammar::parsers::expression::binary_expression::primary::relational::{relational, relational_primary};

pub fn equality_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((relational, relational_primary))(input)
}

/// Parse equality expression.
pub fn equality(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(equality_primary, parse_equality_operator)(input)
}