use crate::grammar::ast::Block;
use crate::grammar::model::Fragment;
use crate::grammar::testing::setup;

fn block_test(
    src: &'static str,
    should_fail: bool,
    remaining: Option<&'static str>,
    block_params: impl Fn(Block) -> bool
) {
    let (f, h) = setup(src);
    let fr = Fragment::new(&f, h);
    let res = Block::parse(fr);
    if should_fail {
        assert!(res.is_err());
    } else {
        assert!(res.is_ok());
        let (rem, node) = res.unwrap();
        assert_eq!(rem.source(), remaining.unwrap());
        assert!(block_params(node));
    }
}

/// Block test (should succeed).
fn btss(
    src: &'static str,
    rem: &'static str,
    block_params: impl Fn(Block) -> bool
) {block_test(src, false, Some(rem), block_params)}

#[test]
fn test_empty_block() {
    btss(
        "{} ",
        " ",
        |b| {
            b.result.is_none() && b.statements.is_empty()
    })
}

#[test]
fn test_discarded_expression() {
    btss(
        "{2+2; } ",
        " ",
        |b| {
            b.result.is_none() && b.statements.len() == 1
    })
}

#[test]
fn test_unterminated() {
    block_test("{", true, None, |b| {true})
}