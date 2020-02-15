use crate::grammar::model::Fragment;
use crate::grammar::ast::ExpressionSt;
use codespan::{FileId, Files};

const INPUTS: [&'static str; 2] = [
    "true;",
    "false ;",
];

fn setup(inputs: &[&str]) -> (Files<String>, Vec<FileId>) {
    let mut files: Files<String> = Files::new();
    let mut vec = Vec::new();
    for i in 0..inputs.len() {
        vec.push(files.add(format!("test{}", i), inputs[i].to_string()));
    };
    (files, vec)
}

#[test]
fn expression_statement() {
    let (files, handles) = setup(&INPUTS);
    for handle in handles {
        let frag = Fragment::new(&files, handle);
        let (_, _statement) = ExpressionSt::parse(frag).unwrap();
        // ...
    }
}
