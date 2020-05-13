use crate::grammar::ast::Parens;
use crate::grammar::model::HasSourceReference;
use crate::grammar::testing::TestingContext;
use crate::grammar::tracing::input::OptionallyTraceable;

#[test]
fn test_basic() {
    let ctx = TestingContext::with(&["(ident)"]);

    ctx.test_output(Parens::parse, 0, |(rem, node)| {
        rem.get_trace().unwrap().print().unwrap();
        assert_eq!(rem.len(), 0);
        assert_eq!(node.source, "(ident)");
        assert_eq!(node.inner.get_source_ref(), "ident");
    });
}
