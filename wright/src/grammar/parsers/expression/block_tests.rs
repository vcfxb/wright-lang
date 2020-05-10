use crate::grammar::ast::Block;
use crate::grammar::model::Fragment;
use crate::grammar::testing::setup;

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