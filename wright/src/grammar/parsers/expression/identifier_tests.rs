use crate::grammar::parsers::testing::setup;
use crate::grammar::model::Fragment;
use crate::grammar::ast::Identifier;
use nom::error::ErrorKind;

fn test_ident(s: &'static str, should_err: bool) {
    let (f, h) = setup(s);
    let fr = Fragment::new(&f, h);
    let r = Identifier::parse(fr);
    if should_err {
        assert!(r.is_err());
        r.map_err(|e| e.map(|t| {
            let fr: Fragment = t.0;
            assert_eq!(fr.source(), s);
        }));
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
    [
        "abc",
        "a_bc",
        "n0",
        "xd"
    ].iter()
        .for_each(|s| test_ident(s, false))
}