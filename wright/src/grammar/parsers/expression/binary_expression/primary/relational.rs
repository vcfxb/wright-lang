use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::Expression;
use crate::grammar::parsers::expression::binary_expression::primary::parser_left;
use nom::branch::alt;
use crate::grammar::parsers::expression::binary_expression::operator::parse_relational_operator;

/// Parser for sub expressions of a relational expression.
pub fn relational_primary(input: Fragment) -> IResult<Fragment, Expression> {
    todo!()
}

/// Parse a relational expression.
pub fn relational(input: Fragment) -> IResult<Fragment, Expression> {
    parser_left(relational_primary, parse_relational_operator)(input)
}