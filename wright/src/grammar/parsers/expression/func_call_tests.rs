use crate::grammar::ast::FuncCall;
use crate::grammar::parsers::testing::TestingContext;

#[test]
fn test_empty() {
    TestingContext::with(&["", " ", "\n"])
        .test_all_fail(FuncCall::parse)
}

#[test]
fn test_underscore() {
    TestingContext::with(&["_()"])
        .test_all_fail(FuncCall::parse)
}

#[test]
fn test_reserved() {
    TestingContext::with(&["true()", "if()", "else()"])
        .test_all_fail(FuncCall::parse)
}

#[test]
fn test_calls() {
    TestingContext::with(&[
        "abc()",
        "a_bc(1, \"abc\")",
        "n0(a, b, c)",
        "xd(e)",
        "foo(bar(baz), baz(bar(), foo), bar)",
        "a(b(), c(), d())",
    ]).test_all_succeed(FuncCall::parse)
}
