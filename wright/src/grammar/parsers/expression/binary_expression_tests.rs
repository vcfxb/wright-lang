use crate::grammar::parsers::testing::{test_should_succeed};
use crate::grammar::ast::{BinaryExpression, Expression};
use crate::grammar::model::Fragment;
use nom::IResult;


fn test_binary_expr_simple() {
    test_should_succeed(
        BinaryExpression::parse,
        "2 + 2",
        "",
        |expr| {
            true
        }
    )
}
