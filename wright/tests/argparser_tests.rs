extern crate wright;
use wright::argparser::*;
use std::env::Args;

#[test]
fn file_name() {
    let mut test_args: Vec<String> = vec!["wright".to_string(), "file.wright".to_string()];
    assert_eq!(argparse(test_args.clone()), Some("file.wright".to_string()));
    test_args[1] = "file.wr".to_string();
    assert_eq!(argparse(test_args.clone()), Some("file.wr".to_string()));
    test_args[1] = "file".to_string();
    assert_eq!(argparse(test_args.clone()), None);
}