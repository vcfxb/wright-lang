use crate::grammar::ast::Block;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::ToExpression;
use codespan::{FileId, Files};

fn setup(s: &str) -> (Files<String>, FileId) {
    let mut f = Files::new();
    let h = f.add("test", s.to_string());
    (f, h)
}

fn invalid(s: &str) -> bool {
    let (files, handle) = setup(s);
    let fr = Fragment::new(&files, handle);
    Block::parse(fr).is_err()
}

#[test]
fn errors() {
    let inputs = ["{", "}", "{true{}"];
    for i in &inputs {
        assert!(invalid(i));
    }
}

#[test]
fn empty() {
    let (files, handle) = setup("{ }");
    let fr = Fragment::new(&files, handle);
    let res = Block::parse(fr);
    if let Ok((rem, blk)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(blk.statements.len(), 0);
        assert_eq!(blk.result.is_none(), true);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn single_expr() {
    let (files, handle) = setup("{ true }");
    let fr = Fragment::new(&files, handle);
    let res = Block::parse(fr);
    if let Ok((rem, blk)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(blk.statements.len(), 0);
        assert_eq!(blk.result.is_some(), true);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn statements() {
    let (files, handle) = setup("{ true; false; }");
    let fr = Fragment::new(&files, handle);
    let res = Block::parse(fr);
    if let Ok((rem, blk)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(blk.statements.len(), 2);
        assert_eq!(blk.result.is_none(), true);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn statements_and_expr() {
    let (files, handle) = setup("{ true; false; true }");
    let fr = Fragment::new(&files, handle);
    let res = Block::parse(fr);
    if let Ok((rem, blk)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(blk.statements.len(), 2);
        assert_eq!(blk.result.is_some(), true);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn statements_and_expr_nospaces() {
    let (files, handle) = setup("{true;false;true}");
    let fr = Fragment::new(&files, handle);
    let res = Block::parse(fr);
    if let Ok((rem, blk)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(blk.statements.len(), 2);
        assert_eq!(blk.result.is_some(), true);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}
