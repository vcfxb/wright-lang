extern crate wright;
use wright::lexer::*;
use wright::position::*;
use wright::lexer::error::LexerError;

#[test]
#[ignore]
fn print_err() {
    let mut lexer_err = LexerError::new(Position::new(), "Tested Line.".to_string())
        .set_info('t', Some('T'));
    lexer_err.set_module_name("TestModule".to_string());
    println!("{}", lexer_err);
}

#[test]
#[ignore]
fn print_err_vec() {
    let mut lexer_err = LexerError::new(Position::new(), "Tested Line.".to_string())
        .set_info_as_vec(vec!['t', 'T'], Some('T'));
    lexer_err.set_module_name("TestModule".to_string());
    println!("{}", lexer_err);
}

#[test]
fn catch_double_char_err() {
    let mut test_lexer = Lexer::new("'ha'".to_string());
    let result = test_lexer.lex();
    assert!(result.is_err());
    //println!("{}", result.err().unwrap());
}

#[test]
fn catch_unended_char_err() {
    let mut test_lexer = Lexer::new("'h".to_string());
    let result = test_lexer.lex();
    assert!(result.is_err());
    //println!("{}", result.err().unwrap());
}


#[test]
fn escaped_char() {
    let mut test_lexer = Lexer::new("'\\a'".to_string());
    assert!(test_lexer.lex().is_ok());
}


#[test]
fn catch_long_char_err() {
    let mut test_lexer = Lexer::new("'hello world'".to_string());
    let result = test_lexer.lex();
    assert!(result.is_err());
    //println!("{}", result.err().unwrap());
}