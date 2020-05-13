use crate::grammar::ast::Block;
use crate::grammar::model::{Fragment, HasSourceReference};
use crate::grammar::testing::TestingContext;

#[test]
fn test_empty_block() {
    let ctx = TestingContext::with(&["{}"]);
    ctx.test_output_node(Block::parse, 0, |block| {
        assert_eq!(block.source.source(), "{}");
        assert!(block.statements.is_empty());
        assert!(block.result.is_none());
    });
}

#[test]
fn test_discarded_expression() {
    let ctx = TestingContext::with(&["{2+4; } "]);
    ctx.test_output(Block::parse, 0, |(rem, node)| {
        assert_eq!(rem.source(), " ");
        assert_eq!(node.source, "{2+4; }");
        assert_eq!(node.statements.len(), 1);
        assert_eq!(node.statements[0].get_source_ref(), "2+4;");
        assert!(node.result.is_none());
    });
}

#[test]
fn test_unterminated() {
    TestingContext::with(&["{ "]).test_all_fail(Block::parse)
}
