extern crate wright;
use wright::preproc::*;
#[test]
fn single_line_comments() {
    assert_eq!(preproc("hey //comment".to_string()).last().unwrap().line, "hey ".to_string());
}

#[test]
//#[should_panic]   // so that users don't put comments in the middle of their variable names
fn multi_line_comments() {
    assert_eq!(preproc("h/* comment */ey".to_string()).last().unwrap().line, "h ey".to_string());
}

#[test]
fn comments_with_quotes() {
    assert_eq!(preproc("\"//\"No Comment".to_string()).last().unwrap().line, "\"//\"No Comment".to_string());
}