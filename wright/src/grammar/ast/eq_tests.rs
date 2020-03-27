use crate::grammar::ast::{eq::AstEq, NumLit};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;
use codespan::{FileId, Files};

fn setup2(src: &'static str) -> (Files<String>, FileId, FileId) {
    let (mut files, h1) = setup(src);
    let h2 = files.add("other", src.to_owned());
    (files, h1, h2)
}

fn get_numlits(files: &Files<String>, h1: FileId, h2: FileId) -> (NumLit, NumLit) {
    let a = NumLit::parse(Fragment::new(files, h1)).unwrap().1;
    let b = NumLit::parse(Fragment::new(files, h2)).unwrap().1;
    (a, b)
}

#[test]
fn test_basic() {
    let (f, h1, h2) = setup2("5");
    let (a, b) = get_numlits(&f, h1, h2);
    assert!(AstEq::ast_eq(&a, &b))
}

#[test]
fn test_box() {
    let (f, h1, h2) = setup2("5");
    let (a, b) = get_numlits(&f, h1, h2);
    assert!(AstEq::ast_eq(&Box::new(a), &Box::new(b)))
}
