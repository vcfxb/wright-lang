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
use nom::sequence::{delimited, pair};
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

/// Shunting yard algorithm for structuring binary expressions.
///
/// The parser structure can be expressed in pseudo-code as
/// ```text
/// parse_primary;
/// while (op, primary) = (parse_operator, parse_primary) {
///    shunt_operators;
/// }
/// shunt_remaining_ops;
/// return_binary_expr;
/// ```
pub fn shunting_yard<'s>(input: Fragment<'s>) -> IResult<Fragment<'s>, BinaryExpression<'s>> {
    // expression and operator stacks
    // the top (and only) element of the expression
    // stack is returned when this algorithm terminates
    let mut exprs = Vec::new();
    let mut ops = Vec::<OperatorInfo>::new();
    // expose remaining input at top-level. The remaining input is steadily consumed
    // within this parser and eventually returned to the client.
    let mut rem = input;
    // parse the first primary expression
    // if we can't parse a single primary, skip everything else
    let (rem1, operand) = BinaryExpression::primary(rem)?;
    // comsume the input used for the first primary
    rem = rem1;
    exprs.push(operand);
    // parse the operator chain
    while let Ok((rem1, (operator, operand))) =
        pair(binary_operator, BinaryExpression::primary)(rem)
    {
        // consume the input used for parsing the operator + operand pair
        rem = rem1;
        // shunt operators of greater precedence over
        while let Some(top) = ops.last() {
            use Associativity::Left;
            if top.prec > operator.prec || (top.prec == operator.prec && operator.assoc == Left) {
                // pop the operator off the operator stack
                let top = ops.pop().unwrap();
                // build BinaryExpression and push onto exprs
                let b = exprs.pop().expect("(b1) exprs stack must be len >= 2");
                let a = exprs.pop().expect("(a1) exprs stack must be len >= 2");
                exprs.push(build_binary_expr(a, top, b));
            } else {
                // no more operators to shunt
                break;
            }
        }
        // push parsed operator and operand to stacks
        // it is important to do this _after_ shunting operators
        ops.push(operator);
        exprs.push(operand);
    }
    // shunt over remaining operators still on the operator stack
    while let Some(op) = ops.pop() {
        let b = exprs.pop().expect("(b2) exprs stack must be len >= 2");
        let a = exprs.pop().expect("(a2) exprs stack must be len >= 2");
        exprs.push(build_binary_expr(a, op, b));
    }
    // return the remaining expression in the expression stack.
    // Note that this algorithm guarantees that there is at most
    // one expression on the stack at this point. If there is
    // an expression, and it is futhermore a binary expression,
    // it is returned; else, this parser errors.
    if let Some(Expression::BinaryExpression(expr)) = exprs.pop() {
        Ok((rem, expr))
    } else {
        Err(Error((input, ErrorKind::SeparatedNonEmptyList)))
    }
}
