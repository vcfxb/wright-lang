use crate::grammar::ast::ScopedName;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;
use crate::grammar::ast::AstEq;

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

#[test]
fn test_with_whitespace() {
    let (mut f, h1) = setup("foo::bar::baz::biz");
    let h2 = f.add("other", "foo \n::bar :: baz\t\t::biz".to_owned());
    let f1 = Fragment::new(&f, h1);
    let f2 = Fragment::new(&f, h2);
    let r1 = ScopedName::parse(f1).unwrap().1;
    let r2 = ScopedName::parse(f2).unwrap().1;
    assert!(AstEq::ast_eq(&r1, &r2));
}

