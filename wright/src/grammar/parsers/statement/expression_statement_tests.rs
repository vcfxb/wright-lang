use crate::grammar::ast::ExpressionSt;
use crate::grammar::model::Fragment;
use codespan::{FileId, Files};

fn setup(s: &str) -> (Files<String>, FileId) {
    let mut f = Files::new();
    let h = f.add("test", s.to_string());
    (f, h)
}

fn invalid(s: &str) -> bool {
    let (files, handle) = setup(s);
    let fr = Fragment::new(&files, handle);
    ExpressionSt::parse(fr).is_err()
}

#[test]
fn errors() {
    let inputs = ["true", ";"];
    for i in &inputs {
        assert!(invalid(i));
    }
}

#[test]
fn nospaces() {
    let (files, handle) = setup("true;");
    let fr = Fragment::new(&files, handle);
    let res = ExpressionSt::parse(fr);
    assert!(res.is_ok());
}

#[test]
fn spaces() {
    let (files, handle) = setup("false ;");
    let fr = Fragment::new(&files, handle);
    let res = ExpressionSt::parse(fr);
    assert!(res.is_ok());
}
