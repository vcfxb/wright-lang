use crate::grammar::ast::StringLit;
use crate::grammar::parsers::testing::TestingContext;

fn do_test(s: &'static str, r: &'static str, o: &'static str) {
    let tcx = TestingContext::with(&[s]);
    let fr = tcx.get_fragment(0);
    let (rem, out) = StringLit::parse(fr).unwrap();
    assert_eq!(out.inner, o);
    assert_eq!(rem.source(), r);
}

#[test]
fn test_simple() {
    do_test(r#""Hello, World""#, r"", r"Hello, World")
}

#[test]
fn test_null_escape() {
    do_test(r#""Null\0 character""#, r"", "Null\0 character")
}

#[test]
fn test_unterminated() {
    TestingContext::with(&[
        r#""simple"#,
        r#""escaped ending \""#,
    ]).test_all_fail(StringLit::parse)
}
