extern crate wright;

use wright::lexer::*;
use wright::lexer::position::*;

#[test]
fn print_err() {
    let mut lexer_err = LexerError::new(Position::new(), "Tested Line.".to_string());
    lexer_err.set_info('t', Some('T'));
    lexer_err.module_name = "TestModule".to_string();
    println!("{}", lexer_err);
}

#[test]
fn print_err_vec() {
    let mut lexer_err = LexerError::new(Position::new(), "Tested Line.".to_string());
    lexer_err.set_info_as_vec(vec!['t', 'T'], Some('T'));
    lexer_err.module_name = "TestModule".to_string();
    println!("{}", lexer_err);
}

// todo: more tests