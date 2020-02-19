use crate::grammar::model::Fragment;
use crate::grammar::parsers::whitespace;
use codespan::{FileId, Files};

fn setup(src: &str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}

#[test]
fn single_comment() {
    let (f, h) = setup("// line comment");
    let frag = Fragment::new(&f, h);
    let res = whitespace::line_comment(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn empty_comment() {
    let (f, h) = setup("//");
    let frag = Fragment::new(&f, h);
    let res = whitespace::line_comment(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn comment_with_tail() {
    let (f, h) = setup("// line comment\nnot a comment");
    let frag = Fragment::new(&f, h);
    let res = whitespace::line_comment(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 14);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn comments_and_whitespace() {
    let (f, h) = setup("// line comment\n// this is another comment\n    // third comment\n");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn empty() {
    let (f, h) = setup("");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn comment_only() {
    let (f, h) = setup("// comment");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn whitespace_only() {
    let (f, h) = setup("\t  \n\n   \t  ");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, _)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}
