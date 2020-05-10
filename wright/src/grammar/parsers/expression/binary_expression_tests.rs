use crate::grammar::ast::BinaryExpression;
use crate::grammar::testing::TestingContext;

#[test]
fn check_whitespace_simple() {
    let tcx = TestingContext::with(&["2 + 2", "2+2"]);
    assert!(tcx.ast_eq(BinaryExpression::parse, (0, 1)));
}
