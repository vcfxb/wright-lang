use crate::grammar::ast::{BinaryOp, Expression};
use crate::grammar::model::Fragment;
use codespan::{FileId, Files};

fn setup(val: &'static str) -> (Files<String>, FileId) {
    let mut files: Files<String> = Files::new();
    let h = files.add("val", val.to_string());
    (files, h)
}

#[test]
#[ignore] // remove when expressions are implemented
fn bin_op() {
    let (files, h) = setup("2 + 2");
    let frag = Fragment::new(&files, h);
    let (_, expr) = Expression::parse(frag).unwrap();
    if let Expression::BinaryExpression(expr) = expr {
        assert_eq!(expr.op, BinaryOp::Add);

        if let Expression::NumLit(num) = *expr.left {
            assert_eq!(num.inner, 2);
        } else {
            panic!("left operand not num lit")
        }

        if let Expression::NumLit(num) = *expr.right {
            assert_eq!(num.inner, 2);
        } else {
            panic!("right operand not num lit")
        }
    } else {
        panic!("not a binary expression")
    }
}
