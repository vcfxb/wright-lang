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
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::Err::Error;
use nom::IResult;

fn binary_operator(input: Fragment) -> IResult<Fragment, BinaryOp> {
    use BinaryOp::*;
    let get_token = |op: BinaryOp| op.get_info().token;
    let value_tag = |op: BinaryOp| value(op, tag(get_token(op)));
    delimited(
        token_delimiter,
        alt((
            // some of these are sequentially important; AndAnd needs to go before And
            // to avoid under-parsing.
            value_tag(OrOr),
            value_tag(Or),
            value_tag(AndAnd),
            value_tag(And),
            value_tag(Ge),
            value_tag(Gt),
            value_tag(Le),
            value_tag(Lt),
            value_tag(Xor),
            value_tag(NotEq),
            value_tag(EqEq),
            value_tag(DotDot),
            value_tag(Add),
            value_tag(Sub),
            value_tag(Mul),
            value_tag(Div),
            value_tag(Mod),
        )),
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
    // parse the first operand
    if let Ok((rem1, operand)) = BinaryExpression::primary(rem) {
        rem = rem1;
        exprs.push(operand);
        // parse the operator chain
        while let Ok((rem1, operator)) = binary_operator(rem) {
            // shunt operators of greater precedence over
            let operator = operator.get_info();
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
            if let Ok((rem1, operand)) = BinaryExpression::primary(rem1) {
                // commit consuming the source of (operator + second operand)
                rem = rem1;
                ops.push(operator);
                exprs.push(operand);
            } else {
                // don't comsume the source of (operator)
                break;
            }
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
