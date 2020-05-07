use crate::grammar::ast::NumLit;
use crate::grammar::parsers::testing::TestingContext;

#[test]
fn from_dec() {
    assert_eq!(NumLit::from_dec("1000").unwrap(), 1000);
}

#[test]
fn dec_primary() {
    TestingContext::with(&["1000"])
        .test_output_node(NumLit::dec_primary, 0, |n| assert_eq!(n, 1000));
}

#[test]
fn dec_passthrough() {
    TestingContext::with(&["1000"]).test_output(NumLit::parse, 0, |(rem, node)| {
        assert_eq!(rem.len(), 0);
        assert_eq!(node.inner, 1000);
    });
}

#[test]
fn hex() {
    TestingContext::with(&["0xCafE_babe "]).test_output(NumLit::parse, 0, |(rem, node)| {
        assert_eq!(rem.source(), " ");
        assert_eq!(node.inner, 0xcafebabe);
    });
}

#[test]
fn bin() {
    TestingContext::with(&["0b1010_1001_1001\t"]).test_output(NumLit::parse, 0, |(rem, node)| {
        assert_eq!(rem.source(), "\t");
        assert_eq!(node.inner, 0b1010_1001_1001);
    });
}
