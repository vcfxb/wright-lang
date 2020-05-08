use crate::grammar::ast::Identifier;
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::testing::TestingContext;

fn test_ident(s: &'static str, should_err: bool) {
    let tcx = TestingContext::with(&[s]);
    let fr = tcx.get_fragment(0);
    let r = Identifier::parse(fr);
    if should_err {
        assert!(r.is_err());
        r.map_err(|e| {
            e.map(|t| {
                let fr: Fragment = t.0;
                assert_eq!(fr.source(), s);
            })
        })
        .unwrap_err();
    } else {
        assert!(r.is_ok());
        let o = r.unwrap();
        assert_eq!(o.0.len(), 0);
        assert_eq!(o.1.frag.source(), s);
    }
}

#[test]
fn test_empty() {
    test_ident("", true);
}

#[test]
fn test_underscore() {
    test_ident("_", true);
}

#[test]
fn test_reserved() {
    test_ident("true", true);
}

#[test]
fn test_idents() {
    ["abc", "a_bc", "n0", "xd"]
        .iter()
        .for_each(|s| test_ident(s, false))
}

#[test]
fn test_trailing() {
    TestingContext::with(&["variable "])
        .test_output(Identifier::parse, 0, |(rem, node)| {
            assert_eq!(rem.source(), " ");
            assert_eq!(node.get_fragment_reference().source(), "variable");
        })
}