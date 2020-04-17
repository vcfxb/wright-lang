use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::shunting_yard::shunting_yard;
use crate::grammar::parsers::testing::setup;

fn ensure_number(expr: Expression, value: u128, err: &'static str) {
    match expr {
        Expression::NumLit(n) => {
            assert_eq!(n.inner, value);
        }
        _ => panic!(err),
    }
}

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

#[test]
fn test_multiple_addition() {
    use Expression::BinaryExpression as Bxp;
    let (f, h) = setup("1 + 2 + 3");
    let frag = Fragment::new(&f, h);
    let res = shunting_yard(frag);
    assert!(res.is_ok());
    let binexp = res.unwrap().1;
    assert_eq!(binexp.frag.source(), "1 + 2 + 3");
    assert_eq!(binexp.op, BinaryOp::Add);
    ensure_number(*binexp.right, 3, "Right not a number");
    match *binexp.left {
        Bxp(left) => {
            assert_eq!(left.frag.source(), "1 + 2");
            assert_eq!(left.op, BinaryOp::Add);
            ensure_number(*left.left, 1, "Left of left not a number");
            ensure_number(*left.right, 2, "Right of left not a number");
        }
        _ => panic!("Left not a binary expression"),
    }
}

#[test]
fn test_addition_multiplication() {
    use Expression::BinaryExpression as Bxp;
    let (f, h) = setup("1 + 2 * 3");
    let frag = Fragment::new(&f, h);
    let res = shunting_yard(frag);
    assert!(res.is_ok());
    let binexp = res.unwrap().1;
    assert_eq!(binexp.frag.source(), "1 + 2 * 3");
    assert_eq!(binexp.op, BinaryOp::Add);
    ensure_number(*binexp.left, 1, "Left not a number");
    match *binexp.right {
        Bxp(right) => {
            assert_eq!(right.frag.source(), "2 * 3");
            assert_eq!(right.op, BinaryOp::Mul);
            ensure_number(*right.left, 2, "Left of right not a number");
            ensure_number(*right.right, 3, "Right of right not a number");
        }
        _ => panic!("Right not a binary expression"),
    }
}

#[test]
fn test_multiplication_addition() {
    use Expression::BinaryExpression as Bxp;
    let (f, h) = setup("1 * 2 + 3");
    let frag = Fragment::new(&f, h);
    let res = shunting_yard(frag);
    assert!(res.is_ok());
    let binexp = res.unwrap().1;
    assert_eq!(binexp.frag.source(), "1 * 2 + 3");
    assert_eq!(binexp.op, BinaryOp::Add);
    ensure_number(*binexp.right, 3, "Right not a number");
    match *binexp.left {
        Bxp(left) => {
            assert_eq!(left.frag.source(), "1 * 2");
            assert_eq!(left.op, BinaryOp::Mul);
            ensure_number(*left.left, 1, "Left of left not a number");
            ensure_number(*left.right, 2, "Right of left not a number");
        }
        _ => panic!("Left not a binary expression"),
    }
}

#[test]
fn test_subtraction_addition() {
    use Expression::BinaryExpression as Bxp;
    let (f, h) = setup("1 - 2 + 3");
    let frag = Fragment::new(&f, h);
    let res = shunting_yard(frag);
    assert!(res.is_ok());
    let binexp = res.unwrap().1;
    assert_eq!(binexp.frag.source(), "1 - 2 + 3");
    assert_eq!(binexp.op, BinaryOp::Add);
    ensure_number(*binexp.right, 3, "Right not a number");
    match *binexp.left {
        Bxp(left) => {
            assert_eq!(left.frag.source(), "1 - 2");
            assert_eq!(left.op, BinaryOp::Sub);
            ensure_number(*left.left, 1, "Left of left not a number");
            ensure_number(*left.right, 2, "Right of left not a number");
        }
        _ => panic!("Left not a binary expression"),
    }
}
