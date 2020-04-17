use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::binary_expression::operators::{
    Associativity, OperatorInfo,
};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::sequence::delimited;
use nom::error::ErrorKind;
use nom::Err::Error;
use nom::IResult;

fn binary_operator<'s>(input: Fragment<'s>) -> IResult<Fragment<'s>, OperatorInfo> {
    use BinaryOp::*;
    delimited(
        token_delimiter,
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
        ),
        token_delimiter,
    )(input)
}

fn build_binary_expr<'s>(a: Expression<'s>, op: OperatorInfo, b: Expression<'s>) -> Expression<'s> {
    BinaryExpression::new(
        Fragment::merge(a.get_fragment(), b.get_fragment()).expect("Fragments must be contiguous"),
        a,
        op.id,
        b,
    )
    .create_expr()
}

/// Shunting yard algorithm for structuring binary expressions. Takes
pub fn shunting_yard<'s>(input: Fragment<'s>) -> IResult<Fragment<'s>, BinaryExpression<'s>> {
    let mut exprs = Vec::new();
    let mut ops = Vec::<OperatorInfo>::new();
    // expose remaining input at top-level
    let mut rem = input;
    while let Ok((rem1, operand)) = BinaryExpression::primary(rem) {
        rem = rem1;
        exprs.push(operand);
        if let Ok((rem1, operator)) = binary_operator(rem1) {
            rem = rem1;
            // shunt operators of greater precedence over
            while let Some(top) = ops.pop() {
                use Associativity::Left;
                if top.prec > operator.prec || (top.prec == operator.prec && operator.assoc == Left)
                {
                    // build BinaryExpression and push onto exprs
                    let b = exprs.pop().expect("(b1) exprs stack must be len >= 2");
                    let a = exprs.pop().expect("(a1) exprs stack must be len >= 2");
                    exprs.push(build_binary_expr(a, top, b));
                } else {
                    // repush popped operator, not shunting
                    ops.push(top);
                    break;
                }
            }
            ops.push(operator);
        } else {
            // no operator, stop parsing
            break;
        }
    }
    // shunt over remaining operators
    while let Some(op) = ops.pop() {
        let b = exprs.pop().expect("(b2) exprs stack must be len >= 2");
        let a = exprs.pop().expect("(a2) exprs stack must be len >= 2");
        exprs.push(build_binary_expr(a, op, b));
    }
    if let Some(Expression::BinaryExpression(expr)) = exprs.pop() {
        Ok((rem, expr))
    } else {
        Err(Error((input, ErrorKind::SeparatedNonEmptyList)))
    }
}
