use crate::grammar::ast::CharLit;
use crate::grammar::parsers::testing::TestingContext;

#[test]
fn body() {
    TestingContext::with(&["a", r#"\n"#]).test_all_succeed(CharLit::character_body);
}

#[test]
fn wrapper() {
    TestingContext::with(&["'a'"]).test_all_succeed(CharLit::character_wrapper);
}

fn test_char(s: &'static str, v: char) {
    let tcx = TestingContext::with(&[s]);
    let res = tcx.run_parser_on(0, CharLit::parse);
    let val = res.unwrap().1;
    assert_eq!(val.inner, v);
}

#[test]
fn basic_char() {
    test_char("'a'", 'a');
}

#[test]
fn unicode_char() {
    test_char("'♦'", '♦');
}

#[test]
fn newline_escape() {
    test_char(r"'\n'", '\n');
}

#[test]
fn byte_escape() {
    test_char(r"'\x69'", '\x69');
}

#[test]
fn unicode_escape() {
    test_char(r"'\u{2666}'", '\u{2666}');
}

#[test]
fn invalid_syntax() {
    TestingContext::with(&["'ab'", "'a", r"'\a'", r"'\xA'", r"'\u{}'", r"'\u{1234567}'"])
        .test_all_fail(CharLit::parse);
}
