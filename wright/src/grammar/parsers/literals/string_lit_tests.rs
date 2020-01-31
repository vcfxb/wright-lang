use crate::grammar::ast::StringLit;
use crate::grammar::model::Fragment;
use codespan::{FileId, Files};

fn setup(src: &'static str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}

fn fragment(f: &Files<String>, h: FileId) -> Fragment {
    Fragment::new(f, h)
}

fn do_test(s: &'static str, r: &'static str, o: &'static str) {
    let (f, h) = setup(s);
    let fr = fragment(&f, h);
    let (rem, out) = StringLit::parse(fr).unwrap();
    assert_eq!(out.inner, o);
    assert_eq!(rem.source(), r);
}

#[test]
fn test_simple() {
    do_test(r#""Hello, World""#, r"", r"Hello, World")
}

// todo: more testing here.
