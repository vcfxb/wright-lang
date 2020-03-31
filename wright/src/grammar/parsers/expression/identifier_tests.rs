use crate::grammar::ast::Identifier;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;

#[test]
fn test_empty() {
    let (f, h) = setup("");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    assert!(result.is_err());
}

#[test]
fn test_single() {
    let (f, h) = setup("foo");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    if let Ok((_, id)) = result {
        assert_eq!(id.frag.source(), "foo");
    } else {
        assert!(false);
    }
}

#[test]
fn test_with_underscore() {
    let (f, h) = setup("foo_3bar");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    if let Ok((_, id)) = result {
        assert_eq!(id.frag.source(), "foo_3bar");
    } else {
        assert!(false);
    }
}

#[test]
fn test_with_underscore_start() {
    let (f, h) = setup("_3foo_3bar");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    if let Ok((_, id)) = result {
        assert_eq!(id.frag.source(), "_3foo_3bar");
    } else {
        assert!(false);
    }
}

#[test]
fn test_invalid_start() {
    let (f, h) = setup("3foo");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    assert!(result.is_err());
}

#[test]
fn test_keyword() {
    let (f, h) = setup("false");
    let frag = Fragment::new(&f, h);
    let result = Identifier::parse(frag);
    assert!(result.is_err());
}
