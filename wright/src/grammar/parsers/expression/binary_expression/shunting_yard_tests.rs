use crate::grammar::ast::{BinaryExpression, BinaryOp};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::shunting_yard::shunting_yard;
use crate::grammar::parsers::testing::setup;

#[test]
fn test_simple() {
    let (f, h) = setup("2 + 2");
    let frag = Fragment::new(&f, h);
    let res = shunting_yard(frag);
    assert!(res.is_ok());
    let binexp = res.unwrap().1;
    assert_eq!(binexp.frag.source(), "2 + 2");
    assert_eq!(binexp.op, BinaryOp::Add);
}
