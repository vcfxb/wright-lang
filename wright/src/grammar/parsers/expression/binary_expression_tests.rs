use crate::grammar::ast::{BinaryExpression, Expression, BinaryOp};
use crate::grammar::testing::TestingContext;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::model::HasSourceReference;
use std::time::Instant;

#[test]
fn check_whitespace_simple() {
    let tcx = TestingContext::with(&["2 + 2", "2+2"]);
    assert!(tcx.ast_eq(BinaryExpression::parse, (0, 1)));
}

#[test]
fn benchmark_trace_disabled_simple() {
    let tcx = TestingContext::with(&["6*8+1"]);
    let frag = tcx.get_fragment_trace_disabled(0);
    let start = Instant::now();
    let res = BinaryExpression::parse(frag.clone()).unwrap();
    println!("{} microseconds to parse {}.", (Instant::now() - start).as_micros(), frag.source());
}

#[test]
fn benchmark_trace_disabled_complex() {
    let tcx = TestingContext::with(&[
        r#"a|b*7+1/5 || a and f-"string" "#
    ]);
    let frag = tcx.get_fragment_trace_disabled(0);
    let start = Instant::now();
    let res = BinaryExpression::parse(frag.clone()).unwrap();
    println!("{} microseconds to parse {} without tracing.",
             (Instant::now() - start).as_micros(), frag.source()
    );
}

#[test]
fn test_simple() {
    let ctx = TestingContext::with(&[
        "2+6"
    ]);
    ctx.test_output(BinaryExpression::parse, 0, |(rem, node)| {
        rem.get_trace().unwrap().print();
        assert_eq!(node.op, BinaryOp::Add);
        assert_eq!(node.left.get_source_ref(), "2");
        assert_eq!(node.right.get_source_ref(), "6");
        assert_eq!(node.get_source_ref(), "2+6");
    })
}

#[test]
fn test_complicated_parse() {
    let ctx = TestingContext::with(&[
        "2*2+2-4+5/5"
    ]);

    ctx.test_output(BinaryExpression::parse, 0, |(rem, _)| {
        rem.get_trace().unwrap().print();
    });
}

#[test]
fn test_expensive_parse() {
    let ctx = TestingContext::with(&[
        r#"a or b || a * n&&b&&d-(5 mod "string") & 2 * 6"#
    ]);

    ctx.test_output(BinaryExpression::parse, 0, |(rem, node)| {
        rem.get_trace().unwrap().print();
    });
}

#[test]
fn test_single_expr() {
    let ctx = TestingContext::with(&[
        "2",
        "(a)",
        r#""string literal""#
    ]);

    // ctx.test_output(BinaryExpression::parse, 1, |(rem, _)|
    //     rem.get_trace().unwrap().print().unwrap()
    // );
}