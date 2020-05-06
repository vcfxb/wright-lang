use crate::grammar::parsers::testing::run_test;
// use crate::grammar::parsers::testing::{test_should_succeed};
use crate::grammar::ast::{BinaryExpression, Expression};
use crate::grammar::model::Fragment;
use nom::IResult;


// #[test]
// fn test_binary_expr_simple() {
//     //let p = BinaryExpression::parse as for<'a> fn(Fragment<'a>) -> IResult<Fragment<'a>, Expression<'a>>;
//     test_should_succeed::<Expression, _>(
//         p as for<'a> fn(Fragment<'a>) -> IResult<Fragment<'a>, Expression<'a>>,
//         "2 + 2",
//         "",
//         |expr| {
//             true
//         }
//     )
// }

// fn test_binary_expr_simple() {
//     run_test(
//         BinaryExpression::parse,
//         "2+2",
//         false,
//         Some(""),
//         |_: &Expression| true
//     )
// }