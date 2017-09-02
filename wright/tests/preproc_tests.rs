extern crate wright;

use wright::preproc::*;
#[test]
fn single_line_comments() {
    assert_eq!(preproc("hey // hey".to_string()), "hey ".to_string());
}