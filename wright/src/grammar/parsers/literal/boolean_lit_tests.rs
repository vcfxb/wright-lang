use crate::grammar::ast::BooleanLit;
use crate::grammar::testing::TestingContext;

#[test]
fn test_bool_lit() {
    TestingContext::with(&["true", "false"]).test_all_succeed(BooleanLit::parse)
}
