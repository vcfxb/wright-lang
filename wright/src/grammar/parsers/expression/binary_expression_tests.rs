use crate::grammar::model::Fragment;
use crate::grammar::ast::{BinaryExpression as Bx, BinaryOp, Expression};
use codespan::{FileId, Files};

fn setup(s: &str) -> (Files<String>, FileId) {
    let mut f = Files::new();
    let h = f.add("test", s.to_string());
    (f, h)
}

#[test]
fn simple() {
    use Expression::*;
    let (f, h) = setup("1 + 2");
    let frag = Fragment::new(&f, h);
    let res = Bx::parse(frag);
    if let Ok((rem, BinaryExpression(val))) = res {
        assert_eq!(rem.len(), 0);
        if let (NumLit(left), op, NumLit(right)) = (*val.left, val.op, *val.right) {
            assert_eq!(op, BinaryOp::Add);
            assert_eq!(left.inner, 1);
            assert_eq!(right.inner, 2);
        } else {
            assert!(false);
        }
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn associativity() {
    use Expression::*;
    let (f, h) = setup("1 + 2 - 3");
    let frag = Fragment::new(&f, h);
    let res = Bx::parse(frag);
    if let Ok((rem, BinaryExpression(val))) = res {
        assert_eq!(rem.len(), 0);
        if let (BinaryExpression(left), op, NumLit(right)) = (&*val.left, val.op, &*val.right) {
            assert_eq!(op, BinaryOp::Sub);
            assert_eq!(right.inner, 3);
            if let (NumLit(left), op, NumLit(right)) = (&*left.left, left.op, &*left.right) {
                assert_eq!(op, BinaryOp::Add);
                assert_eq!(left.inner, 1);
                assert_eq!(right.inner, 2);
            } else {
                eprintln!("{:#?}", left);
                assert!(false);
            }
        } else {
            eprintln!("{:#?}", val);
            assert!(false);
        }
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn precedence_left() {
    use Expression::*;
    let (f, h) = setup("1 * 2 - 3");
    let frag = Fragment::new(&f, h);
    let res = Bx::parse(frag);
    if let Ok((rem, BinaryExpression(val))) = res {
        assert_eq!(rem.len(), 0);
        if let (BinaryExpression(left), op, NumLit(right)) = (&*val.left, val.op, &*val.right) {
            assert_eq!(op, BinaryOp::Sub);
            assert_eq!(right.inner, 3);
            if let (NumLit(left), op, NumLit(right)) = (&*left.left, left.op, &*left.right) {
                assert_eq!(op, BinaryOp::Mul);
                assert_eq!(left.inner, 1);
                assert_eq!(right.inner, 2);
            } else {
                eprintln!("{:#?}", left);
                assert!(false);
            }
        } else {
            eprintln!("{:#?}", val);
            assert!(false);
        }
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn precedence_right() {
    use Expression::*;
    let (f, h) = setup("1 + 2 * 3");
    let frag = Fragment::new(&f, h);
    let res = Bx::parse(frag);
    if let Ok((rem, BinaryExpression(val))) = res {
        assert_eq!(rem.len(), 0);
        if let (NumLit(left), op, BinaryExpression(right)) = (&*val.left, val.op, &*val.right) {
            assert_eq!(op, BinaryOp::Add);
            assert_eq!(left.inner, 1);
            if let (NumLit(left), op, NumLit(right)) = (&*right.left, right.op, &*right.right) {
                assert_eq!(op, BinaryOp::Mul);
                assert_eq!(left.inner, 2);
                assert_eq!(right.inner, 3);
            } else {
                eprintln!("{:#?}", left);
                assert!(false);
            }
        } else {
            eprintln!("{:#?}", val);
            assert!(false);
        }
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}
