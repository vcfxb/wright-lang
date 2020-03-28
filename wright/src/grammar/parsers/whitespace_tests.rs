use crate::grammar::model::Fragment;
use crate::grammar::parsers::whitespace;
use codespan::{FileId, Files};

fn setup(src: &str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}

#[test]
pub fn single_comment() {
    let (f, h) = setup("// line comment");
    let frag = Fragment::new(&f, h);
    let res = whitespace::line_comment(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val.source(), " line comment");
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn empty_single_comment() {
    let (f, h) = setup("//");
    let frag = Fragment::new(&f, h);
    let res = whitespace::line_comment(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val.source(), "");
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
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 14);
        assert_eq!(val.source(), " line comment");
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn multi_comment_single() {
    let (f, h) = setup("/* single line multi comment */");
    let frag = Fragment::new(&f, h);
    let res = whitespace::multiline_comment(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val.source(), " single line multi comment ");
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn multi_comment_mutli() {
    let (f, h) = setup("/* mutli line\n * multi comment */");
    let frag = Fragment::new(&f, h);
    let res = whitespace::multiline_comment(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val.source(), " mutli line\n * multi comment ");
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn multi_comment_empty() {
    let (f, h) = setup("/**/");
    let frag = Fragment::new(&f, h);
    let res = whitespace::multiline_comment(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
        assert_eq!(val.source(), "");
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
pub fn empty() {
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
fn line_comment_only() {
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
pub fn whitespace_only() {
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

#[test]
fn multiline_comment_only() {
    let (f, h) = setup("/* comment */");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn multiline_comments_and_whitespace() {
    let (f, h) = setup("/* these are many */\n  /* multiline comments */");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn everything() {
    let (f, h) = setup(" // single\n /* these are many */\n  /* multiline comments */ // another");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn multi_in_single() {
    let (f, h) = setup("// comment /* not nested */ comment");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}

#[test]
fn single_in_multi() {
    let (f, h) = setup("/* comment // not nested */");
    let frag = Fragment::new(&f, h);
    let res = whitespace::token_delimiter(frag);
    if let Ok((rem, val)) = res {
        assert_eq!(rem.len(), 0);
    } else {
        eprintln!("{:#?}", res);
        assert!(false);
    }
}
