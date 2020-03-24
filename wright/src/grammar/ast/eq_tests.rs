use crate::grammar::ast::eq;
use crate::grammar::parsers::testing::setup;
use crate::grammar::model::Fragment;
use codespan::{Files, FileId};

fn setup2(src: &'static str) -> (Files<String>, FileId, FileId) {
    let (mut files, h1) = setup(src);
    let h2 = files.add("other", src.to_owned());
    (files, h1, h2)
}

#[test]
fn test_basic() {
    let (mut files, h1) = setup("5");
    let h2 = files.add("other", "5".to_owned());
    let a = Fragment::new(&files, h1);
    let b = Fragment::new(&files, h2);
    
}