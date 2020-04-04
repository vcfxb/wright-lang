use crate::grammar::parsers::testing::setup;
use crate::grammar::model::Fragment;
use crate::grammar::ast::Block;

#[test]
#[ignore] // waiting for expression parsing to be implemented
fn test_empty_block() {
    let (f, h) = setup("{} ");
    let fr = Fragment::new(&f, h);
    let res = Block::parse(fr);
    assert!(res.is_ok());
    let (remaining, block) = res.unwrap();
    assert_eq!(remaining.len(), 1);
    assert!(block.result.is_none());
    assert!(block.statements.is_empty());
    assert_eq!(block.frag.len(), 2);
}