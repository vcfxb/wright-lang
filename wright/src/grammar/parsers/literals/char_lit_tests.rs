use crate::grammar::ast::CharLit;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::ToExpression;
use codespan::{FileId, Files};

fn setup(s: &'static str) -> (Files<String>, FileId) {
    let mut f = Files::new();
    let h = f.add("char_lit_test file", s.to_string());
    (f, h)
}

#[test]
fn body() {
    let (f, h) = setup("a");
    let fr = Fragment::new(&f, h);
    let res = CharLit::character_body(fr);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val, 'a');
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn wrapper() {
    let (f, h) = setup("'a'");
    let fr = Fragment::new(&f, h);
    let res = CharLit::character_wrapper(fr);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val, 'a');
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

fn test_char(s: &'static str, v: char, rem_sp: usize) {
    let (f, h) = setup(s);
    let frag = Fragment::new(&f, h);
    let res = CharLit::parse(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), rem_sp);
        assert_eq!(val.inner, v);
        if rem_sp == 0 {
            assert_eq!(val.frag.get_span(), frag.get_span());
        }
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}

#[test]
fn basic_char() {
    test_char("'a'", 'a', 0);
}

#[test]
fn unicode_char() {
    test_char("'♦'", '♦', 0);
}

#[test]
fn newline_escape() {
    test_char(r"'\n'", '\n', 0);
}

#[test]
fn byte_escape() {
    test_char(r"'\x69'", '\x69', 0);
}

#[test]
fn unicode_escape() {
    test_char(r"'\u{2666}'", '\u{2666}', 0);
}

fn invalid_test(s: &'static str) -> bool {
    let (f, h) = setup(s);
    let fr = Fragment::new(&f, h);
    CharLit::parse(fr).is_err()
}

#[test]
fn invalid_syntax() {
    let l = ["'ab'", "'a", r"'\a'", r"'\xA'", r"'\u{}'", r"'\u{1234567}'"];
    l.iter().for_each(|i| assert!(invalid_test(i)));
}
