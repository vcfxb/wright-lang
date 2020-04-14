use crate::grammar::ast::ScopedName;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;

#[test]
fn test_empty() {
    let (f, h) = setup("");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    assert!(result.is_err());
}

#[test]
fn test_single() {
    let (f, h) = setup("foo");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    if let Ok((_, scoped_name)) = result {
        assert_eq!(scoped_name.path.len(), 0);
        assert_eq!(scoped_name.name.frag.source(), "foo");
    } else {
        assert!(false);
    }
}

#[test]
fn test_multiple() {
    let (f, h) = setup("foo::bar :: baz");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    if let Ok((_, scoped_name)) = result {
        assert_eq!(scoped_name.path.len(), 2);
        assert_eq!(scoped_name.path[0].frag.source(), "foo");
        assert_eq!(scoped_name.path[1].frag.source(), "bar");
        assert_eq!(scoped_name.name.frag.source(), "baz");
    } else {
        assert!(false);
    }
}

#[test]
fn test_delimiter() {
    let (f, h) = setup("::");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    assert!(result.is_err());
}

#[test]
fn test_trailing() {
    let (f, h) = setup("foo::");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    if let Ok((remaining, scoped_name)) = result {
        assert_eq!(remaining.source(), "::");
        assert_eq!(scoped_name.path.len(), 0);
        assert_eq!(scoped_name.name.frag.source(), "foo");
    } else {
        assert!(false);
    }
}

#[test]
fn test_leading() {
    let (f, h) = setup("::foo");
    let frag = Fragment::new(&f, h);
    let result = ScopedName::parse(frag);
    assert!(result.is_err());
}
