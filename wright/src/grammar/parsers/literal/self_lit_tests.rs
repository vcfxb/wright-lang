use crate::grammar::ast::SelfLit;
use crate::grammar::model::HasSourceReference;
use crate::grammar::testing::TestingContext;

#[test]
fn test_self_lit() {
    let tcx = TestingContext::with(&["self", "self "]);

    tcx.test_output(SelfLit::parse, 0, |(remaining, node)| {
        assert_eq!(remaining.len(), 0);
        assert_eq!(node.get_source_ref().source(), "self");
    });

    tcx.test_output(SelfLit::parse, 1, |(remaining, node)| {
        assert_eq!(remaining.source(), " ");
        assert_eq!(node.get_source_ref().source(), "self");
    });
}
