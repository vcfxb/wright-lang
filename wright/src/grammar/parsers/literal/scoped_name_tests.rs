use crate::grammar::ast::ScopedName;
use crate::grammar::testing::TestingContext;
use crate::grammar::model::HasFragment;

#[test]
fn test_empty() {
    TestingContext::with(&[""]).test_all_fail(ScopedName::parse)
}

#[test]
fn test_single() {
    TestingContext::with(&["foo"]).test_output_node(ScopedName::parse, 0, |sn| {
        assert_eq!(sn.path.len(), 0);
        assert_eq!(sn.name.source.source(), "foo");
    })
}

#[test]
fn test_multiple() {
    TestingContext::with(&["foo::bar :: baz"]).test_output_node(ScopedName::parse, 0, |sn| {
        assert_eq!(sn.path.len(), 2);
        assert_eq!(sn.path[0].frag.source(), "foo");
        assert_eq!(sn.path[1].frag.source(), "bar");
        assert_eq!(sn.name.source.source(), "baz");
    });
}

#[test]
fn test_delimiter() {
    TestingContext::with(&["::"]).test_all_fail(ScopedName::parse)
}

#[test]
fn test_trailing() {
    let ctx = TestingContext::with(&["foo::", "foo ::", "foo::1"]);

    ctx.test_output(ScopedName::parse, 0, |(remaining, node)| {
        assert_eq!(remaining.source(), "::");
        assert!(node.path.is_empty());
        assert_eq!(node.name.source.source(), "foo");
    });

    ctx.test_output(ScopedName::parse, 2, |(rem, node)| {
        assert_eq!(rem.source(), "::1");
        assert!(node.path.is_empty());
        assert_eq!(node.name.get_fragment_reference().source(), "foo");
    });

    ctx.test_output(ScopedName::parse, 0, |(remaining, node)| {
        assert_eq!(node.name.source.source(), "foo");
        assert_eq!(remaining.source(), " ::");
        assert_eq!(node.path.len(), 0);
    })
}

#[test]
fn test_leading() {
    TestingContext::with(&["::foo", ":: foo"]).test_all_fail(ScopedName::parse);
}

#[test]
fn test_with_whitespace() {
    assert!(
        TestingContext::with(&["foo::bar::baz::biz", "foo \n::bar :: baz\t\t::biz"])
            .ast_eq(ScopedName::parse, (0, 1))
    )
}
