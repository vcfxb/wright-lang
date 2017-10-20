extern crate wright;
extern crate regex;
use regex::*;
use wright::parser::*;

// tests

#[test]
fn variable_ids() {
    let id = Regex::new(r"([[:alpha:]][[[:alnum:]]_]*)").unwrap();
    let strings_to_test: Vec<&str> = vec!["A", "a", "Var0", "var0", "d_x", "D_y"];
    for t in strings_to_test {
        assert!(id.is_match(&t.to_string()));
    }
}
