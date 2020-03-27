use crate::grammar::ast::NumLit;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;
use codespan::{FileId, Files};

#[test]
fn from_dec() {
    assert_eq!(NumLit::from_dec("1000").unwrap(), 1000);
}

#[test]
fn dec_primary() {
    let (files, h) = setup("1000");
    let frag = Fragment::new(&files, h);
    let res = NumLit::dec_primary(frag);
    if let Ok((f, v)) = res {
        assert_eq!(f.len(), 0);
        assert_eq!(v, 1000);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn dec_passthrough() {
    let (files, h) = setup("1000");
    let frag = Fragment::new(&files, h);
    let res = NumLit::parse(frag);
    if let Ok((remaining, node)) = res {
        assert_eq!(remaining.len(), 0);
        assert_eq!(node.inner, 1000);
        assert_eq!(node.frag.start(), frag.start());
        assert_eq!(node.frag.end(), frag.end());
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn hex() {
    let (files, h) = setup("0xCafE_babe ");
    let frag = Fragment::new(&files, h);
    let res = NumLit::parse(frag);
    if let Ok((remaining, node)) = res {
        assert_eq!(remaining.len(), 1);
        assert_eq!(node.inner, 0xcafebabe);
        assert_eq!(node.frag.start(), frag.start());
        assert_eq!(node.frag.end(), remaining.start());
        assert_eq!(remaining.end(), frag.end());
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn bin() {
    let (f, h) = setup("0b1010_1001_1001\t");
    let frag = Fragment::new(&f, h);
    let res = NumLit::parse(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.source(), "\t");
        assert_eq!(val.inner, 0b1010_1001_1001);
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}
