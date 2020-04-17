use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::operators::OperatorInfo;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::IResult;

fn binary_operator<'s>(input: Fragment<'s>) -> IResult<Fragment<'s>, OperatorInfo> {
    use BinaryOp::*;
    map(
        alt((
            value(OrOr, tag("||")),
            value(AndAnd, tag("&&")),
            value(Or, tag("|")),
            value(Xor, tag("^")),
            value(And, tag("&")),
            value(EqEq, tag("==")),
            value(NotEq, tag("!=")),
            value(Le, tag("<=")),
            value(Ge, tag(">=")),
            value(Lt, tag("<")),
            value(Gt, tag(">")),
            value(DotDot, tag("..")),
            value(Add, tag("+")),
            value(Sub, tag("-")),
            value(Mul, tag("*")),
            value(Div, tag("/")),
            value(Mod, tag("%")),
        )),
        BinaryOp::get_info,
    )(input)
}

/// Shunting yard algorithm for structuring binary expressions. Takes
pub fn shunting_yard<'s>(
    expressions: Vec<Expression<'s>>,
    ops: Vec<BinaryOp>,
    frag: Fragment<'s>,
) -> BinaryExpression<'s> {
    unimplemented!()
}
