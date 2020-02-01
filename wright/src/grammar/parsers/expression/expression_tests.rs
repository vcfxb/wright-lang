use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use codespan::{FileId, Files};

fn setup(val: &'static str) -> (Files<String>, FileId) {
    let mut files: Files<String> = Files::new();
    let h = files.add("val", val.to_string());
    (files, h)
}

#[test]
fn bin_op() {
    let (files, h) = setup("2 + 2");
    let frag = Fragment::new(&files, h);
    let expr = Expression::parse(frag).unwrap();
    println!("{:#?}", expr);
}
