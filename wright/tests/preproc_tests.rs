extern crate wright;

use wright::preproc::*;
#[test]
fn single_line_comments() {
    assert_eq!(preproc("hey //comment".to_string()), "hey ".to_string());
}

#[test]
fn multi_line_comments() {
    assert_eq!(preproc("h/* comment */ey".to_string()), "hey".to_string());
}